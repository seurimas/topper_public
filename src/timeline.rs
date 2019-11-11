use crate::actions::*;
use crate::alpha_beta::*;
use crate::classes::get_offensive_actions;
use crate::types::*;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct PromptStats {
    pub health: CType,
    pub mana: CType,
    pub equilibrium: bool,
    pub balance: bool,
    pub shadow: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum Observation {
    Connects(String),
    Afflicts(FType),
    Cures(FType),
    Rebounds,
    Shield,
    Diverts,
    Dodges,
}

#[derive(Debug, Deserialize)]
pub struct CombatAction {
    pub caster: String,
    pub category: String,
    pub skill: String,
    pub annotation: String,
    pub target: String,
    pub associated: Vec<Observation>,
}

impl CombatAction {
    pub fn rebounded(&self) -> bool {
        let mut rebounded = false;
        for observation in self.associated.iter() {
            if *observation == Observation::Rebounds {
                rebounded = true;
                break;
            }
        }
        rebounded
    }
}

#[derive(Debug, Deserialize)]
pub enum SimpleCure {
    Herb(String),
    Salve(String),
    Smoke(String),
}

#[derive(Debug, Deserialize)]
pub struct SimpleCureAction {
    pub cure_type: SimpleCure,
    pub caster: String,
    pub target: String,
    pub associated: Vec<Observation>,
}

#[derive(Debug, Deserialize)]
pub enum Incident {
    CombatAction(CombatAction),
    SimpleCureAction(SimpleCureAction),
}

#[derive(Debug, Deserialize)]
pub enum Prompt {
    Blackout,
    Stats(PromptStats),
}

#[derive(Debug, Deserialize)]
pub struct TimeSlice {
    pub incidents: Vec<Incident>,
    pub prompt: Prompt,
    pub time: CType,
}

pub struct TimelineState {
    agent_states: HashMap<String, AgentState>,
}

impl TimelineState {
    pub fn new() -> Self {
        TimelineState {
            agent_states: HashMap::new(),
        }
    }

    pub fn get_agent(&mut self, name: &String) -> AgentState {
        self.agent_states
            .get_mut(name)
            .unwrap_or(&mut BASE_STATE.clone())
            .clone()
    }
}

pub struct Timeline {
    pub slices: Vec<TimeSlice>,
    pub state: TimelineState,
}
impl Timeline {
    pub fn new() -> Self {
        Timeline {
            slices: Vec::new(),
            state: TimelineState::new(),
        }
    }

    pub fn push_time_slice(&mut self, slice: TimeSlice) {
        self.slices.push(slice);
    }
}

lazy_static! {
    static ref BASE_STATE: AgentState = {
        let mut val = AgentState::default();
        val.initialize_stat(SType::Health, 4000);
        val.initialize_stat(SType::Mana, 4000);
        val.set_flag(FType::Player, true);
        val.set_flag(FType::Blindness, true);
        val.set_flag(FType::Deafness, true);
        val.set_flag(FType::Frost, true);
        val.set_flag(FType::Levitation, true);
        val.set_flag(FType::Speed, true);
        val.set_flag(FType::Frost, true);
        val.set_flag(FType::Vigor, true);
        val.set_flag(FType::Rebounding, true);
        val
    };
}

pub fn apply_observed_afflictions(
    who: &mut AgentState,
    aff_count: i32,
    observations: &Vec<Observation>,
) {
    let mut found_affs = 0;
    for observation in observations.iter() {
        match observation {
            Observation::Afflicts(affliction) => {
                who.set_flag(*affliction, true);
                found_affs += 1;
            }
            _ => {}
        }
    }
    if found_affs < aff_count {}
}
