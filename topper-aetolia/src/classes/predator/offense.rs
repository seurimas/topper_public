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
    let mut controller = get_controller("predator", me, target, timeline, strategy, db);
    controller.init_predator();
    add_hints(db, &mut controller);
    let tree_name = if strategy.eq("class") {
        format!("predator/base")
    } else {
        format!("predator/{}", strategy)
    };
    let tree = get_tree(&tree_name);
    if let Ok(mut tree) = tree.lock() {
        unsafe {
            if DEBUG_TREES {
                if let Some(you) = AetTarget::Target.get_target(&timeline, &controller) {
                    println!("Predator: {:?}", you);
                }
            }
        }
        tree.resume_with(&timeline, &mut controller);
    }
    controller.plan
}

fn add_hints(db: Option<&impl AetDatabaseModule>, controller: &mut BehaviorController) {
    if let Some(db) = db {
        if let Some(mawcrush_freely) = db.get_hint(&MAWCRUSH_FREELY_HINT.to_string()) {
            controller.hint_plan(MAWCRUSH_FREELY_HINT.to_string(), mawcrush_freely);
        }
    }
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

fn get_stance_color(stance: &KnifeStance) -> String {
    match stance {
        KnifeStance::None => "white".to_string(),
        KnifeStance::Gyanis => "red".to_string(),
        KnifeStance::VaeSant => "orange".to_string(),
        KnifeStance::Rizet => "yellow".to_string(),
        KnifeStance::EinFasit => "green".to_string(),
        KnifeStance::Laesan => "blue".to_string(),
        KnifeStance::Bladesurge => "purple".to_string(),
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
    let apex = me
        .check_if_predator(&|predator| predator.apex)
        .map(|apex| {
            let color = if apex >= 3 {
                "green"
            } else if apex >= 2 {
                "yellow"
            } else {
                "white"
            };
            format!("<{}>{}", color, apex)
        })
        .unwrap_or("<white>---".to_string());
    let stance = me
        .check_if_predator(&|predator| predator.stance.clone())
        .or(Some(KnifeStance::None))
        .map(|stance| format!("<{}>{}", get_stance_color(&stance), stance.to_name()))
        .unwrap();
    let companion = me
        .check_if_predator(&|predator| {
            if predator.has_spider() {
                "<magenta>Spider".to_string()
            } else if predator.has_orel() {
                "<yellow>Orel".to_string()
            } else if predator.has_orgyuk() {
                "<red>Orgyuk".to_string()
            } else {
                "<white>NO PET".to_string()
            }
        })
        .unwrap_or("<white>NO PET".to_string());
    let veinrip = if you.predator_board.veinrip.is_active() {
        "<red>Veinrip\n".to_string()
    } else {
        "".to_string()
    };
    let fleshbane = if you.predator_board.fleshbane.is_active() {
        "<red>Fleshbane\n".to_string()
    } else {
        "".to_string()
    };
    let bloodscourge = if you.predator_board.bloodscourge.is_active() {
        "<red>Bloodscourge\n".to_string()
    } else {
        "".to_string()
    };
    format!(
        "{}\n{} {}\n{}{}{}",
        apex, stance, companion, veinrip, fleshbane, bloodscourge
    )
}
