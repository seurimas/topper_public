pub mod denizen;
pub mod rooms;
pub use denizen::*;
pub use rooms::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum AetNonAgent {
    Room(Room),
    Denizen(Denizen),
    Players(Vec<String>),
}

impl AetNonAgent {
    pub fn as_denizen(&self) -> Option<&Denizen> {
        match self {
            AetNonAgent::Denizen(denizen) => Some(denizen),
            _ => None,
        }
    }
    pub fn as_denizen_mut(&mut self) -> Option<&mut Denizen> {
        match self {
            AetNonAgent::Denizen(denizen) => Some(denizen),
            _ => None,
        }
    }

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
