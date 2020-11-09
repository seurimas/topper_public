use crate::timeline::aetolia::*;
use crate::types::*;

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut AetTimelineState,
    _before: &Vec<AetObservation>,
    after: &Vec<AetObservation>,
) -> Result<(), String> {
    match combat_action.skill.as_ref() {
        "Fitness" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            apply_or_infer_cures(&mut me, vec![FType::Asthma], after)?;
            apply_or_infer_balance(&mut me, (BType::ClassCure1, 12.0), after);
            agent_states.set_agent(&combat_action.caster, me);
        }
        "Shield" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            me.set_flag(FType::Shielded, true);
            apply_or_infer_balance(&mut me, (BType::Equil, 4.0), after);
            agent_states.set_agent(&combat_action.caster, me);
        }
        _ => {}
    }
    Ok(())
}
