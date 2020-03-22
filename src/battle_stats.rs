use crate::io::Topper;
use crate::timeline::*;
use crate::types::*;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct PlayerStats {
    afflictions: Vec<String>,
    balances: HashMap<String, f32>,
    warnings: Vec<String>,
    lock_duration: Option<f32>,
}

fn get_hypno_warning(state: &AgentState) -> Option<String> {
    if let Some(aff) = state.get_next_hypno_aff() {
        Some(format!("<magenta>Next aff: <red>{:?}", aff))
    } else if state.hypnosis_stack.len() > 0 {
        Some(format!(
            "<magenta>Stack size: <red>{} {}",
            state.hypnosis_stack.len(),
            if !state.is(FType::Hypnotized) {
                "SEALED"
            } else {
                ""
            },
        ))
    } else {
        None
    }
}

fn get_lock_warning(state: &AgentState) -> Option<String> {
    use crate::classes::get_venoms;
    use crate::classes::syssin::{should_lock, SOFT_STACK};
    if should_lock(state, &get_venoms(SOFT_STACK.to_vec(), 3, &state)) {
        Some(format!("<pink>Close to a lock!"))
    } else {
        None
    }
}

impl PlayerStats {
    pub fn new() -> Self {
        PlayerStats {
            afflictions: Vec::new(),
            warnings: Vec::new(),
            balances: HashMap::new(),
            lock_duration: None,
        }
    }
    pub fn for_player(state: &AgentState) -> Self {
        let mut afflictions = Vec::new();
        for aff in state.flags.aff_iter() {
            afflictions.push(format!("{:?}", aff));
        }
        let mut warnings = Vec::new();
        if let Some(warning) = get_hypno_warning(&state) {
            warnings.push(warning);
        }
        if let Some(warning) = get_lock_warning(&state) {
            warnings.push(warning);
        }
        let mut balances = HashMap::new();
        balances.insert("Tree".to_string(), state.get_balance(BType::Tree));
        balances.insert("Focus".to_string(), state.get_balance(BType::Focus));
        balances.insert(
            "Rebounding".to_string(),
            state.get_balance(BType::Rebounding),
        );
        let lock_duration = state.lock_duration();
        PlayerStats {
            afflictions,
            warnings,
            balances,
            lock_duration,
        }
    }
}

#[derive(Serialize)]
pub struct BattleStats {
    pub feed: Vec<String>,
    pub my_stats: PlayerStats,
    pub target_stats: Option<PlayerStats>,
}

fn format_self_limbs(state: &AgentState) -> String {
    format!("<green>My Limbs: [{:?}]", state.limb_damage)
}

fn format_target_limbs(state: &AgentState) -> String {
    format!("<red>Target Limbs: [{:?}]", state.limb_damage)
}

fn format_combat_action(combat_action: &CombatAction) -> Vec<String> {
    let lines = vec![format!(
        "{} ={}= @ {}",
        combat_action.caster, combat_action.skill, combat_action.target
    )];
    lines
}

pub fn get_battle_stats(topper: &mut Topper) -> BattleStats {
    let mut lines = Vec::new();
    let my_stats = PlayerStats::for_player(&topper.timeline.state.get_me());
    let target_stats = if let Some(target) = &topper.target {
        Some(PlayerStats::for_player(
            &topper.timeline.state.get_agent(target),
        ))
    } else {
        None
    };
    let mut lines_available = 16;
    lines.push(format_self_limbs(&topper.timeline.state.get_me()));
    if let Some(target) = &topper.target {
        let target = topper.timeline.state.get_agent(target);
        lines.push(format_target_limbs(&target));
    }
    for timeslice in topper.timeline.slices.iter().rev() {
        for observation in timeslice.observations.iter().rev() {
            if lines_available <= 0 {
                break;
            }
            if let Observation::CombatAction(combat_action) = observation {
                if let Some(who) = &topper.target {
                    if !who.eq_ignore_ascii_case(&combat_action.target)
                        && !who.eq_ignore_ascii_case(&combat_action.caster)
                        && !who.eq_ignore_ascii_case(&topper.timeline.who_am_i())
                    {
                        continue;
                    }
                }
                let new_lines = format_combat_action(combat_action);
                for line in new_lines.iter().rev() {
                    if lines_available > 0 {
                        lines.push(line.to_string());
                        lines_available -= 1;
                    }
                }
            }
            if None == topper.target {
                if let Observation::SimpleCureAction(simple_cure) = observation {
                    lines.push(format!(
                        "{} <= {:?}",
                        simple_cure.caster, simple_cure.cure_type
                    ));
                    lines_available -= 1;
                }
            }
        }
    }
    BattleStats {
        feed: lines,
        my_stats,
        target_stats,
    }
}
