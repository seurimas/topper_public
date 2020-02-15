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
    before: &Vec<Observation>,
    after: &Vec<Observation>,
) -> Result<(), String> {
    match combat_action.skill.as_ref() {
        "Fitness" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            apply_or_infer_cures(&mut me, vec![FType::Asthma], after)?;
            apply_or_infer_balance(&mut me, (BType::ClassCure1, 12.0), after);
            agent_states.set_agent(&combat_action.caster, me);
        }
        _ => {}
    }
    Ok(())
}
