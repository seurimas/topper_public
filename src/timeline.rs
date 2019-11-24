use crate::actions::*;
use crate::alpha_beta::*;
use crate::classes::{get_offensive_actions, handle_combat_action, VENOM_AFFLICTS};
use crate::curatives::{handle_simple_cure_action, remove_in_order, PILL_CURE_ORDERS};
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
    Devenoms(String),
    Afflicted(String),
    Cured(String),
    Stripped(String),
    Balance(String, f32),
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
    Pill(String),
    Salve(String, String),
    Smoke(String),
}

#[derive(Debug, Deserialize)]
pub struct SimpleCureAction {
    pub cure_type: SimpleCure,
    pub caster: String,
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

#[derive(Debug)]
pub struct Hints {
    pub strategy: String,
    pub class: String,
}

pub struct TimelineState {
    agent_states: HashMap<String, AgentState>,
    hints: HashMap<String, Hints>,
    time: CType,
}

impl TimelineState {
    pub fn new() -> Self {
        TimelineState {
            agent_states: HashMap::new(),
            hints: HashMap::new(),
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
                Incident::SimpleCureAction(simple_cure) => {
                    handle_simple_cure_action(&simple_cure, self);
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
    pub static ref BASE_STATE: AgentState = {
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
        val.set_flag(FType::Insomnia, true);
        val.set_flag(FType::Energetic, true);
        val
    };
}

pub fn apply_venom(who: &mut AgentState, venom: &String) {
    if let Some(affliction) = VENOM_AFFLICTS.get(venom) {
        who.set_flag(*affliction, true);
    } else if venom == "epseth" {
        if who.is(FType::BrokenLeftLeg) {
            who.set_flag(FType::BrokenRightLeg, true);
        } else {
            who.set_flag(FType::BrokenLeftLeg, true);
        }
    } else if venom == "epteth" {
        if who.is(FType::BrokenLeftArm) {
            who.set_flag(FType::BrokenRightArm, true);
        } else {
            who.set_flag(FType::BrokenLeftArm, true);
        }
    }
}

pub fn apply_observed_venoms(who: &mut AgentState, observations: &Vec<Observation>) {
    for observation in observations.iter() {
        match observation {
            Observation::Devenoms(venom) => {
                apply_venom(who, venom);
            }
            _ => {}
        }
    }
}

pub fn apply_or_infer_balance(
    who: &mut AgentState,
    expected_value: (BType, f32),
    observations: &Vec<Observation>,
) {
    for observation in observations.iter() {
        match observation {
            Observation::Balance(btype, duration) => {
                who.set_balance(BType::from_name(&btype), *duration);
                return;
            }
            _ => {}
        }
    }
    who.set_balance(expected_value.0, expected_value.1);
}

pub fn apply_or_infer_cure(
    who: &mut AgentState,
    cure: &SimpleCure,
    observations: &Vec<Observation>,
) -> Vec<FType> {
    let mut found_cures = Vec::new();
    for observation in observations.iter() {
        match observation {
            Observation::Cured(aff_name) => {
                let aff = FType::from_name(&aff_name);
                who.set_flag(aff, false);
                found_cures.push(aff);
            }
            Observation::Stripped(def_name) => {
                let def = FType::from_name(&def_name);
                who.set_flag(def, false);
                found_cures.push(def);
            }
            _ => {}
        }
    }
    if found_cures.len() == 0 {
        match cure {
            SimpleCure::Pill(pill_name) => {
                if let Some(order) = PILL_CURE_ORDERS.get(pill_name) {
                    remove_in_order(order.to_vec())(who);
                }
            }
            _ => {}
        }
    }
    found_cures
}
