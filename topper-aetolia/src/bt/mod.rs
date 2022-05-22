mod behavior;
mod predicate;
mod sub_trees;
pub use behavior::*;
pub use predicate::*;
use serde::{Deserialize, Serialize};
pub use sub_trees::*;
use topper_bt::unpowered::*;

use crate::{observables::ActionPlan, timeline::AetTimeline};

pub type AetBehaviorTreeDef = UnpoweredTreeDef<AetBehaviorTreeNode>;

lazy_static! {
    pub static ref DEFAULT_BEHAVIOR_TREE: AetBehaviorTreeDef =
        serde_json::from_str::<AetBehaviorTreeDef>(include_str!("./DEFAULT_BEHAVIOR_TREE.ron"))
            .unwrap();
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum AetBehaviorTreeNode {
    Action(AetBehavior),
    Predicate(AetPredicate),
    SubTree(String),
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
            Self::SubTree(sub_tree) => get_tree(sub_tree)
                .lock()
                .unwrap()
                .resume_with(model, controller),
        }
    }

    fn reset(self: &mut Self, model: &Self::Model) {
        match self {
            Self::Action(action) => action.reset(model),
            Self::Predicate(predicate) => predicate.reset(model),
            Self::SubTree(sub_tree) => get_tree(sub_tree).lock().unwrap().reset(model),
        }
    }
}
