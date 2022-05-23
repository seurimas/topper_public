use super::apply_functions::*;
use crate::classes::{get_skill_class, handle_combat_action, handle_sent, Class, VENOM_AFFLICTS};
use crate::curatives::{
    handle_simple_cure_action, top_aff, CALORIC_TORSO_ORDER, PILL_CURE_ORDERS, PILL_DEFENCES,
    SALVE_CURE_ORDERS, SMOKE_CURE_ORDERS,
};
use crate::db::AetDatabaseModule;
use crate::types::*;
use log::warn;
use regex::Regex;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::ops::{Deref, DerefMut};
use topper_core::observations::EnumFromArgs;
use topper_core::timeline::db::DatabaseModule;
use topper_core::timeline::types::*;

#[derive(Debug, Deserialize, Clone)]
pub enum AetPrompt {
    Promptless,
    Blackout,
    Simulation,
    Stats(PromptStats),
}
pub type AetTimeSlice = TimeSlice<AetObservation, AetPrompt>;
pub type AetTimelineState = TimelineState<AgentState>;
pub type AetTimeline = Timeline<AetObservation, AetPrompt, AgentState>;

#[derive(Debug, Deserialize, Clone)]
pub struct PromptStats {
    pub health: CType,
    pub mana: CType,
    pub sp: CType,
    pub equilibrium: bool,
    pub balance: bool,
    pub shadow: bool,
    pub prone: bool,
}

