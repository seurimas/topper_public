use crate::actions::*;
use crate::types::*;
use std::collections::HashMap;

pub struct StateEvent {
    pub description: String,
    pub changes: HashMap<String, Vec<StateChange>>,
    pub time: CType,
}

#[derive(Debug)]
pub struct PromptStats {
    pub health: CType,
    pub mana: CType,
    pub equilibrium: bool,
    pub balance: bool,
    pub shadow: bool,
}

#[derive(Debug)]
pub enum CombatActionObservation {
    Connects(String),
    Afflict(String),
    Rebounds,
    Shield,
    Diverts,
    Dodges,
}

#[derive(Debug)]
pub struct CombatAction {
    pub caster: String,
    pub category: String,
    pub skill: String,
    pub annotation: String,
    pub target: Option<String>,
    pub associated: Vec<CombatActionObservation>,
}

#[derive(Debug)]
pub enum Incident {
    CombatAction(CombatAction),
}

#[derive(Debug)]
pub enum Prompt {
    Blackout,
    Stats(PromptStats),
}

#[derive(Debug)]
pub struct TimeSlice {
    pub incidents: Vec<Incident>,
    pub prompt: Prompt,
    pub time: CType,
}

pub fn parse_time_slice(line: &String) -> TimeSlice {
    TimeSlice {
        incidents: Vec::new(),
        prompt: Prompt::Blackout,
        time: 0,
    }
}
