pub mod rooms;
pub use rooms::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum AetNonAgent {
    Room(Room),
}

impl AetNonAgent {
    pub fn as_room(&self) -> Option<&Room> {
        match self {
            AetNonAgent::Room(room) => Some(room),
            _ => None,
        }
    }

    pub fn as_room_mut(&mut self) -> Option<&mut Room> {
        match self {
            AetNonAgent::Room(room) => Some(room),
            _ => None,
        }
    }
}
