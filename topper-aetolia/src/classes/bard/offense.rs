use topper_bt::unpowered::*;

use super::*;

use crate::{bt::*, db::*, defense::*, observables::*, timeline::*, types::*};

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
    let tree_name = format!("bard/{}", strategy);
    let tree = get_tree(&tree_name);
    if let Ok(mut tree) = tree.lock() {
        tree.resume_with(&timeline, &mut controller);
    }
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
