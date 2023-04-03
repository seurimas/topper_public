use crate::curatives::remove_in_order;
use crate::curatives::STEROID_ORDER;
use crate::timeline::*;
use crate::types::*;

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut AetTimelineState,
    _before: &Vec<AetObservation>,
    after: &Vec<AetObservation>,
) -> Result<(), String> {
    match combat_action.skill.as_ref() {
        "Shedding" => {
            let observations = after.clone();
            let first_person = agent_states.me.eq(&combat_action.caster);
            for_agent(
                agent_states,
                &combat_action.caster,
                &move |me: &mut AgentState| {
                    apply_or_infer_cures(me, vec![FType::Slickness], &observations, first_person);
                    apply_or_infer_balance(me, (BType::ClassCure1, 12.0), &observations);
                },
            );
        }
        "Rage" => {
            for_agent(agent_states, &combat_action.caster, &|me| {
                remove_in_order(STEROID_ORDER.to_vec(), me);
            });
        }
        _ => {}
    }
    Ok(())
}
