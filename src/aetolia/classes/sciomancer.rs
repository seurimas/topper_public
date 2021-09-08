use crate::aetolia::timeline::*;
use crate::aetolia::types::*;

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut AetTimelineState,
    _before: &Vec<AetObservation>,
    after: &Vec<AetObservation>,
) -> Result<(), String> {
    match combat_action.skill.as_ref() {
        "Grip" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Fallen, FType::Paresis],
                after,
            );
        }
        _ => {}
    }
    Ok(())
}
