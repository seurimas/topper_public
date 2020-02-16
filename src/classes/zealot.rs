use crate::timeline::*;
use crate::types::*;

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut TimelineState,
    _before: &Vec<Observation>,
    after: &Vec<Observation>,
) -> Result<(), String> {
    match combat_action.skill.as_ref() {
        "Pendulum" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            let mut you = agent_states.get_agent(&combat_action.target);
            apply_or_infer_balance(&mut me, (BType::Equil, 3.0), after);
            you.rotate_limbs(combat_action.annotation == "anti-clockwise");
            agent_states.set_agent(&combat_action.caster, me);
            agent_states.set_agent(&combat_action.target, you);
        }
        _ => {}
    }
    Ok(())
}
