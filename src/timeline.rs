use crate::actions::*;
use crate::alpha_beta::*;
use crate::classes::{get_offensive_actions, handle_combat_action};
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
    Balance(BType, f32),
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
    time: CType,
}

impl TimelineState {
    pub fn new() -> Self {
        TimelineState {
            agent_states: HashMap::new(),
            time: 0,
        }
    }

    pub fn get_agent(&mut self, name: &String) -> AgentState {
        self.agent_states
            .get_mut(name)
            .unwrap_or(&mut BASE_STATE.clone())
            .clone()
    }

    pub fn set_agent(&mut self, name: &String, state: AgentState) {
        self.agent_states.insert(name.to_string(), state);
    }

    fn wait(&mut self, duration: CType) {
        for agent_state in self.agent_states.values_mut() {
            agent_state.wait(duration);
        }
    }

    fn apply_time_slice(&mut self, slice: &TimeSlice) {
        if slice.time > self.time {
            self.wait(slice.time - self.time);
        }
        for incident in slice.incidents.iter() {
            match incident {
                Incident::CombatAction(combat_action) => {
                    handle_combat_action(&combat_action, self);
                }
                _ => {}
            }
        }
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
        self.state.apply_time_slice(&slice);
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

pub fn apply_or_infer_balance(
    who: &mut AgentState,
    expected_value: (BType, f32),
    observations: &Vec<Observation>,
) {
    for observation in observations.iter() {
        match observation {
            Observation::Balance(btype, duration) => {
                who.set_balance(*btype, *duration);
                return;
            }
            _ => {}
        }
    }
    who.set_balance(expected_value.0, expected_value.1);
}
