mod behavior;
mod predicate;
pub use behavior::*;
pub use predicate::*;
use serde::{Deserialize, Serialize};
use topper_bt::unpowered::*;

use crate::{observables::ActionPlan, timeline::AetTimeline};

pub type AetBehaviorTreeDef = UnpoweredTreeDef<AetBehaviorTreeNode>;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum AetBehaviorTreeNode {
    Action(AetBehavior),
    Predicate(AetPredicate),
}

pub type BehaviorModel = AetTimeline;

pub struct BehaviorController {
    pub plan: ActionPlan,
    pub target: Option<String>,
}

impl UnpoweredFunction for AetBehaviorTreeNode {
    type Model = BehaviorModel;
    type Controller = BehaviorController;

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState {
        match self {
            Self::Action(action) => action.resume_with(model, controller),
            Self::Predicate(predicate) => predicate.resume_with(model, controller),
        }
    }

    fn reset(self: &mut Self, model: &Self::Model) {
        match self {
            Self::Action(action) => action.reset(model),
            Self::Predicate(predicate) => predicate.reset(model),
        }
    }
}
