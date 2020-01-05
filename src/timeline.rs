use crate::actions::*;
use crate::alpha_beta::*;
use crate::classes::{handle_combat_action, handle_sent, VENOM_AFFLICTS};
use crate::curatives::{
    handle_simple_cure_action, remove_in_order, top_aff, CALORIC_TORSO_ORDER, PILL_CURE_ORDERS,
    PILL_DEFENCES, SALVE_CURE_ORDERS, SMOKE_CURE_ORDERS,
};
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

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct CombatAction {
    pub caster: String,
    pub category: String,
    pub skill: String,
    pub annotation: String,
    pub target: String,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum SimpleCure {
    Pill(String),
    Salve(String, String),
    Smoke(String),
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct SimpleCureAction {
    pub cure_type: SimpleCure,
    pub caster: String,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum Observation {
    CombatAction(CombatAction),
    SimpleCureAction(SimpleCureAction),
    Connects(String),
    Devenoms(String),
    Afflicted(String),
    OtherAfflicted(String, String),
    Cured(String),
    DiscernedCure(String, String),
    DiscernedAfflict(String),
    Gained(String, String),
    LostRebound(String),
    Stripped(String),
    Balance(String, f32),
    BalanceBack(String),
    Rebounds,
    Fangbarrier,
    Shield,
    Diverts,
    Dodges,
    Relapse(String),
    Sent(String),
}

#[derive(Debug, Deserialize)]
pub enum Prompt {
    Promptless,
    Blackout,
    Stats(PromptStats),
}

#[derive(Debug, Deserialize)]
pub struct TimeSlice {
    pub observations: Vec<Observation>,
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
    free_hints: HashMap<String, String>,
    time: CType,
}

impl TimelineState {
    pub fn new() -> Self {
        TimelineState {
            agent_states: HashMap::new(),
            free_hints: HashMap::new(),
            hints: HashMap::new(),
            time: 0,
        }
    }

    pub fn add_player_hint(&mut self, name: &str, hint_type: &str, hint: String) {
        self.free_hints
            .insert(format!("{}_{}", name, hint_type), hint);
    }

    pub fn get_player_hint(&self, name: &String, hint_type: &String) -> Option<String> {
        self.free_hints
            .get(&format!("{}_{}", name, hint_type))
            .cloned()
    }

    pub fn get_agent(&mut self, name: &String) -> AgentState {
        self.agent_states
            .get_mut(name)
            .unwrap_or(&mut BASE_STATE.clone())
            .clone()
    }

    pub fn borrow_agent(&self, name: &String) -> AgentState {
        self.agent_states
            .get(name)
            .unwrap_or(&mut BASE_STATE.clone())
            .clone()
    }

    pub fn set_agent(&mut self, name: &String, state: AgentState) {
        self.agent_states.insert(name.to_string(), state);
    }

    fn set_flag_for_agent(
        &mut self,
        who: &String,
        flag_name: &String,
        val: bool,
    ) -> Result<(), String> {
        let mut me = self.get_agent(who);
        if let Some(aff_flag) = FType::from_name(flag_name) {
            if aff_flag == FType::ThinBlood && !val {
                me.clear_relapses();
            }
            me.set_flag(aff_flag, val);
        } else {
            return Err(format!("Failed to find flag {}", flag_name));
        }
        self.set_agent(who, me);
        Ok(())
    }

    fn wait(&mut self, duration: CType) {
        for agent_state in self.agent_states.values_mut() {
            agent_state.wait(duration);
        }
    }

    pub fn apply_observation(
        &mut self,
        observation: &Observation,
        before: &Vec<Observation>,
        after: &Vec<Observation>,
    ) -> Result<(), String> {
        match observation {
            Observation::Sent(command) => {
                handle_sent(command, self);
            }
            Observation::CombatAction(combat_action) => {
                handle_combat_action(combat_action, self, before, after)?;
            }
            Observation::SimpleCureAction(simple_cure) => {
                handle_simple_cure_action(simple_cure, self, before, after)?;
            }
            Observation::DiscernedCure(who, affliction) => {
                self.set_flag_for_agent(who, affliction, false)?;
            }
            Observation::Cured(affliction) => {
                self.set_flag_for_agent(&"Seurimas".into(), affliction, false)?;
            }
            Observation::Afflicted(affliction) => {
                self.set_flag_for_agent(&"Seurimas".into(), affliction, true)?;
            }
            Observation::OtherAfflicted(who, affliction) => {
                println!("{} {}", who, affliction);
                self.set_flag_for_agent(who, affliction, true)?;
            }
            Observation::Stripped(defense) => {
                self.set_flag_for_agent(&"Seurimas".into(), defense, false)?;
            }
            Observation::LostRebound(who) => {
                self.set_flag_for_agent(who, &"Rebounding".to_string(), false)?;
            }
            Observation::Gained(who, defence) => {
                self.set_flag_for_agent(who, defence, true)?;
            }
            Observation::Relapse(who) => {
                let mut you = self.get_agent(who);
                apply_or_infer_relapse(&mut you, after)?;
                self.set_agent(who, you);
            }
            _ => {}
        }
        Ok(())
    }

    fn apply_time_slice(&mut self, slice: &TimeSlice) -> Result<(), String> {
        if slice.time > self.time {
            self.wait(slice.time - self.time);
            self.time = slice.time;
        }
        let mut before = Vec::new();
        let mut after = slice.observations.clone();
        for observation in slice.observations.iter() {
            self.apply_observation(observation, &before, &after)?;
            if after.len() > 0 {
                let next = after.remove(0);
                before.push(next);
            }
        }
        Ok(())
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

    pub fn reset(&mut self) {
        self.state.agent_states = HashMap::new();
    }

    pub fn push_time_slice(&mut self, slice: TimeSlice) -> Result<(), String> {
        let result = self.state.apply_time_slice(&slice);
        self.slices.push(slice);
        result
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
        val.set_flag(FType::HardenedSkin, true);
        val.set_flag(FType::Energetic, true);
        val
    };
}

pub fn pop_suggestion(who: &mut AgentState) {
    who.hypnosis_stack.pop();
}

pub fn push_suggestion(who: &mut AgentState, suggestion: Hypnosis) {
    who.hypnosis_stack.push(suggestion);
}

pub fn apply_or_infer_suggestion(
    who: &mut AgentState,
    after: &Vec<Observation>,
) -> Result<(), String> {
    if let Some(Observation::DiscernedAfflict(affliction)) = after.get(0) {
        // This is fine. Discerned afflict will be applied independently.
    } else if let Some(Hypnosis::Aff(affliction)) = who.hypnosis_stack.get(0) {
        // Discernment is off??? Might be out of sync, don't assume.
        // who.set_flag(*affliction, true);
    }
    who.hypnosis_stack.remove(0);
    if who.hypnosis_stack.len() == 0 {
        who.set_flag(FType::Snapped, false);
    }
    Ok(())
}

pub fn apply_venom(who: &mut AgentState, venom: &String) -> Result<(), String> {
    if who.is(FType::ThinBlood) {
        who.push_toxin(venom.clone());
    }
    if let Some(affliction) = VENOM_AFFLICTS.get(venom) {
        who.set_flag(*affliction, true);
    } else if venom == "epseth" {
        if who.is(FType::LeftLegBroken) {
            who.set_flag(FType::RightLegBroken, true);
        } else {
            who.set_flag(FType::LeftLegBroken, true);
        }
    } else if venom == "epteth" {
        if who.is(FType::LeftArmBroken) {
            who.set_flag(FType::RightArmBroken, true);
        } else {
            who.set_flag(FType::LeftArmBroken, true);
        }
    } else if venom == "camus" {
        who.set_stat(SType::Health, who.get_stat(SType::Health) - 1000);
    } else if venom == "delphinium" && who.is(FType::Insomnia) {
        who.set_flag(FType::Insomnia, false);
    } else if venom == "delphinium" {
        who.set_flag(FType::Asleep, true);
    } else {
        return Err(format!("Could not determine effect of {}", venom));
    }
    Ok(())
}

pub fn apply_weapon_hits(
    attacker: &mut AgentState,
    target: &mut AgentState,
    observations: &Vec<Observation>,
) -> Result<(), String> {
    for i in 0..observations.len() {
        if let Some(Observation::Devenoms(venom)) = observations.get(i) {
            if let Some(Observation::Rebounds) = observations.get(i + 1) {
                apply_venom(attacker, venom)?;
            } else {
                apply_venom(target, venom)?;
            }
        }
    }
    Ok(())
}

pub fn apply_observed_venoms(
    who: &mut AgentState,
    observations: &Vec<Observation>,
) -> Result<(), String> {
    for observation in observations.iter() {
        match observation {
            Observation::Devenoms(venom) => {
                apply_venom(who, venom)?;
            }
            _ => {}
        }
    }
    Ok(())
}

pub fn apply_or_infer_relapse(
    who: &mut AgentState,
    observations: &Vec<Observation>,
) -> Result<(), String> {
    if let Some(venom) = who.relapse() {
        println!("Relapse: {}", venom);
        apply_venom(who, &venom)?;
    } else {
        println!("No relapse found...");
    }
    Ok(())
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

pub fn apply_or_infer_random_afflictions(
    who: &mut AgentState,
    after: &Vec<Observation>,
) -> Result<(), String> {
    for observation in after.iter() {
        match observation {
            Observation::DiscernedAfflict(aff_name) => {
                if let Some(aff) = FType::from_name(&aff_name) {
                    who.set_flag(aff, true);
                }
            }
            _ => {}
        }
    }
    Ok(())
}

pub fn apply_or_infer_cures(
    who: &mut AgentState,
    cures: Vec<FType>,
    after: &Vec<Observation>,
) -> Result<(), String> {
    let mut found_cures = Vec::new();
    for observation in after.iter() {
        match observation {
            Observation::Cured(aff_name) => {
                if let Some(aff) = FType::from_name(&aff_name) {
                    who.set_flag(aff, false);
                    found_cures.push(aff);
                }
            }
            Observation::Stripped(def_name) => {
                if let Some(def) = FType::from_name(&def_name) {
                    who.set_flag(def, false);
                    found_cures.push(def);
                }
            }
            _ => {}
        }
    }
    if found_cures.len() == 0 {
        remove_in_order(cures)(who);
    }
    Ok(())
}

pub fn apply_or_infer_cure(
    who: &mut AgentState,
    cure: &SimpleCure,
    after: &Vec<Observation>,
) -> Result<Vec<FType>, String> {
    let mut found_cures = Vec::new();
    if let Some(Observation::Cured(aff_name)) = after.get(0) {
        if let Some(aff) = FType::from_name(&aff_name) {
            who.set_flag(aff, false);
            found_cures.push(aff);
        }
    } else if let Some(Observation::DiscernedCure(you, aff_name)) = after.get(0) {
        if let Some(aff) = FType::from_name(&aff_name) {
            who.set_flag(aff, false);
            if aff == FType::Void {
                who.set_flag(FType::Weakvoid, true);
            }
            found_cures.push(aff);
        }
    } else if let Some(Observation::Stripped(def_name)) = after.get(0) {
        if let Some(def) = FType::from_name(&def_name) {
            who.set_flag(def, false);
            found_cures.push(def);
        }
    } else {
        match cure {
            SimpleCure::Pill(pill_name) => {
                if pill_name == "anabiotic" {
                } else if let Some(order) = PILL_CURE_ORDERS.get(pill_name) {
                    let cured = top_aff(who, order.to_vec());
                    remove_in_order(order.to_vec())(who);
                    if cured == Some(FType::ThinBlood) {
                        who.clear_relapses();
                    }
                } else if let Some(defence) = PILL_DEFENCES.get(pill_name) {
                    who.set_flag(*defence, true);
                } else {
                    return Err(format!("Could not find pill {}", pill_name));
                }
            }
            SimpleCure::Salve(salve_name, salve_loc) => {
                if salve_name == "caloric" {
                    if some(CALORIC_TORSO_ORDER.to_vec())(who, who) {
                        remove_in_order(CALORIC_TORSO_ORDER.to_vec())(who);
                    } else {
                        who.set_flag(FType::Insulation, true);
                    }
                } else if salve_name == "mass" {
                    who.set_flag(FType::Density, true);
                } else if let Some(order) =
                    SALVE_CURE_ORDERS.get(&(salve_name.to_string(), salve_loc.to_string()))
                {
                    remove_in_order(order.to_vec())(who);
                } else {
                    return Err(format!("Could not find {} on {}", salve_name, salve_loc));
                }
            }
            SimpleCure::Smoke(herb_name) => {
                if let Some(order) = SMOKE_CURE_ORDERS.get(herb_name) {
                    remove_in_order(order.to_vec())(who);
                } else {
                    return Err(format!("Could not find smoke {}", herb_name));
                }
            }
            _ => {}
        }
    }
    Ok(found_cures)
}
