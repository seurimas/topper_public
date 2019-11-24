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

fn format_combat_action(combat_action: &CombatAction) -> String {
    format!(
        "{} ={}= @ {}",
        combat_action.caster, combat_action.skill, combat_action.target
    )
}

pub fn get_battle_stats(topper: &mut Topper) -> BattleStats {
    let mut lines = Vec::new();
    let mut lines_available = 20;
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
                lines.push(format_combat_action(combat_action));
                lines_available -= 1;
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
