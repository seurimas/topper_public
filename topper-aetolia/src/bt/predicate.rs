use serde::Deserialize;
use serde::Serialize;
use topper_bt::unpowered::*;

use crate::classes::bard::BardPredicate;
use crate::timeline::*;
use crate::types::*;

use super::BehaviorController;
use super::BehaviorModel;

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
    Locked(AetTarget, bool),
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
