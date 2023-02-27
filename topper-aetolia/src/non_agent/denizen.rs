use std::collections::HashSet;

use serde::Deserialize;

use crate::timeline::AetTimelineState;

use super::{AetNonAgent, AetTimelineRoomExt};

#[derive(Debug, Deserialize, PartialEq, Clone, Copy)]
pub enum EvalStatus {
    Uninjured,
    SlightlyBruised,
    HeavilyBruised,
    SeveralOpenWounds,
    CoveredInBlood,
    BleedingHeavily,
    AlmostDead,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Denizen {
    pub id: String,
    pub room_id: i64,
    pub full_name: String,
    pub status: EvalStatus,
    pub aggroed: bool,
    pub tags: HashSet<String>,
}

pub fn format_denizen_id(room_id: i64) -> String {
    format!("npc_{}", room_id)
}

pub trait AetTimelineDenizenExt {
    fn add_denizen(
        &mut self,
        denizen_id: i64,
        id: String,
        room_id: i64,
        full_name: String,
        status: Option<EvalStatus>,
    );

    fn for_denizen(&mut self, denizen_id: i64, action: &Fn(&mut Denizen));

    fn kill_denizen(&mut self, denizen_id: i64);

    fn observe_denizen_in_room(&mut self, denizen_id: i64, room_id: i64);

    fn check_denizen<R>(&self, denizen_id: i64, predicate: &Fn(&Denizen) -> R) -> Option<R>;

    fn denizen_has_tag(&self, denizen_id: i64, tag: String) -> bool {
        self.check_denizen(denizen_id, &|denizen: &Denizen| denizen.tags.contains(&tag))
            .unwrap_or(false)
    }
}

impl AetTimelineDenizenExt for AetTimelineState {
    fn add_denizen(
        &mut self,
        denizen_id: i64,
        id: String,
        room_id: i64,
        full_name: String,
        status: Option<EvalStatus>,
    ) {
        let key = format_denizen_id(denizen_id);
        if let Some(existing) = self
            .non_agent_states
            .get(&key)
            .and_then(AetNonAgent::as_denizen)
        {
            self.for_room(existing.room_id, &|mut room| {
                room.denizens.remove(&denizen_id);
            });
            self.for_room(room_id, &|mut room| {
                room.denizens.insert(denizen_id);
            });
        } else {
            self.non_agent_states.insert(
                key,
                AetNonAgent::Denizen(Denizen {
                    id,
                    full_name,
                    room_id,
                    status: status.unwrap_or(EvalStatus::Uninjured),
                    aggroed: false,
                    tags: HashSet::new(),
                }),
            );
            self.for_room(room_id, &|mut room| {
                room.denizens.insert(denizen_id);
            });
        }
    }

    fn kill_denizen(&mut self, denizen_id: i64) {
        if let Some(room_id) = self.check_denizen(denizen_id, &|denizen| denizen.room_id) {
            self.for_room(room_id, &|mut room| {
                room.denizens.remove(&denizen_id);
            });
        }
        self.non_agent_states.remove(&format_denizen_id(denizen_id));
    }

    fn observe_denizen_in_room(&mut self, denizen_id: i64, room_id: i64) {
        let previous_room_id = self.check_denizen(denizen_id, &|denizen| denizen.room_id);
        if let Some(previous_room_id) = previous_room_id {
            self.for_room(previous_room_id, &|mut room| {
                room.denizens.remove(&denizen_id);
            });
        }
        self.for_room(room_id, &|mut room| {
            room.denizens.insert(denizen_id);
        })
    }

    fn for_denizen(&mut self, denizen_id: i64, action: &Fn(&mut Denizen)) {
        if denizen_id == 0 {
            // Do not do anything to the null room. It's wasted breath.
            return;
        }

        if let Some(denizen) = self
            .non_agent_states
            .get_mut(&format_denizen_id(denizen_id))
            .and_then(AetNonAgent::as_denizen_mut)
        {
            action(denizen);
        }
    }

    fn check_denizen<R>(&self, denizen_id: i64, predicate: &Fn(&Denizen) -> R) -> Option<R> {
        if let Some(denizen) = self
            .non_agent_states
            .get(&format_denizen_id(denizen_id))
            .and_then(AetNonAgent::as_denizen)
        {
            Some(predicate(denizen))
        } else {
            None
        }
    }
}
