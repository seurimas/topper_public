use topper_bt::unpowered::*;

use super::*;

use crate::{bt::*, db::*, defense::*, observables::*, timeline::*, types::*};

lazy_static! {
    pub static ref DEFAULT_BEHAVIOR_TREE: AetBehaviorTreeDef = {
        let mut tree_def = UnpoweredTreeDef::Sequence(vec![UnpoweredTreeDef::User(
            AetBehaviorTreeNode::Action(AetBehavior::BardBehavior(
                BardBehavior::PerformanceAttack(PerformanceAttack::Crackshot),
            )),
        )]);
        tree_def
    };
}

pub fn get_action_plan(
    timeline: &AetTimeline,
    me: &String,
    target: &String,
    strategy: &String,
    db: Option<&impl AetDatabaseModule>,
) -> ActionPlan {
    let mut controller = BehaviorController {
        plan: ActionPlan::new(me),
        target: Some(target.clone()),
    };
    let mut tree = DEFAULT_BEHAVIOR_TREE.create_tree();
    tree.resume_with(&timeline, &mut controller);
    controller.plan
}

pub fn get_attack(
    timeline: &AetTimeline,
    target: &String,
    strategy: &String,
    db: Option<&impl AetDatabaseModule>,
) -> String {
    let action_plan = get_action_plan(&timeline, &timeline.who_am_i(), &target, &strategy, db);
    action_plan.get_inputs(&timeline)
}
