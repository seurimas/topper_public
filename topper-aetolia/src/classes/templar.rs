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
        "Vorpal" => {
            let observations = after.clone();
            let first_person = combat_action.caster.eq(&agent_states.me);
            let hints =
                agent_states.get_player_hint(&combat_action.caster, &"CALLED_VENOMS".to_string());
            let hints = hints.map(|hints| {
                if let Some(captures) = CALLED_VENOMS_THREE.captures(&hints) {
                    agent_states.add_player_hint(
                        &combat_action.caster,
                        &"CALLED_VENOMS".to_string(),
                        format!(
                            "{}, {}",
                            captures.get(2).unwrap().as_str().to_string(),
                            captures.get(3).unwrap().as_str().to_string()
                        ),
                    );
                    captures.get(1).unwrap().as_str().to_string()
                } else {
                    hints
                }
            });
            for_agent(
                agent_states,
                &combat_action.caster,
                &move |me: &mut AgentState| {
                    apply_or_infer_balance(me, (BType::Balance, 2.65), &observations);
                },
            );
            apply_weapon_hits(
                agent_states,
                &combat_action.caster,
                &combat_action.target,
                after,
                first_person,
                &hints,
            );
        }
        "Duality" => {
            let observations = after.clone();
            let first_person = combat_action.caster.eq(&agent_states.me);
            let hints =
                agent_states.get_player_hint(&combat_action.caster, &"CALLED_VENOMS".to_string());
            for_agent(
                agent_states,
                &combat_action.caster,
                &move |me: &mut AgentState| {
                    apply_or_infer_balance(me, (BType::Balance, 2.65), &observations);
                },
            );
            apply_weapon_hits(
                agent_states,
                &combat_action.caster,
                &combat_action.target,
                after,
                first_person,
                &hints,
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
