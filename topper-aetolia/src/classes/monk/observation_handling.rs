use crate::timeline::*;
use crate::types::*;

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut AetTimelineState,
    _before: &Vec<AetObservation>,
    after: &Vec<AetObservation>,
) -> Result<(), String> {
    match combat_action.skill.as_ref() {
        "Kipup" => {
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.toggle_flag(FType::Fallen, false);
            });
        }
        _ => {}
    }
    Ok(())
}