pub fn simulation_slice(observations: Vec<AetObservation>, time: CType) -> AetTimeSlice {
    AetTimeSlice {
        observations: Some(observations),
        gmcp: Vec::new(),
        lines: Vec::new(),
        prompt: AetPrompt::Simulation,
        time,
        me: "".to_string(),
    }
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
        category: &str,
        skill: &str,
        annotation: &str,
        target: &str,
    ) -> AetObservation {
        AetObservation::CombatAction(CombatAction {
            caster: caster.to_string(),
            target: target.to_string(),
            category: category.to_string(),
            skill: skill.to_string(),
            annotation: annotation.to_string(),
        })
    }
    pub fn proc_observation(
        caster: &str,
        category: &str,
        skill: &str,
        annotation: &str,
        target: &str,
    ) -> AetObservation {
        AetObservation::Proc(CombatAction {
            caster: caster.to_string(),
            target: target.to_string(),
            category: category.to_string(),
            skill: skill.to_string(),
            annotation: annotation.to_string(),
        })
    }
    pub fn normalized(&self) -> Self {
        crate::classes::mirrors::normalize_combat_action(self)
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

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Perspective {
    Attacker,
    Target,
    Bystander,
}

#[derive(Debug, Deserialize, PartialEq, Clone, EnumFromArgs)]
pub enum AetObservation {
    // Basic actions
    #[skip_args]
    CombatAction(CombatAction),
    #[skip_args]
    Proc(CombatAction),
    #[skip_args]
    SimpleCureAction(SimpleCureAction),
    #[skip_args]
    DualWield {
        who: String,
        left: String,
        right: String,
    },
    #[skip_args]
    Wield {
        who: String,
        what: String,
        hand: String,
    },
    #[skip_args]
    Unwield {
        who: String,
        what: String,
        hand: String,
    },
    TwoHandedWield(String, String),
    // Action-related
    Connects(String),
    Devenoms(String),
    ParryStart(String, String),
    Parry(String, String),
    Damaged(String, String),
    Mangled(String, String),
    Absorbed(String, String),
    DiscernedAfflict(String),
    HiddenAff,
    Rebounds,
    Diverts,
    Dodges(String),
    Misses(String),
    OtherAfflicted(String, String),
    DiscernedCure(String, String),
    LostRebound(String),
    LostShield(String),
    LostFangBarrier(String),
    PurgeVenom(String, String),
    FlameShield(String),
    Fangbarrier,
    Shield,
    ListStart(String, String),
    ListItem(String, String, String, String),
    Undithered,
    // First-Aid simples
    Afflicted(String),
    Discovered(String),
    Cured(String),
    Gained(String, String),
    Stripped(String),
    // Specific case, non-action
    Relapse(String),
    TickAff(String, String),
    // Pipes
    FillPipe(String),
    PipeEmpty,
    // General messages
    #[skip_args]
    Balance(String, f32),
    BalanceBack(String),
    #[skip_args]
    LimbDamage(String, f32),
    #[skip_args]
    LimbHeal(String, f32),
    LimbDone(String),
    Fall(String),
    Stand(String),
    Sent(String),
}

pub trait AetTimelineStateTrait {
    fn get_perspective(&self, combat_action: &CombatAction) -> Perspective;

    fn set_flag_for_agent(
        &mut self,
        who: &String,
        flag_name: &String,
        val: bool,
    ) -> Result<(), String>;

    fn tick_counter_up_for_agent(&mut self, who: &String, flag_name: &String)
        -> Result<(), String>;

    fn adjust_agent_limb(&mut self, who: &String, what: &String, val: f32) -> Result<(), String>;

    fn finish_agent_restore(&mut self, who: &String, what: &String) -> Result<(), String>;

    fn apply_observation<DB: AetDatabaseModule>(
        &mut self,
        observation: &AetObservation,
        before: &Vec<AetObservation>,
        after: &Vec<AetObservation>,
        db: Option<&DB>,
    ) -> Result<(), String>;

    fn apply_time_slice<DB: AetDatabaseModule>(
        &mut self,
        slice: &TimeSlice<AetObservation, AetPrompt>,
        db: Option<&DB>,
    ) -> Result<(), String>;

    fn strikeout(&mut self);
}

impl AetTimelineStateTrait for AetTimelineState {
    fn get_perspective(&self, combat_action: &CombatAction) -> Perspective {
        if combat_action.caster.eq(&self.me) {
            Perspective::Attacker
        } else if combat_action.target.eq(&self.me) {
            Perspective::Target
        } else {
            Perspective::Bystander
        }
    }
    fn set_flag_for_agent(
        &mut self,
        who: &String,
        flag_name: &String,
        val: bool,
    ) -> Result<(), String> {
        let flag_name = flag_name.clone();
        self.for_agent(who, &move |me: &mut AgentState| {
            if let Ok((_damage_type, _damage_amount)) = get_damage_barrier(&flag_name) {
                // Do nothing...
            } else if let Some(aff_flag) = FType::from_name(&flag_name) {
                if aff_flag == FType::ThinBlood && !val {
                    me.clear_relapses();
                }
                if aff_flag == FType::Insomnia && val && me.is(FType::Hypersomnia) {
                } else {
                    me.set_flag(aff_flag, val);
                }
            } else {
                // Err(format!("Failed to find flag {}", flag_name));
            }
        });
        Ok(())
    }

    fn tick_counter_up_for_agent(
        &mut self,
        who: &String,
        flag_name: &String,
    ) -> Result<(), String> {
        let flag_name = flag_name.clone();
        self.for_agent(who, &move |me: &mut AgentState| {
            if let Some(aff_flag) = FType::from_name(&flag_name) {
                if aff_flag.is_counter() {
                    me.tick_flag_up(aff_flag);
                } else {
                    // return Err(format!("Tried to tick non-counter: {}", flag_name));
                }
            } else {
                // return Err(format!("Failed to find flag {}", flag_name));
            }
        });
        Ok(())
    }

    fn adjust_agent_limb(&mut self, who: &String, what: &String, val: f32) -> Result<(), String> {
        let limb = get_limb_damage(what)?;
        self.for_agent(who, &move |me: &mut AgentState| {
            me.limb_damage.adjust_limb(limb, (val * 100.0) as CType);
        });
        Ok(())
    }

    fn finish_agent_restore(&mut self, who: &String, what: &String) -> Result<(), String> {
        let limb = get_limb_damage(what)?;
        self.for_agent(who, &move |me: &mut AgentState| {
            me.complete_restoration(limb);
        });
        Ok(())
    }

    fn apply_observation<DB: AetDatabaseModule>(
        &mut self,
        observation: &AetObservation,
        before: &Vec<AetObservation>,
        after: &Vec<AetObservation>,
        db: Option<&DB>,
    ) -> Result<(), String> {
        apply_observation(self, observation, before, after, db)
    }

    fn apply_time_slice<DB: AetDatabaseModule>(
        &mut self,
        slice: &TimeSlice<AetObservation, AetPrompt>,
        db: Option<&DB>,
    ) -> Result<(), String> {
        self.me = slice.me.clone();
        self.update_time(slice.time);
        let mut before = Vec::new();
        let observations: Vec<AetObservation> = slice
            .observations
            .iter()
            .flatten()
            .cloned()
            .collect::<Vec<AetObservation>>();
        let mut after = observations.clone();
        for observation in observations.iter() {
            let obs_results = self.apply_observation(observation, &before, &after, db);
            if let Err(error) = obs_results {
                println!("Bad observation: {:?} ({})", observation, error);
            }
            if after.len() > 0 {
                let next = after.remove(0);
                before.push(next);
            }
        }
        if let AetPrompt::Stats(stats) = &slice.prompt {
            let sp = stats.sp;
            for_agent(self, &slice.me, &move |you| {
                you.set_stat(SType::SP, sp);
            });
        }
        self.strikeout();
        Ok(())
    }

    fn strikeout(&mut self) {
        for (key, values) in self.agent_states.iter_mut() {
            let mut lowest_strikes = usize::MAX;
            for branch in values.iter() {
                if branch.branch_state.strikes() < lowest_strikes {
                    lowest_strikes = branch.branch_state.strikes();
                }
            }
            let before = values.len();
            values.retain(|branch| branch.branch_state.strikes() == lowest_strikes);
            let mid = values.len();
            if mid > 32 {
                let mut set = HashSet::new();
                for branch in values.iter() {
                    set.insert(branch.clone());
                }
                values.splice(.., set);
            }
            let after = values.len();
            if before != after {
                println!("Strikeout! ({}: {} -> {} -> {})", key, before, mid, after);
            }
        }
    }
}

pub trait AetTimelineTrait {
    fn reset(&mut self, full: bool);
}

impl AetTimelineTrait for AetTimeline {
    fn reset(&mut self, full: bool) {
        if full {
            self.state.agent_states = HashMap::new();
        } else {
            for (key, val) in self.state.agent_states.iter_mut() {
                val.truncate(1);
                let mut agent = val.first_mut().unwrap();
                let mut affs = Vec::new();
                for aff in agent.flags.aff_iter() {
                    affs.push(aff);
                }
                for aff in affs.iter() {
                    agent.set_flag(*aff, false);
                }
                agent.branch_state = BranchState::Single;
                agent.set_flag(FType::Blindness, true);
                agent.set_flag(FType::Deafness, true);
                agent.set_flag(FType::Temperance, true);
                agent.set_flag(FType::Levitation, true);
                agent.set_flag(FType::Speed, true);
                agent.set_flag(FType::Temperance, true);
                agent.set_flag(FType::Vigor, true);
                agent.set_flag(FType::Rebounding, true);
                agent.set_flag(FType::Insomnia, true);
                agent.set_flag(FType::Fangbarrier, true);
                agent.set_flag(FType::Instawake, true);
                agent.set_flag(FType::Insulation, true);
                agent.limb_damage = LimbSet::default();
            }
        }
    }
}

impl<DB: AetDatabaseModule + DatabaseModule> BaseTimeline<AetObservation, AetPrompt, DB>
    for AetTimeline
{
    fn push_time_slice(
        &mut self,
        slice: TimeSlice<AetObservation, AetPrompt>,
        db: Option<&DB>,
    ) -> Result<(), String> {
        let result = self.state.apply_time_slice::<DB>(&slice, db);
        self.slices.push(slice);
        result
    }
}
