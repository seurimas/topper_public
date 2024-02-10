use std::collections::HashMap;

use topper_bt::unpowered::*;

use super::*;

use crate::{
    bt::*,
    classes::{get_controller, get_stack, VenomPlan},
    db::*,
    defense::*,
    non_agent::AetNonAgent,
    observables::*,
    timeline::*,
    types::*,
};

pub fn get_action_plan(
    timeline: &AetTimeline,
    me: &String,
    target: &String,
    strategy: &String,
    db: Option<&impl AetDatabaseModule>,
) -> ActionPlan {
    let mut controller = get_controller("monk", me, target, timeline, strategy, db);
    controller.init_monk();
    let tree_name = if strategy.eq("class") {
        format!("monk/base")
    } else {
        format!("monk/{}", strategy)
    };
    let tree = get_tree(&tree_name);
    if let Ok(mut tree) = tree.lock() {
        unsafe {
            if DEBUG_TREES {
                if let Some(you) = AetTarget::Target.get_target(&timeline, &controller) {
                    println!("Monk: {:?}", you);
                }
            }
        }
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

pub fn get_stance_color(stance: &MonkStance) -> &'static str {
    match stance {
        MonkStance::None => "white",
        MonkStance::Bear => "red",
        MonkStance::Cat => "cyan",
        MonkStance::Cobra => "green",
        MonkStance::Dragon => "yellow",
        MonkStance::Eagle => "magenta",
        MonkStance::Horse => "green",
        MonkStance::Phoenix => "red",
        MonkStance::Rat => "yellow",
        MonkStance::Scorpion => "magenta",
        MonkStance::Tiger => "cyan",
        MonkStance::Wolf => "green",
    }
}

pub fn get_class_state(
    timeline: &AetTimeline,
    target: &String,
    strategy: &String,
    db: Option<&impl AetDatabaseModule>,
) -> String {
    let me = timeline.state.borrow_me();
    let you = timeline.state.borrow_agent(target);
    let kai = me
        .check_if_monk(&|monk| monk.kai)
        .map(|kai| {
            let color = if kai >= 80 {
                "green"
            } else if kai >= 50 {
                "yellow"
            } else if kai >= 20 {
                "red"
            } else {
                "white"
            };
            format!("<{}>{}", color, kai)
        })
        .unwrap_or("<white>---".to_string());
    let stance = me
        .check_if_monk(&|monk| monk.stance.clone())
        .or(Some(MonkStance::None))
        .map(|stance| format!("<{}>{}", get_stance_color(&stance), stance.to_name()))
        .unwrap();
    format!("{}\t{}", kai, stance)
}
