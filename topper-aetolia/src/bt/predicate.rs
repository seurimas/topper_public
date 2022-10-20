use serde::Deserialize;
use serde::Serialize;
use topper_bt::unpowered::*;

use crate::classes::bard::BardPredicate;
use crate::classes::get_affs_from_plan;
use crate::classes::VenomPlan;
use crate::curatives::get_cure_depth;
use crate::timeline::*;
use crate::types::*;

use super::BehaviorController;
use super::BehaviorModel;

pub const QUEUE_TIME: f32 = 0.25;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum AetTarget {
    Me,
    Target,
}

impl AetTarget {
    pub fn get_target<'a>(
        &self,
        model: &'a BehaviorModel,
        controller: &BehaviorController,
    ) -> Option<&'a AgentState> {
        match self {
            AetTarget::Me => model
                .state
                .get_agent(&model.who_am_i())
                .and_then(|branches| branches.get(0)),
            AetTarget::Target => controller
                .target
                .as_ref()
                .and_then(|target| model.state.get_agent(&target))
                .and_then(|branches| branches.get(0)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum AetPredicate {
    AllAffs(AetTarget, Vec<FType>),
    SomeAffs(AetTarget, Vec<FType>),
    NoAffs(AetTarget, Vec<FType>),
    AffCountOver(AetTarget, usize, Vec<FType>),
    AffCountUnder(AetTarget, usize, Vec<FType>),
    PriorityAffIs(AetTarget, FType),
    CannotCure(AetTarget, FType),
    Buffered(AetTarget, FType),
    Locked(AetTarget, bool),
    ReboundingWindow(AetTarget, CType),
    HasBalanceEquilibrium(AetTarget),
    HasBalance(AetTarget),
    HasEquilibrium(AetTarget),
    BardPredicate(AetTarget, BardPredicate),
}

pub trait TargetPredicate {
    fn check(
        &self,
        target: &AetTarget,
        world: &BehaviorModel,
        controller: &BehaviorController,
    ) -> bool;
}

fn all_affs(
    target: &AetTarget,
    world: &BehaviorModel,
    controller: &BehaviorController,
    affs: &Vec<FType>,
) -> bool {
    if let Some(target) = target.get_target(world, controller) {
        for aff in affs {
            if !target.is(*aff) {
                return false;
            }
        }
        return true;
    }
    return false;
}

fn some_affs(
    target: &AetTarget,
    world: &BehaviorModel,
    controller: &BehaviorController,
    affs: &Vec<FType>,
) -> bool {
    if let Some(target) = target.get_target(world, controller) {
        for aff in affs {
            if target.is(*aff) {
                return true;
            }
        }
        return false;
    }
    return false;
}

fn no_affs(
    target: &AetTarget,
    world: &BehaviorModel,
    controller: &BehaviorController,
    affs: &Vec<FType>,
) -> bool {
    if let Some(target) = target.get_target(world, controller) {
        for aff in affs {
            if target.is(*aff) {
                return false;
            }
        }
        return true;
    }
    return false;
}

fn aff_counts(
    target: &AetTarget,
    model: &BehaviorModel,
    controller: &mut BehaviorController,
    affs: &Vec<FType>,
) -> Option<usize> {
    target.get_target(model, controller).map(|target| {
        if affs.len() > 0 {
            target.affs_count(affs)
        } else {
            target.aff_count()
        }
    })
}

pub fn get_priority_aff(
    target: &AetTarget,
    model: &BehaviorModel,
    controller: &BehaviorController,
    stack: Option<Vec<VenomPlan>>,
) -> Option<FType> {
    if let (Some(target), Some(stack)) = (target.get_target(model, controller), stack) {
        get_affs_from_plan(&stack, 1, target).get(0).cloned()
    } else {
        None
    }
}

impl UnpoweredFunction for AetPredicate {
    type Model = BehaviorModel;
    type Controller = BehaviorController;

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState {
        match self {
            AetPredicate::AllAffs(target, affs) => {
                if all_affs(target, model, controller, affs) {
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetPredicate::SomeAffs(target, affs) => {
                if some_affs(target, model, controller, affs) {
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetPredicate::NoAffs(target, affs) => {
                if no_affs(target, model, controller, affs) {
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetPredicate::AffCountOver(target, min_count, affs) => {
                if let Some(aff_count) = aff_counts(target, model, controller, affs) {
                    if aff_count >= *min_count {
                        UnpoweredFunctionState::Complete
                    } else {
                        UnpoweredFunctionState::Failed
                    }
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetPredicate::AffCountUnder(target, min_count, affs) => {
                if let Some(aff_count) = aff_counts(target, model, controller, affs) {
                    if aff_count <= *min_count {
                        UnpoweredFunctionState::Complete
                    } else {
                        UnpoweredFunctionState::Failed
                    }
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetPredicate::Locked(target, hard_only) => {
                if let Some(target) = target.get_target(model, controller) {
                    if let Some(lock) = target.lock_duration() {
                        if !*hard_only || lock >= 10.0 {
                            return UnpoweredFunctionState::Complete;
                        }
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::CannotCure(target, aff) => {
                if let Some(target) = target.get_target(model, controller) {
                    let mut afflicted = target.clone();
                    afflicted.set_flag(*aff, true);
                    let cure_depth = get_cure_depth(&afflicted, *aff);
                    if cure_depth.time > 100 {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::PriorityAffIs(target, aff) => {
                if let Some(priority_aff) =
                    get_priority_aff(target, model, controller, controller.aff_priorities.clone())
                {
                    if priority_aff == *aff {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::Buffered(target, aff) => {
                if let Some(target) = target.get_target(model, controller) {
                    if get_cure_depth(target, *aff).cures > 1 {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::ReboundingWindow(target, minimum) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.get_balance(BType::Rebounding) > (*minimum as f32 / BALANCE_SCALE) {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::HasBalance(target) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.get_balance(BType::Balance) < QUEUE_TIME {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::HasEquilibrium(target) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.get_balance(BType::Equil) < QUEUE_TIME {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::HasBalanceEquilibrium(target) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.get_balance(BType::Balance) < QUEUE_TIME
                        && target.get_balance(BType::Equil) < QUEUE_TIME
                    {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::BardPredicate(target, bard_predicate) => {
                if bard_predicate.check(target, model, controller) {
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
        }
    }

    fn reset(self: &mut Self, model: &Self::Model) {
        // Nothing to do
    }
}
