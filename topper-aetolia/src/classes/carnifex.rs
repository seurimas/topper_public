use crate::timeline::*;
use crate::types::*;

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut AetTimelineState,
    _before: &Vec<AetObservation>,
    after: &Vec<AetObservation>,
) -> Result<(), String> {
    match combat_action.skill.as_ref() {
        "Shield" => {
            let observations = after.clone();
            for_agent(
                agent_states,
                &combat_action.caster,
                &move |me: &mut AgentState| {
                    me.set_flag(FType::Shielded, true);
                    apply_or_infer_balance(me, (BType::Equil, 4.0), &observations);
                },
            );
        }
        _ => {}
    }
    Ok(())
}
