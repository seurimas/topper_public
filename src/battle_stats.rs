use crate::io::Topper;
use crate::timeline::*;
use crate::types::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct BattleStats {
    pub lines: Vec<String>,
}

fn format_self_state(state: &AgentState) -> String {
    let locked = if let Some(duration) = state.lock_duration() {
        format!("<magenta>[LOCKED FOR {}] <green>", duration)
    } else {
        "".into()
    };
    format!(
        "<green>My Afflictions: {}{} {}",
        locked, state.flags, state.limb_damage
    )
}

fn format_target_state(state: &AgentState) -> String {
    let locked = if let Some(duration) = state.lock_duration() {
        format!("<magenta>[LOCKED FOR {}] <red>", duration)
    } else {
        "".into()
    };
    format!(
        "<red>Target Afflictions: {}{} {}",
        locked, state.flags, state.limb_damage
    )
}

fn format_self_limbs(state: &AgentState) -> String {
    format!("<green>My Limbs: [{:?}]", state.limb_damage)
}

fn format_target_limbs(state: &AgentState) -> String {
    format!("<red>Target Limbs: [{:?}]", state.limb_damage)
}

fn format_combat_action(combat_action: &CombatAction) -> Vec<String> {
    let mut lines = vec![format!(
        "{} ={}= @ {}",
        combat_action.caster, combat_action.skill, combat_action.target
    )];
    lines
}

pub fn get_battle_stats(topper: &mut Topper) -> BattleStats {
    let mut lines = Vec::new();
    let mut lines_available = 16;
    lines.push(format_self_state(&topper.timeline.state.get_me()));
    if let Some(target) = &topper.target {
        lines.push(format_target_state(
            &topper.timeline.state.get_agent(target),
        ));
    }
    lines.push(format_self_limbs(&topper.timeline.state.get_me()));
    if let Some(target) = &topper.target {
        lines.push(format_target_limbs(
            &topper.timeline.state.get_agent(target),
        ));
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
                let mut new_lines = format_combat_action(combat_action);
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
    BattleStats { lines }
}
