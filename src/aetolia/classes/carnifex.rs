use crate::aetolia::timeline::*;
use crate::aetolia::types::*;

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut AetTimelineState,
    _before: &Vec<AetObservation>,
    after: &Vec<AetObservation>,
) -> Result<(), String> {
    match combat_action.skill.as_ref() {
        "Fitness" => {
            let observations = after.clone();
            for_agent_closure(
                agent_states,
                &combat_action.caster,
                Box::new(move |me| {
                    apply_or_infer_cures(me, vec![FType::Asthma], &observations);
                    apply_or_infer_balance(me, (BType::ClassCure1, 12.0), &observations);
                }),
            );
        }
        "Shield" => {
            let observations = after.clone();
            for_agent_closure(
                agent_states,
                &combat_action.caster,
                Box::new(move |me| {
                    me.set_flag(FType::Shielded, true);
                    apply_or_infer_balance(me, (BType::Equil, 4.0), &observations);
                }),
            );
        }
        _ => {}
    }
    Ok(())
}
