use crate::classes::{get_skill_class, handle_combat_action, handle_sent, Class, VENOM_AFFLICTS};
use crate::curatives::{
    handle_simple_cure_action, remove_in_order, top_aff, CALORIC_TORSO_ORDER, PILL_CURE_ORDERS,
    PILL_DEFENCES, SALVE_CURE_ORDERS, SMOKE_CURE_ORDERS,
};
use crate::types::*;
use log::warn;
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct PromptStats {
    pub health: CType,
    pub mana: CType,
    pub equilibrium: bool,
    pub balance: bool,
    pub shadow: bool,
    pub prone: bool,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct CombatAction {
    pub caster: String,
    pub category: String,
    pub skill: String,
    pub annotation: String,
    pub target: String,
}

impl CombatAction {
    pub fn observation(
        caster: &str,
        target: &str,
        category: &str,
        skill: &str,
        annotation: &str,
    ) -> Observation {
        Observation::CombatAction(CombatAction {
            caster: caster.to_string(),
            target: target.to_string(),
            category: category.to_string(),
            skill: skill.to_string(),
            annotation: annotation.to_string(),
        })
    }
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

impl SimpleCureAction {
    pub fn pill(caster: &str, pill: &str) -> Self {
        SimpleCureAction {
            cure_type: SimpleCure::Pill(pill.to_string()),
            caster: caster.to_string(),
        }
    }
    pub fn smoke(caster: &str, herb: &str) -> Self {
        SimpleCureAction {
            cure_type: SimpleCure::Smoke(herb.to_string()),
            caster: caster.to_string(),
        }
    }
    pub fn salve(caster: &str, salve: &str, location: &str) -> Self {
        SimpleCureAction {
            cure_type: SimpleCure::Salve(salve.to_string(), location.to_string()),
            caster: caster.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum Observation {
    // Basic actions
    CombatAction(CombatAction),
    SimpleCureAction(SimpleCureAction),
    DualWield {
        who: String,
        left: String,
        right: String,
    },
    Wield {
        who: String,
        what: String,
        hand: String,
    },
    Unwield {
        who: String,
        what: String,
        hand: String,
    },
    TwoHandedWield(String, String),
    // Action-related
    Connects(String),
    Devenoms(String),
    Parry(String, String),
    Absorbed(String, String),
    DiscernedAfflict(String),
    Rebounds,
    Diverts,
    Dodges,
    OtherAfflicted(String, String),
    DiscernedCure(String, String),
    LostRebound(String),
    LostShield(String),
    LostFangBarrier(String),
    PurgeVenom(String, String),
    Fangbarrier,
    Shield,
    // First-Aid simples
    Afflicted(String),
    Cured(String),
    Gained(String, String),
    Stripped(String),
    // Specific case, non-action
    Relapse(String),
    TickAff(String, String),
    // General messages
    Balance(String, f32),
    BalanceBack(String),
    LimbDamage(String, f32),
    LimbHeal(String, f32),
    LimbDone(String),
    Sent(String),
}

#[derive(Debug, Deserialize, Clone)]
pub enum Prompt {
    Promptless,
    Blackout,
    Simulation,
    Stats(PromptStats),
}

#[derive(Debug, Deserialize, Clone)]
pub struct TimeSlice {
    pub observations: Vec<Observation>,
    pub lines: Vec<(String, u32)>,
    pub prompt: Prompt,
    pub time: CType,
    pub me: String,
}

impl TimeSlice {
    pub fn simulation(observations: Vec<Observation>, time: CType) -> Self {
        TimeSlice {
            observations,
            lines: Vec::new(),
            prompt: Prompt::Simulation,
            time,
            me: "".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct TimelineState {
    agent_states: HashMap<String, AgentState>,
    free_hints: HashMap<String, String>,
    classes: HashMap<String, Class>,
    time: CType,
    pub me: String,
}

impl TimelineState {
    pub fn new() -> Self {
        let mut classes = HashMap::new();
        classes.insert("Haven".to_string(), Class::Luminary);
        TimelineState {
            agent_states: HashMap::new(),
            free_hints: HashMap::new(),
            classes,
            time: 0,
            me: "".to_string(),
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

    pub fn get_me(&mut self) -> AgentState {
        self.get_agent(&self.me.clone())
    }

    pub fn get_my_hint(&self, hint_type: &String) -> Option<String> {
        self.get_player_hint(&self.me, hint_type)
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

    pub fn set_flag_for_agent(
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
        } else if let Ok((_damage_type, _damage_amount)) = get_damage_barrier(flag_name) {
            // Do nothing...
        } else {
            return Err(format!("Failed to find flag {}", flag_name));
        }
        self.set_agent(who, me);
        Ok(())
    }

    pub fn tick_counter_up_for_agent(
        &mut self,
        who: &String,
        flag_name: &String,
    ) -> Result<(), String> {
        let mut me = self.get_agent(who);
        if let Some(aff_flag) = FType::from_name(flag_name) {
            if aff_flag.is_counter() {
                println!("Ticking up!");
                me.tick_flag_up(aff_flag);
            } else {
                return Err(format!("Tried to tick non-counter: {}", flag_name));
            }
        } else {
            return Err(format!("Failed to find flag {}", flag_name));
        }
        self.set_agent(who, me);
        Ok(())
    }

    fn adjust_agent_limb(&mut self, who: &String, what: &String, val: f32) -> Result<(), String> {
        let mut me = self.get_agent(who);
        let limb = get_limb_damage(what)?;
        me.adjust_limb(limb, (val * 100.0) as CType);
        self.set_agent(who, me);
        Ok(())
    }

    fn finish_agent_restore(&mut self, who: &String, what: &String) -> Result<(), String> {
        let mut me = self.get_agent(who);
        let limb = get_limb_damage(what)?;
        me.complete_restoration(limb);
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
                if let Some(known_class) = get_skill_class(&combat_action.category) {
                    self.classes
                        .insert(combat_action.caster.clone(), known_class);
                }
                handle_combat_action(combat_action, self, before, after)?;
            }
            Observation::SimpleCureAction(simple_cure) => {
                handle_simple_cure_action(simple_cure, self, before, after)?;
            }
            Observation::DiscernedCure(who, affliction) => {
                self.set_flag_for_agent(who, affliction, false)?;
            }
            Observation::Cured(affliction) => {
                self.set_flag_for_agent(&self.me.clone(), affliction, false)?;
            }
            Observation::Afflicted(affliction) => {
                self.set_flag_for_agent(&self.me.clone(), affliction, true)?;
            }
            Observation::OtherAfflicted(who, affliction) => {
                println!("{} {}", who, affliction);
                self.set_flag_for_agent(who, affliction, true)?;
            }
            Observation::Stripped(defense) => {
                self.set_flag_for_agent(&self.me.clone(), defense, false)?;
            }
            Observation::LostRebound(who) => {
                self.set_flag_for_agent(who, &"Rebounding".to_string(), false)?;
            }
            Observation::LostShield(who) => {
                self.set_flag_for_agent(who, &"Shielded".to_string(), false)?;
            }
            Observation::LostFangBarrier(who) => {
                self.set_flag_for_agent(who, &"Fangbarrier".to_string(), false)?;
            }
            Observation::Gained(who, defence) => {
                self.set_flag_for_agent(who, defence, true)?;
                if defence.eq("rebounding") {
                    let mut me = self.get_agent(who);
                    me.set_balance(BType::Rebounding, 0.0);
                    self.set_agent(who, me);
                }
            }
            Observation::LimbDamage(what, much) => {
                self.adjust_agent_limb(&self.me.clone(), what, *much)?;
            }
            Observation::LimbHeal(what, much) => {
                self.adjust_agent_limb(&self.me.clone(), what, -much)?;
            }
            Observation::LimbDone(what) => {
                self.finish_agent_restore(&self.me.clone(), what)?;
            }
            Observation::Parry(who, what) => {
                let mut me = self.get_agent(who);
                let limb = get_limb_damage(what)?;
                me.set_parrying(limb);
                self.set_agent(who, me);
            }
            Observation::Wield { who, what, hand } => {
                let left = if hand.eq("left") {
                    Some(what.clone())
                } else {
                    None
                };
                let right = if hand.eq("right") {
                    Some(what.clone())
                } else {
                    None
                };
                let mut me = self.get_agent(who);
                me.wield_multi(left, right);
                self.set_agent(who, me);
            }
            Observation::Unwield { who, what: _, hand } => {
                let left = hand.eq("left");
                let right = hand.eq("right");
                let mut me = self.get_agent(who);
                me.unwield_multi(left, right);
                self.set_agent(who, me);
            }
            Observation::DualWield { who, left, right } => {
                let mut me = self.get_agent(who);
                me.wield_multi(Some(left.clone()), Some(right.clone()));
                self.set_agent(who, me);
            }
            Observation::TwoHandedWield(who, what) => {
                let mut me = self.get_agent(who);
                me.wield_two_hands(what.clone());
                self.set_agent(who, me);
            }
            Observation::TickAff(who, what) => {
                self.tick_counter_up_for_agent(who, what)?;
            }
            Observation::Relapse(who) => {
                if before.len() == 0 {
                    // Just don't do the next check.
                } else if let Some(Observation::Relapse(last_relapse)) =
                    before.get(before.len() - 1)
                {
                    if last_relapse.eq(who) {
                        // We've already handled this block.
                        return Ok(());
                    }
                }
                let mut you = self.get_agent(who);
                apply_or_infer_relapse(&mut you, after)?;
                self.set_agent(who, you);
            }
            _ => {}
        }
        Ok(())
    }

    fn apply_time_slice(&mut self, slice: &TimeSlice) -> Result<(), String> {
        self.me = slice.me.clone();
        if slice.time > self.time {
            self.wait(slice.time - self.time);
            self.time = slice.time;
        }
        let mut before = Vec::new();
        let mut after = slice.observations.clone();
        for observation in slice.observations.iter() {
            let obs_results = self.apply_observation(observation, &before, &after);
            if let Err(error) = obs_results {
                println!("Bad observation: {:?} ({})", observation, error);
            }
            if after.len() > 0 {
                let next = after.remove(0);
                before.push(next);
            }
        }
        if let Prompt::Stats(prompt) = &slice.prompt {
            self.set_flag_for_agent(&self.me.clone(), &"prone".to_string(), prompt.prone)?;
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

    pub fn branch(&self) -> Self {
        Timeline {
            slices: Vec::new(),
            state: self.state.clone(),
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

    pub fn who_am_i(&self) -> String {
        self.state.me.clone()
    }

    pub fn get_my_class(&self) -> Option<Class> {
        self.state.classes.get(&self.state.me).cloned()
    }

    pub fn get_class(&self, who: &str) -> Option<Class> {
        self.state.classes.get(who).cloned()
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
        val.set_flag(FType::Temperance, true);
        val.set_flag(FType::Levitation, true);
        val.set_flag(FType::Speed, true);
        val.set_flag(FType::Temperance, true);
        val.set_flag(FType::Vigor, true);
        val.set_flag(FType::Rebounding, true);
        val.set_flag(FType::Insomnia, true);
        val.set_flag(FType::Fangbarrier, true);
        val.set_flag(FType::Instawake, true);
        val
    };
}

pub fn apply_or_infer_suggestion(
    who: &mut AgentState,
    after: &Vec<Observation>,
) -> Result<(), String> {
    if let Some(Observation::DiscernedAfflict(affliction)) = after.get(1) {
        println!("Discerned correct!");
        if let Some(affliction) = FType::from_name(affliction) {
            who.set_flag(affliction, true);
        }
    } else if let Some(Observation::DiscernedAfflict(affliction)) = after.get(1) {
        println!(
            "Expected {:?} but got {:?}!",
            who.hypnosis_stack.get(0),
            affliction
        );
        if let Some(affliction) = FType::from_name(affliction) {
            who.set_flag(affliction, true);
        }
    } else if let Some(Hypnosis::Aff(_affliction)) = who.hypnosis_stack.get(0) {
        println!(
            "Expected {:?} but got {:?}!",
            who.hypnosis_stack.get(0),
            after.get(0)
        );
    }
    if who.hypnosis_stack.len() > 0 {
        who.hypnosis_stack.remove(0);
    }
    if who.hypnosis_stack.len() == 0 {
        who.set_flag(FType::Snapped, false);
    }
    Ok(())
}

pub fn apply_venom(who: &mut AgentState, venom: &String) -> Result<(), String> {
    if who.is(FType::ThinBlood) {
        who.push_toxin(venom.clone());
    }
    if venom == "prefarar" && who.is(FType::Deafness) {
        who.set_flag(FType::Deafness, false);
    } else if venom == "oculus" && who.is(FType::Blindness) {
        who.set_flag(FType::Deafness, false);
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
    } else if let Some(affliction) = VENOM_AFFLICTS.get(venom) {
        who.set_flag(*affliction, true);
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

lazy_static! {
    static ref CALLED_VENOM: Regex = Regex::new(r"(\w+)").unwrap();
}

lazy_static! {
    static ref CALLED_VENOMS_TWO: Regex = Regex::new(r"(\w+),? (\w+)").unwrap();
}

pub fn apply_weapon_hits(
    attacker: &mut AgentState,
    target: &mut AgentState,
    observations: &Vec<Observation>,
    first_person: bool,
    venom_hints: Option<String>,
) -> Result<(), String> {
    if first_person {
        for i in 0..observations.len() {
            if let Some(Observation::Devenoms(venom)) = observations.get(i) {
                if let Some(Observation::Rebounds) = observations.get(i - 1) {
                    apply_venom(attacker, venom)?;
                } else {
                    if let Some(Observation::PurgeVenom(_, _v2)) = observations.get(i + 1) {
                    } else {
                        apply_venom(target, venom)?;
                    }
                }
            }
        }
    } else if let Some(venom_hints) = venom_hints {
        let mut venoms = Vec::new();
        if let Some(captures) = CALLED_VENOMS_TWO.captures(&venom_hints) {
            venoms.push(captures.get(1).unwrap().as_str().to_string());
            venoms.push(captures.get(2).unwrap().as_str().to_string());
        } else if let Some(captures) = CALLED_VENOM.captures(&venom_hints) {
            venoms.push(captures.get(1).unwrap().as_str().to_string());
        }
        if (Some(&Observation::Dodges) == observations.get(0))
            || (Some(&Observation::Dodges) == observations.get(1))
        {
            println!("Found dodge!");
            venoms.pop();
        }
        println!("Caught {:?}", venoms);
        for venom in venoms.iter() {
            apply_venom(target, venom)?;
        }
    }
    Ok(())
}

pub fn apply_or_infer_relapse(
    who: &mut AgentState,
    after: &Vec<Observation>,
) -> Result<(), String> {
    let mut relapse_count = 1;
    let mut name = "";
    for observation in after.iter() {
        if let Observation::Relapse(next_name) = observation {
            if name.eq("") {
                name = next_name;
            } else if name.eq(next_name) {
                relapse_count += 1;
            } else {
                break;
            }
        }
    }
    println!("{} {}", relapse_count, name);
    match who.get_relapses(relapse_count) {
        RelapseResult::Concrete(venoms) => {
            println!("Relapses: {:?}", venoms);
            for venom in venoms.iter() {
                apply_venom(who, &venom)?;
            }
        }
        RelapseResult::Uncertain(venoms) => {
            println!("Possible relapses: {:?}", venoms);
        }
        RelapseResult::None => {
            println!("No relapses found???");
        }
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
                    if aff == FType::ThinBlood {
                        who.clear_relapses();
                    } else if aff == FType::Void {
                        who.set_flag(FType::Weakvoid, true);
                    }
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
    if let Some(Observation::Cured(aff_name)) = after.get(1) {
        if let Some(aff) = FType::from_name(&aff_name) {
            who.set_flag(aff, false);
            found_cures.push(aff);
        }
    } else if let Some(Observation::DiscernedCure(_you, aff_name)) = after.get(1) {
        if let Some(aff) = FType::from_name(&aff_name) {
            who.set_flag(aff, false);
            if aff == FType::Void {
                who.set_flag(FType::Weakvoid, true);
            }
            found_cures.push(aff);
        }
    } else if let Some(Observation::Stripped(def_name)) = after.get(1) {
        if let Some(def) = FType::from_name(&def_name) {
            who.set_flag(def, false);
            found_cures.push(def);
        }
    } else {
        match cure {
            SimpleCure::Pill(pill_name) => {
                if who.is(FType::Anorexia) {
                    who.set_flag(FType::Anorexia, false);
                    warn!("Missed Anorexia cure!");
                }
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
                if who.is(FType::Slickness) {
                    who.set_flag(FType::Slickness, false);
                    warn!("Missed Slickness cure!");
                }
                if salve_name == "caloric" {
                    if who.some(CALORIC_TORSO_ORDER.to_vec()) {
                        remove_in_order(CALORIC_TORSO_ORDER.to_vec())(who);
                    } else {
                        who.set_flag(FType::Insulation, true);
                    }
                } else if salve_name == "mass" {
                    who.set_flag(FType::Density, true);
                } else if salve_name == "restoration" {
                    let limb = get_limb_damage(salve_loc)?;
                    who.set_restoring(limb);
                } else if let Some(order) =
                    SALVE_CURE_ORDERS.get(&(salve_name.to_string(), salve_loc.to_string()))
                {
                    remove_in_order(order.to_vec())(who);
                } else {
                    return Err(format!("Could not find {} on {}", salve_name, salve_loc));
                }
            }
            SimpleCure::Smoke(herb_name) => {
                if who.is(FType::Asthma) {
                    who.set_flag(FType::Asthma, false);
                    warn!("Missed Asthma cure!");
                }
                if let Some(order) = SMOKE_CURE_ORDERS.get(herb_name) {
                    remove_in_order(order.to_vec())(who);
                } else if herb_name == "reishi" {
                    who.set_balance(BType::Rebounding, 6.25);
                } else {
                    return Err(format!("Could not find smoke {}", herb_name));
                }
            } // _ => {}
        }
    }
    Ok(found_cures)
}
