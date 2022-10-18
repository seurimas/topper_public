mod behavior;
mod predicate;
mod sub_trees;
use std::collections::{HashMap, HashSet};

pub use behavior::*;
pub use predicate::*;
use serde::{Deserialize, Serialize};
pub use sub_trees::*;
use topper_bt::unpowered::*;

use crate::{classes::VenomPlan, observables::ActionPlan, timeline::AetTimeline};

pub type AetBehaviorTreeDef = UnpoweredTreeDef<AetBehaviorTreeNode>;

pub static mut DEBUG_TREES: bool = false;

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

#[derive(Default, Debug)]
pub struct BehaviorController {
    pub plan: ActionPlan,
    pub used_balance: bool,
    pub used_equilibrium: bool,
    pub used_secondary_balance: bool,
    pub shifted_left_hand: bool,
    pub shifted_right_hand: bool,
    pub aff_priorities: Option<Vec<VenomPlan>>,
    pub plan_tags: HashSet<String>,
    pub plan_hints: HashMap<String, String>,
    pub target: Option<String>,
    pub allies: HashMap<String, i32>,
}

impl BehaviorController {
    pub fn has_qeb(&self) -> bool {
        !self.used_balance && !self.used_equilibrium
    }

    pub fn tag_plan(&mut self, tag: String) {
        self.plan_tags.insert(tag);
    }

    pub fn hint_plan(&mut self, hint_name: String, hint: String) {
        self.plan_hints.insert(hint_name, hint);
    }

    pub fn get_hint<T: ToString>(&self, hint_name: T) -> Option<&String> {
        self.plan_hints.get(&hint_name.to_string())
    }
}

impl UnpoweredFunction for AetBehaviorTreeNode {
    type Model = BehaviorModel;
    type Controller = BehaviorController;

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState {
        let result = match self {
            Self::Action(action) => action.resume_with(model, controller),
            Self::Predicate(predicate) => predicate.resume_with(model, controller),
            Self::SubTree(sub_tree) => get_tree(sub_tree)
                .lock()
                .unwrap()
                .resume_with(model, controller),
        };
        unsafe {
            if DEBUG_TREES {
                println!("BT: {:?} ({:?})", self, result);
            }
        }
        result
    }

    fn reset(self: &mut Self, model: &Self::Model) {
        match self {
            Self::Action(action) => action.reset(model),
            Self::Predicate(predicate) => predicate.reset(model),
            Self::SubTree(sub_tree) => get_tree(sub_tree).lock().unwrap().reset(model),
        }
    }
}
