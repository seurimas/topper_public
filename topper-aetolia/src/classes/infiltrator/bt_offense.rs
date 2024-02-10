use std::collections::HashMap;

use topper_bt::unpowered::*;

use super::*;

use crate::{
    bt::*,
    classes::{get_controller, get_stack, VenomPlan, VenomType},
    curatives::get_cure_depth,
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
    let mut controller = get_controller("infiltrator", me, target, timeline, strategy, db);
    controller.init_infiltrator();
    *controller.hypno_stack() = get_hypno_stack(timeline, target, strategy, db);
    let tree_name = if strategy.eq("class") {
        format!("infiltrator/base")
    } else {
        format!("infiltrator/{}", strategy)
    };
    let tree = get_tree(&tree_name);
    if let Ok(mut tree) = tree.lock() {
        unsafe {
            if DEBUG_TREES {
                if let Some(you) = AetTarget::Target.get_target(&timeline, &controller) {
                    println!("Infiltrator: {:?}", you);
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

pub fn add_delphs(you: &AgentState, venoms: &mut Vec<VenomType>, count: usize) -> bool {
    if you.is(FType::Allergies) || you.is(FType::Vomiting) {
        return false;
    }
    let mut delphing = false;
    if you.is(FType::Hypersomnia) {
        match (
            you.is(FType::Insomnia),
            you.is(FType::Asleep),
            you.is(FType::Instawake),
        ) {
            (true, false, true) => {
                if get_cure_depth(you, FType::Hypersomnia).cures > 1 {
                    venoms.insert(0, "delphinium");
                    delphing = true;
                }
            }
            (false, false, true) => {
                if count == 2 {
                    venoms.insert(0, "delphinium");
                    venoms.insert(0, "delphinium");
                    delphing = true;
                }
            }
            (true, _, _) | (_, false, _) | (_, _, true) => {
                venoms.insert(0, "delphinium");
                delphing = true;
            }
            _ => {}
        }
        if !delphing {
            return false;
        }
        if venoms.len() >= count && Some(&"darkshade") == venoms.get(venoms.len() - count) {
            venoms.remove(venoms.len() - count);
        }
        if venoms.len() >= count && Some(&"euphorbia") == venoms.get(venoms.len() - count) {
            venoms.remove(venoms.len() - count);
        }
    }
    delphing
}

pub fn get_top_suggestion(target: &AgentState, hypnos: &Vec<Hypnosis>) -> Option<Hypnosis> {
    let mut hypno_idx = 0;
    for i in 0..target.hypno_state.suggestion_count() {
        if target.hypno_state.get_suggestion(i) == hypnos.get(hypno_idx) {
            hypno_idx += 1;
        }
    }
    if hypno_idx < hypnos.len() {
        hypnos.get(hypno_idx).map(|hypno| hypno.clone())
    } else {
        None
    }
}

pub fn get_hypno_stack_name(timeline: &AetTimeline, target: &String, strategy: &String) -> String {
    timeline
        .state
        .get_my_hint(&"HYPNO_STACK".to_string())
        .unwrap_or(strategy.to_string())
}

lazy_static! {
    static ref HARD_HYPNO: Vec<Hypnosis> = vec![
        Hypnosis::Aff(FType::Hypochondria),
        Hypnosis::Aff(FType::Impatience),
        Hypnosis::Aff(FType::Loneliness),
        Hypnosis::Aff(FType::Hypochondria),
        Hypnosis::Aff(FType::Impatience),
        Hypnosis::Aff(FType::Vertigo),
        Hypnosis::Aff(FType::Impatience),
        Hypnosis::Aff(FType::Loneliness),
    ];
}

pub fn get_hypno_stack<'s>(
    timeline: &AetTimeline,
    target: &String,
    strategy: &String,
    db: Option<&impl AetDatabaseModule>,
) -> Vec<Hypnosis> {
    db.and_then(|db| {
        let stack = get_hypno_stack_name(timeline, target, strategy);
        if stack == "normal" {
            None // Default to HARD_HYPNO
        } else if stack == "class" {
            if let Some(class) = db.get_class(target) {
                db.get_hypno_plan(&class.to_string())
            } else {
                db.get_hypno_plan(&format!("hypno_{}", stack))
            }
        } else {
            db.get_hypno_plan(&format!("hypno_{}", stack))
        }
    })
    .unwrap_or(HARD_HYPNO.to_vec())
}
