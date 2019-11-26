use crate::io::Topper;
use crate::timeline::*;
use crate::types::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct BattleStats {
    pub lines: Vec<String>,
}

fn format_self_state(state: &AgentState) -> String {
    format!("My Afflictions: {}", state.flags)
}

fn format_target_state(state: &AgentState) -> String {
    format!("Target Afflictions: {}", state.flags)
}

fn format_combat_action(combat_action: &CombatAction) -> Vec<String> {
    let mut lines = vec![format!(
        "{} ={}= @ {}",
        combat_action.caster, combat_action.skill, combat_action.target
    )];
    let mut line2 = "".to_string();
    for observe in combat_action.associated.iter() {
        match observe {
            Observation::Devenoms(venom) => {
                line2 = format!("{} *{}*", line2, venom);
            }
            _ => {}
        }
    }
    if line2.len() > 0 {
        lines.push(line2);
    }
    lines
}

pub fn get_battle_stats(topper: &mut Topper) -> BattleStats {
    let mut lines = Vec::new();
    let mut lines_available = 16;
    lines.push(format_self_state(
        &topper.timeline.state.get_agent(&topper.me),
    ));
    if let Some(target) = &topper.target {
        lines.push(format_target_state(
            &topper.timeline.state.get_agent(target),
        ));
    }
    for timeslice in topper.timeline.slices.iter().rev() {
        for incident in timeslice.incidents.iter().rev() {
            if lines_available <= 0 {
                break;
            }
            if let Incident::CombatAction(combat_action) = incident {
                let mut new_lines = format_combat_action(combat_action);
                for line in new_lines.iter().rev() {
                    if lines_available > 0 {
                        lines.push(line.to_string());
                        lines_available -= 1;
                    }
                }
            }
            if let Incident::SimpleCureAction(simple_cure) = incident {
                lines.push(format!(
                    "{} <= {:?}",
                    simple_cure.caster, simple_cure.cure_type
                ));
                lines_available -= 1;
            }
        }
    }
    BattleStats { lines }
}
