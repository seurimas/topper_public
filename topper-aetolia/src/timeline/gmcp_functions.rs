use topper_core::timeline::*;

use crate::{
    db::*,
    non_agent::{AetTimelineRoomExt, Direction, Room},
    types::*,
};

use super::*;

pub fn apply_gmcp<DB: AetDatabaseModule>(
    timeline: &mut AetTimelineState,
    gmcp: &GMCP,
    db: Option<&DB>,
) -> Result<(), String> {
    match gmcp.0.as_ref() {
        "gmcp.Room.Info" => {
            handle_room_info(&gmcp.1, timeline);
        }
        "gmcp.Room.Players" => {
            handle_room_players(&gmcp.1, timeline);
        }
        "gmcp.Char.Vitals" => handle_char_vitals(&gmcp.1, timeline),
        _ => {}
    }
    Ok(())
}

fn handle_char_vitals(
    gmcp: &serde_json::Value,
    timeline: &mut TimelineState<crate::types::AgentState, crate::non_agent::AetNonAgent>,
) {
    if let Some(elevation) = gmcp
        .get("elevation")
        .and_then(|elevation| elevation.as_str())
    {
        let elevation = match elevation {
            "ground" => Elevation::Ground,
            "flying" => Elevation::Flying,
            "trees" => Elevation::Trees,
            "roofs" => Elevation::Roof,
            _ => Elevation::Ground,
        };
        for_agent(
            timeline,
            &timeline.me.clone(),
            &move |me: &mut AgentState| {
                me.elevation = elevation;
            },
        );
    }
    if let (Some(left), Some(right)) = (
        gmcp.get("wield_left").and_then(|left| left.as_str()),
        gmcp.get("wield_right").and_then(|left| left.as_str()),
    ) {
        let left = if left.eq("empty") {
            None
        } else {
            Some(left.to_string())
        };
        let right = if right.eq("empty") {
            None
        } else {
            Some(right.to_string())
        };
        for_agent(
            timeline,
            &timeline.me.clone(),
            &move |me: &mut AgentState| {
                me.unwield_multi(true, true);
                me.wield_multi(left.clone(), right.clone());
            },
        );
    }
    if let Some(dithering) = gmcp
        .get("dithering")
        .and_then(|dithering| dithering.as_str())
        .and_then(|dithering| dithering.parse::<usize>().ok())
    {
        for_agent(
            timeline,
            &timeline.me.clone(),
            &move |me: &mut AgentState| {
                if let ClassState::Bard(class_state) = &mut me.class_state {
                    class_state.dithering = dithering;
                }
            },
        );
    }
    handle_predator_values(gmcp, timeline);
    if let (Some(hp), Some(mp), Some(max_hp), Some(max_mp)) = (
        gmcp.get("hp")
            .and_then(|hp| hp.as_str())
            .and_then(|hp| hp.parse::<i32>().ok()),
        gmcp.get("mp")
            .and_then(|mp| mp.as_str())
            .and_then(|mp| mp.parse::<i32>().ok()),
        gmcp.get("maxhp")
            .and_then(|max_hp| max_hp.as_str())
            .and_then(|max_hp| max_hp.parse::<i32>().ok()),
        gmcp.get("maxmp")
            .and_then(|max_mp| max_mp.as_str())
            .and_then(|max_mp| max_mp.parse::<i32>().ok()),
    ) {
        for_agent(
            timeline,
            &timeline.me.clone(),
            &move |me: &mut AgentState| {
                if !me.is(FType::Recklessness) {
                    me.set_stat(SType::Health, hp);
                    me.set_stat(SType::Mana, mp);
                    me.set_max_stat(SType::Health, max_hp);
                    me.set_max_stat(SType::Mana, max_mp);
                } else if max_mp != mp || max_hp != hp {
                    me.observe_flag(FType::Recklessness, false);
                }
            },
        );
    }
}

fn handle_predator_values(
    gmcp: &serde_json::Value,
    timeline: &mut TimelineState<AgentState, crate::non_agent::AetNonAgent>,
) {
    if let Some(stance) = gmcp
        .get("knife_stance")
        .and_then(|stance| stance.as_str())
        .map(KnifeStance::from_name)
    {
        for_agent(
            timeline,
            &timeline.me.clone(),
            &move |me: &mut AgentState| {
                me.assume_predator(&move |class_state| class_state.stance = stance);
            },
        )
    }
    if let Some(apex) = gmcp
        .get("apex")
        .and_then(|apex| apex.as_str())
        .and_then(|apex| apex.parse::<u32>().ok())
    {
        for_agent(
            timeline,
            &timeline.me.clone(),
            &move |me: &mut AgentState| {
                if let ClassState::Predator(class_state) = &mut me.class_state {
                    class_state.apex = apex;
                }
            },
        );
    }
}

fn handle_monk_values(
    gmcp: &serde_json::Value,
    timeline: &mut TimelineState<AgentState, crate::non_agent::AetNonAgent>,
) {
    if let Some(stance) = gmcp
        .get("stance")
        .and_then(|stance| stance.as_str())
        .map(MonkStance::from_name)
    {
        for_agent(
            timeline,
            &timeline.me.clone(),
            &move |me: &mut AgentState| {
                me.assume_monk(&move |class_state| class_state.stance = stance);
            },
        )
    }
    if let Some(apex) = gmcp
        .get("kai")
        .and_then(|apex| apex.as_str())
        .and_then(|apex| apex.parse::<i32>().ok())
    {
        for_agent(
            timeline,
            &timeline.me.clone(),
            &move |me: &mut AgentState| {
                if let ClassState::Monk(class_state) = &mut me.class_state {
                    class_state.kai = apex;
                }
            },
        );
    }
}

fn handle_room_info(
    gmcp: &serde_json::Value,
    timeline: &mut TimelineState<crate::types::AgentState, crate::non_agent::AetNonAgent>,
) {
    if let Some(room_id) = gmcp.get("num").and_then(|num| num.as_i64()) {
        timeline.for_agent(&timeline.me.clone(), &|me| {
            me.room_id = room_id;
        });
        if let Some(tags) = gmcp.get("details").and_then(|details| details.as_array()) {
            timeline.for_room(room_id, &|room: &mut Room| {
                for tag in tags.iter() {
                    room.add_tag(tag.as_str().unwrap_or_default());
                }
            });
        }
        if let Some(exits) = gmcp.get("exits").and_then(|exits| exits.as_object()) {
            timeline.for_room(room_id, &|room| {
                for (direction, vnum) in exits.iter() {
                    if let (Some(direction), Some(vnum)) =
                        (Direction::from_short(direction), vnum.as_i64())
                    {
                        room.exits.insert(direction, vnum);
                    }
                }
            });
        }
    }
}

fn handle_room_players(
    player_list: &serde_json::Value,
    timeline: &mut TimelineState<crate::types::AgentState, crate::non_agent::AetNonAgent>,
) {
    if let Some(players) = player_list.as_array() {
        if let Some(player) = player_list.get("name").and_then(|player| player.as_str()) {
            let my_room = timeline.borrow_me().room_id;
            timeline.set_player_room(my_room, player);
        }
    }
}
