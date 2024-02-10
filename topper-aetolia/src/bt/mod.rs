mod behavior;
mod limb_desc;
mod predicate;
mod sub_trees;
use std::collections::{HashMap, HashSet};

pub use behavior::*;
pub use limb_desc::*;
pub use predicate::*;
use serde::{Deserialize, Serialize};
pub use sub_trees::*;
use topper_bt::unpowered::*;

use crate::{
    classes::{
        get_venoms_from_plan,
        monk::{self, MonkComboGenerator, MonkComboSet},
        predator::{
            ComboAttack, ComboGrader, ComboPredicate, ComboSet, ComboSolver, PredatorCombo,
        },
        VenomPlan,
    },
    observables::ActionPlan,
    timeline::AetTimeline,
    types::{AgentState, Hypnosis, KnifeStance, LType},
};

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
    pub class_controller: ClassController,
}

#[derive(Default, Debug)]
pub enum ClassController {
    #[default]
    Unset,
    Predator {
        predator_combo_store: ComboSolver,
        predator_base_graders: Vec<ComboGrader>,
        predator_combos: ComboSet,
    },
    Infiltrator {
        hypno_stack: Vec<Hypnosis>,
    },
    Monk {
        monk_combo_generator: MonkComboGenerator,
        monk_combos: MonkComboSet,
    },
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

    pub fn get_venoms_from_plan(&self, count: usize, you: &AgentState) -> Vec<&'static str> {
        if let Some(venom_plan) = &self.aff_priorities {
            get_venoms_from_plan(&self.aff_priorities.as_ref().unwrap(), count, &you)
        } else {
            vec![""]
        }
    }

    pub fn init_predator(&mut self) {
        self.class_controller = ClassController::Predator {
            predator_combo_store: ComboSolver::default(),
            predator_base_graders: vec![],
            predator_combos: ComboSet::default(),
        };
    }

    pub fn predator_combo_store(&mut self) -> &mut ComboSolver {
        if let ClassController::Predator {
            predator_combo_store,
            ..
        } = &mut self.class_controller
        {
            predator_combo_store
        } else {
            panic!("Not a predator!")
        }
    }

    pub fn predator_base_graders(&mut self) -> &mut Vec<ComboGrader> {
        if let ClassController::Predator {
            predator_base_graders,
            ..
        } = &mut self.class_controller
        {
            predator_base_graders
        } else {
            panic!("Not a predator!")
        }
    }

    pub fn predator_combos(&mut self) -> &mut ComboSet {
        if let ClassController::Predator {
            predator_combos, ..
        } = &mut self.class_controller
        {
            predator_combos
        } else {
            panic!("Not a predator!")
        }
    }

    pub fn init_monk(&mut self) {
        self.class_controller = ClassController::Monk {
            monk_combo_generator: MonkComboGenerator::default(),
            monk_combos: MonkComboSet::default(),
        };
    }

    pub fn monk_combo_generator(&mut self) -> &mut MonkComboGenerator {
        if let ClassController::Monk {
            monk_combo_generator,
            ..
        } = &mut self.class_controller
        {
            monk_combo_generator
        } else {
            panic!("Not a predator!")
        }
    }

    pub fn monk_combos(&mut self) -> &mut MonkComboSet {
        if let ClassController::Monk { monk_combos, .. } = &mut self.class_controller {
            monk_combos
        } else {
            panic!("Not a predator!")
        }
    }

    pub fn get_highest_scored_predator_combo(
        &self,
        predicates: &Vec<ComboPredicate>,
        graders: &Vec<ComboGrader>,
        start_parrying: Option<LType>,
    ) -> Option<PredatorCombo> {
        if let ClassController::Predator {
            predator_combos,
            predator_base_graders,
            ..
        } = &self.class_controller
        {
            predator_combos.get_highest_scored_combo(
                predicates,
                predator_base_graders,
                graders,
                start_parrying,
            )
        } else {
            panic!("Not a predator!")
        }
    }

    pub fn init_infiltrator(&mut self) {
        self.class_controller = ClassController::Infiltrator {
            hypno_stack: vec![],
        };
    }

    pub fn hypno_stack(&mut self) -> &mut Vec<Hypnosis> {
        if let ClassController::Infiltrator { hypno_stack, .. } = &mut self.class_controller {
            hypno_stack
        } else {
            panic!("Not an infiltrator!")
        }
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
