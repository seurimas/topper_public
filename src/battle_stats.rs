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

fn format_target_balances(state: &AgentState) -> String {
    format!(
        "<magenta>Tree: {} <magenta>- Focus: {}",
        if state.balanced(BType::Tree) {
            "<green>Ready".to_string()
        } else {
            format!("{}", state.get_balance(BType::Tree))
        },
        if state.balanced(BType::Focus) {
            "<blue>Ready".to_string()
        } else {
            format!("{}", state.get_balance(BType::Focus))
        },
    )
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

fn push_warnings(lines: &mut Vec<String>, state: &AgentState) {
    if let Some(duration) = state.lock_duration() {
        lines.push(format!("<magenta>YOU ARE LOCKED: {} seconds!", duration));
    }
    if state.is(FType::Paralysis) {
        lines.push(format!("<yellow>You are paralyzed!"));
    }
    if state.is(FType::LeftArmBroken) && state.is(FType::RightArmBroken) {
        lines.push(format!("<red>Your arms are broken!"));
    } else if state.is(FType::LeftArmBroken) {
        lines.push(format!("<red>Your left arm is broken!"));
    }
    if state.is(FType::Prone)
        && (state.is(FType::LeftLegBroken)
            || state.is(FType::RightLegBroken)
            || state.is(FType::Paralysis))
    {
        lines.push(format!("<magenta>YOU ARE FLOORED!"));
    }
}

pub fn get_battle_stats(topper: &mut Topper) -> BattleStats {
    let mut lines = Vec::new();
    let mut lines_available = 16;
    lines.push(format_self_state(&topper.timeline.state.get_me()));
    lines.push(format_self_limbs(&topper.timeline.state.get_me()));
    if let Some(target) = &topper.target {
        let target = topper.timeline.state.get_agent(target);
        lines.push(format_target_state(&target));
        lines.push(format_target_limbs(&target));
        lines.push(format_target_balances(&target));
        if let Some(aff) = target.get_next_hypno_aff() {
            lines.push(format!("<magenta>Next aff: <red>{:?}", aff));
        } else if target.hypnosis_stack.len() > 0 {
            lines.push(format!(
                "<magenta>Stack size: <red>{} {}",
                target.hypnosis_stack.len(),
                if !target.is(FType::Hypnotized) {
                    "SEALED"
                } else {
                    ""
                },
            ));
        }
    }
    push_warnings(&mut lines, &topper.timeline.state.get_me());
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
    BattleStats { lines }
}
