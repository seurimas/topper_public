use serde::Deserialize;
use serde::Serialize;
use topper_bt::unpowered::*;

use crate::classes::bard::BardPredicate;
use crate::classes::get_affs_from_plan;
use crate::classes::infiltrator::InfiltratorPredicate;
use crate::classes::is_affected_by;
use crate::classes::predator::PredatorPredicate;
use crate::classes::Class;
use crate::classes::LockType;
use crate::classes::VenomPlan;
use crate::curatives::get_cure_depth;
use crate::non_agent::AetTimelineRoomExt;
use crate::timeline::*;
use crate::types::*;

use super::BehaviorController;
use super::BehaviorModel;
use super::LimbDescriptor;

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
                .and_then(|branches| branches.get(0))
                .or(Some(&model.default_agent)),
        }
    }

    pub fn get_name<'a>(
        &self,
        model: &'a BehaviorModel,
        controller: &BehaviorController,
    ) -> String {
        match self {
            AetTarget::Me => model.who_am_i(),
            AetTarget::Target => controller
                .target
                .clone()
                .unwrap_or_else(|| "enemy".to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum AetPredicate {
    // Affs
    AllAffs(AetTarget, Vec<FType>),
    SomeAffs(AetTarget, Vec<FType>),
    NoAffs(AetTarget, Vec<FType>),
    AffCountOver(AetTarget, usize, Vec<FType>),
    AffCountUnder(AetTarget, usize, Vec<FType>),
    // Limbs
    IsRestoring(AetTarget, LimbDescriptor),
    CanBreak(AetTarget, LimbDescriptor, f32),
    CanMangled(AetTarget, LimbDescriptor, f32),
    // Priorities
    PriorityAffIs(AetTarget, FType),
    // Buffer/locks
    CannotCure(AetTarget, FType),
    Buffered(AetTarget, FType),
    Locked(AetTarget, bool),
    NearLocked(AetTarget, LockType, usize),
    // Timing
    ReboundingWindow(AetTarget, CType),
    SalveBlocked(AetTarget, CType),
    // Hints
    LimbHintIs(String, LType),
    HintSet(String, String),
    // Stats
    HealthUnder(AetTarget, f32),
    // Balances
    HasBalanceEquilibrium(AetTarget),
    HasBalance(AetTarget),
    HasEquilibrium(AetTarget),
    HasTree(AetTarget, f32),
    HasFocus(AetTarget, f32),
    HasFitness(AetTarget, f32),
    HasClassCure(AetTarget, f32),
    // Elevation
    IsGrounded(AetTarget),
    IsFlying(AetTarget),
    IsClimbing(AetTarget),
    // Room tags
    RoomIsTagged(String),
    // Parries
    KnownParry(AetTarget, LimbDescriptor),
    CanParry(AetTarget),
    // Class-specific
    IsAffectedBy(AetTarget, FType),
    ClassIn(AetTarget, Vec<Class>),
    BardPredicate(AetTarget, BardPredicate),
    PredatorPredicate(AetTarget, PredatorPredicate),
    InfiltratorPredicate(AetTarget, InfiltratorPredicate),
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
    return true;
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
            AetPredicate::IsRestoring(target, limb_descriptor) => {
                if let Some(limb) = limb_descriptor.get_limb(model, controller, target) {
                    if let Some(target) = target.get_target(model, controller) {
                        if target.get_limb_state(limb).is_restoring {
                            return UnpoweredFunctionState::Complete;
                        }
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::CanBreak(target, limb_descriptor, damage) => {
                if let Some(limb) = limb_descriptor.get_limb(model, controller, target) {
                    if let Some(target) = target.get_target(model, controller) {
                        if target.get_limb_state(limb).hits_to_break(*damage) == 1 {
                            return UnpoweredFunctionState::Complete;
                        }
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::CanMangled(target, limb_descriptor, damage) => {
                if let Some(limb) = limb_descriptor.get_limb(model, controller, target) {
                    if let Some(target) = target.get_target(model, controller) {
                        if target.get_limb_state(limb).hits_to_mangle(*damage) == 1 {
                            return UnpoweredFunctionState::Complete;
                        }
                    }
                }
                UnpoweredFunctionState::Failed
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
            AetPredicate::NearLocked(target, lock_type, aff_count) => {
                if let Some(target) = target.get_target(model, controller) {
                    let affs_to_lock = lock_type.affs_to_lock(target);
                    if affs_to_lock <= *aff_count {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::CannotCure(target, aff) => {
                if let Some(target) = target.get_target(model, controller) {
                    let mut afflicted = target.clone();
                    afflicted.set_flag(*aff, true);
                    let cure_depth = get_cure_depth(&afflicted, *aff);
                    let minimum_depth =
                        if let Some(me) = AetTarget::Me.get_target(model, controller) {
                            110 + (BALANCE_SCALE * me.get_qeb_balance()) as CType
                        } else {
                            110
                        };
                    if cure_depth.time > minimum_depth {
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
            AetPredicate::SalveBlocked(target, minimum) => {
                if let Some(target) = target.get_target(model, controller) {
                    if let Some(restore) = target.limb_damage.restore_timer {
                        if restore.get_time_left() > *minimum {
                            return UnpoweredFunctionState::Complete;
                        }
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
            AetPredicate::HasFocus(target, buffer) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.get_balance(BType::Focus) < QUEUE_TIME + *buffer
                        && target.can_focus(true)
                    {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::KnownParry(target, limb_descriptor) => {
                if let Some(limb) = limb_descriptor.get_limb(model, controller, target) {
                    if let Some(target) = target.get_target(model, controller) {
                        if target.get_parrying() == Some(limb) {
                            return UnpoweredFunctionState::Complete;
                        }
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::CanParry(target) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.can_parry() {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::HasTree(target, buffer) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.get_balance(BType::Tree) < QUEUE_TIME + *buffer
                        && target.can_tree(true)
                    {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::HasFitness(target, buffer) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.get_balance(BType::Fitness) < QUEUE_TIME + *buffer {
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::HasClassCure(target, buffer) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.get_balance(BType::ClassCure1) < QUEUE_TIME + *buffer {
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
            AetPredicate::PredatorPredicate(target, predator_predicate) => {
                if predator_predicate.check(target, model, controller) {
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetPredicate::InfiltratorPredicate(target, infiltrator_predicate) => {
                if infiltrator_predicate.check(target, model, controller) {
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetPredicate::LimbHintIs(hint, limb) => {
                if let Some(hint) = controller.get_hint(hint) {
                    if hint.eq_ignore_ascii_case(&limb.to_string()) {
                        UnpoweredFunctionState::Complete
                    } else {
                        UnpoweredFunctionState::Failed
                    }
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetPredicate::HintSet(hint, value) => {
                if let Some(hint) = controller.get_hint(hint) {
                    if hint.eq_ignore_ascii_case(value) {
                        UnpoweredFunctionState::Complete
                    } else {
                        UnpoweredFunctionState::Failed
                    }
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetPredicate::IsGrounded(target) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.elevation == Elevation::Ground {
                        UnpoweredFunctionState::Complete
                    } else {
                        UnpoweredFunctionState::Failed
                    }
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetPredicate::IsFlying(target) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.elevation == Elevation::Flying {
                        UnpoweredFunctionState::Complete
                    } else {
                        UnpoweredFunctionState::Failed
                    }
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetPredicate::IsClimbing(target) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.elevation == Elevation::Trees || target.elevation == Elevation::Roof {
                        UnpoweredFunctionState::Complete
                    } else {
                        UnpoweredFunctionState::Failed
                    }
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetPredicate::RoomIsTagged(tag) => {
                if let Some(room) = model.state.get_my_room() {
                    if room.has_tag(tag) {
                        UnpoweredFunctionState::Complete
                    } else {
                        UnpoweredFunctionState::Failed
                    }
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetPredicate::HealthUnder(target, percent) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.get_health_percent() < *percent {
                        UnpoweredFunctionState::Complete
                    } else {
                        UnpoweredFunctionState::Failed
                    }
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetPredicate::IsAffectedBy(target, aff) => {
                if let Some(target) = target.get_target(model, controller) {
                    if let Some(class) = target.class_state.get_normalized_class() {
                        if is_affected_by(class, *aff) {
                            return UnpoweredFunctionState::Complete;
                        }
                    }
                }
                UnpoweredFunctionState::Failed
            }
            AetPredicate::ClassIn(target, classes) => {
                if let Some(target) = target.get_target(model, controller) {
                    if let Some(class) = target.class_state.get_normalized_class() {
                        if classes.contains(&class) {
                            return UnpoweredFunctionState::Complete;
                        }
                    }
                }
                UnpoweredFunctionState::Failed
            }
        }
    }

    fn reset(self: &mut Self, model: &Self::Model) {
        // Nothing to do
    }
}
