use std::collections::{HashMap, HashSet};

use serde::Deserialize;

use crate::timeline::*;

use super::AetNonAgent;

#[derive(Debug, Deserialize, PartialEq, Clone, Hash, Eq)]
pub enum Direction {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest,
    Up,
    Down,
    In,
    Out,
}

impl Direction {
    pub fn from_short(short_name: &str) -> Option<Direction> {
        match short_name {
            "n" => Some(Direction::North),
            "ne" => Some(Direction::Northeast),
            "e" => Some(Direction::East),
            "se" => Some(Direction::Southeast),
            "s" => Some(Direction::South),
            "sw" => Some(Direction::Southwest),
            "w" => Some(Direction::West),
            "nw" => Some(Direction::Northwest),
            "up" => Some(Direction::Up),
            "down" => Some(Direction::Down),
            "in" => Some(Direction::In),
            "out" => Some(Direction::Out),
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Room {
    pub players: HashSet<String>,
    pub denizens: HashSet<i64>,
    pub exits: HashMap<Direction, i64>,
    tags: HashSet<String>,
}

impl Default for Room {
    fn default() -> Self {
        Room {
            players: HashSet::new(),
            denizens: HashSet::new(),
            exits: HashMap::new(),
            tags: HashSet::new(),
        }
    }
}

impl Room {
    pub fn default_state() -> AetNonAgent {
        AetNonAgent::Room(Room::default())
    }

    pub fn add_tag(&mut self, tag: &str) {
        self.tags.insert(tag.to_string());
    }

    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.remove(&tag.to_string());
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(&tag.to_string())
    }
}

pub fn format_room_id(room_id: i64) -> String {
    format!("room_{}", room_id)
}

pub trait AetTimelineRoomExt {
    fn for_room(&mut self, room_id: i64, action: &Fn(&mut Room));

    fn get_my_room(&self) -> Option<&Room>;

    fn get_my_room_mut(&mut self) -> Option<&mut Room>;

    fn set_player_room(&mut self, room_id: i64, player: &str);
}

impl AetTimelineRoomExt for AetTimelineState {
    fn for_room(&mut self, room_id: i64, action: &Fn(&mut Room)) {
        if room_id == 0 {
            // Do not do anything to the null room. It's wasted breath.
            return;
        }
        if let Some(AetNonAgent::Room(room)) =
            self.non_agent_states.get_mut(&format_room_id(room_id))
        {
            action(room);
        } else {
            self.non_agent_states
                .insert(format_room_id(room_id), Room::default_state());
            self.for_room(room_id, action);
        }
    }

    fn get_my_room(&self) -> Option<&Room> {
        let room_id = self.borrow_me().room_id;
        self.non_agent_states
            .get(&format_room_id(room_id))
            .and_then(AetNonAgent::as_room)
    }

    fn get_my_room_mut(&mut self) -> Option<&mut Room> {
        let room_id = self.borrow_me().room_id;
        self.non_agent_states
            .get_mut(&format_room_id(room_id))
            .and_then(AetNonAgent::as_room_mut)
    }

    fn set_player_room(&mut self, room_id: i64, player: &str) {
        let player = player.to_string();
        let old_room_id = self.borrow_agent(&player).room_id;
        self.for_room(old_room_id, &|room| {
            room.players.remove(&player);
        });
        self.for_room(room_id, &|room| {
            room.players.insert(player.clone());
        });
        self.for_agent(&player, &|me| {
            me.room_id = room_id;
        });
    }
}
