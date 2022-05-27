use topper_bt::unpowered::*;

use super::*;

use crate::{bt::*, classes::VenomPlan, db::*, defense::*, observables::*, timeline::*, types::*};

pub fn get_stack<'s>(
    timeline: &AetTimeline,
    target: &String,
    strategy: &String,
    db: Option<&impl AetDatabaseModule>,
) -> Option<Vec<VenomPlan>> {
    let mut stack_name = format!("bard_{}", strategy);
    if strategy.eq("class") {
        if let Some(class) = db.and_then(|db| db.get_class(target)) {
            stack_name = format!("bard_{:?}", class.normal());
        } else {
            stack_name = format!("bard_aggro");
        }
    }
    db.and_then(|db| {
        db.get_venom_plan(&stack_name)
            .or_else(|| db.get_venom_plan(&"bard_aggro".to_string()))
    })
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
        aff_priorities: get_stack(timeline, target, strategy, db),
        ..Default::default()
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
