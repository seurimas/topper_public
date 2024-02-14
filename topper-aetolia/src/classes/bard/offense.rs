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
    let mut controller = get_controller("bard", me, target, timeline, strategy, db);
    add_hints(db, &mut controller);
    let tree_name = if strategy.eq("class") {
        format!("bard/base")
    } else {
        format!("bard/{}", strategy)
    };
    let tree = get_tree(&tree_name);
    if let Ok(mut tree) = tree.lock() {
        tree.resume_with(&timeline, &mut controller);
    }
    controller.plan
}

fn add_hints(db: Option<&impl AetDatabaseModule>, controller: &mut BehaviorController) {
    if let Some(db) = db {
        if let Some(impetus_weapon) = db.get_hint(&IMPETUS_WEAPON_HINT.to_string()) {
            controller.hint_plan(IMPETUS_WEAPON_HINT.to_string(), impetus_weapon);
        }

        if let Some(fast_weapon) = db.get_hint(&FAST_WEAPON_HINT.to_string()) {
            controller.hint_plan(FAST_WEAPON_HINT.to_string(), fast_weapon);
        }

        if let Some(instrument) = db.get_hint(&INSTRUMENT_HINT.to_string()) {
            controller.hint_plan(INSTRUMENT_HINT.to_string(), instrument);
        }

        if let Some(use_instrument) = db.get_hint(&USE_INSTRUMENT_HINT.to_string()) {
            controller.hint_plan(USE_INSTRUMENT_HINT.to_string(), use_instrument);
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

pub fn get_class_state(
    timeline: &AetTimeline,
    target: &String,
    strategy: &String,
    db: Option<&impl AetDatabaseModule>,
) -> String {
    let me = timeline.state.borrow_me();
    let you = timeline.state.borrow_agent(target);
    let runeband = if let Some((next_runeband, timer)) = you.bard_board.next_runeband() {
        format!(
            "<magenta>{}({}) ",
            next_runeband,
            timer.get_time_left_seconds()
        )
    } else {
        "<gray>No RB ".to_string()
    };
    let globes = if let Some(next_globe) = you.bard_board.next_globe() {
        format!("<blue>{} ", next_globe)
    } else {
        "<gray>G0 ".to_string()
    };
    let dumbness = if !you.bard_board.is_dumb(true) {
        format!("<red>SMART ")
    } else {
        format!("")
    };
    let (anelace, halfbeat, dithering, singing, playing) = me
        .check_if_bard(&|me| {
            let anelace = if me.anelaces > 0 {
                format!("<green>A{} ", me.anelaces)
            } else {
                format!("<red>A0 ")
            };
            let halfbeat = if me.half_beat.active() {
                format!("<green>½ ")
            } else {
                format!("<yellow>X ")
            };
            let dithering = if me.dithering > 0 {
                format!("<red>D{} ", me.dithering)
            } else {
                format!("<green>D0")
            };
            let singing = if let Some(song) = me.voice_song {
                format!(
                    "<magenta>{}({}) ",
                    song,
                    me.voice_timeout as f32 / BALANCE_SCALE
                )
            } else {
                format!("")
            };
            let playing = if let Some(song) = me.instrument_song {
                format!(
                    "<magenta>{}({}) ",
                    song,
                    me.instrument_timeout as f32 / BALANCE_SCALE
                )
            } else {
                format!("")
            };
            (anelace, halfbeat, dithering, singing, playing)
        })
        .unwrap_or_default();
    let ironcollar = if you.bard_board.iron_collar_state.is_active() {
        "<white>COLLAR "
    } else {
        ""
    };
    let self_loathing = if you.is(FType::SelfLoathing) {
        format!("<green>SL ({}) ", you.get_balance(BType::SelfLoathing))
    } else {
        "".to_string()
    };
    let needle = if you.bard_board.needle_venom.is_some() {
        format!("<red>needle ({}) ", you.bard_board.needle_timer)
    } else {
        "".to_string()
    };
    let primary = if let Some(emotion) = you.bard_board.emotion_state.primary {
        format!(
            "<magenta>{} ({})",
            emotion.name(),
            you.bard_board.emotion_state.get_emotion_level(emotion)
        )
    } else {
        "".to_string()
    };
    let pipelocks = if you.is(FType::Perplexed) {
        let empties = you.pipe_state.get_empties();
        if empties.len() > 0 {
            format!("<green>{}", empties.join("/"))
        } else {
            format!("")
        }
    } else {
        format!("")
    };
    let missing_hints = if let Some(db) = db {
        let impetus = if let Some(impetus_weapon) = db.get_hint(&IMPETUS_WEAPON_HINT.to_string()) {
            ""
        } else {
            "<red>Missing IMPETUS_WEAPON hint, use `thint IMPETUS_WEAPON <falchion###>`"
        };
        let fast = if let Some(fast_weapon) = db.get_hint(&FAST_WEAPON_HINT.to_string()) {
            ""
        } else {
            "<red>Missing IMPETUS_WEAPON hint, use `thint IMPETUS_WEAPON <falchion###>`"
        };
        format!("{} {}", impetus, fast)
    } else {
        "<red>DB??".to_string()
    };
    format!("{dumbness}{globes}{runeband}{anelace}{dithering}\n{needle}{halfbeat}{singing}{playing}\n{ironcollar}{self_loathing}{primary}{pipelocks}\n{missing_hints}")
}
