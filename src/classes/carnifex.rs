use crate::actions::*;
use crate::alpha_beta::*;
use crate::classes::{get_venoms, AFFLICT_VENOMS};
use crate::curatives::*;
use crate::io::*;
use crate::timeline::*;
use crate::types::*;

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut TimelineState,
) -> Result<(), String> {
    match combat_action.skill.as_ref() {
        "Fitness" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            apply_or_infer_cures(&mut me, vec![FType::Asthma], &combat_action.associated)?;
            agent_states.set_agent(&combat_action.caster, me);
        }
        _ => {
            apply_observations(&combat_action.associated, agent_states)?;
        }
    }
    Ok(())
}
