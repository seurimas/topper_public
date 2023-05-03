use crate::timeline::*;
use crate::types::*;

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut AetTimelineState,
    _before: &Vec<AetObservation>,
    after: &Vec<AetObservation>,
) -> Result<(), String> {
    let perspective = agent_states.get_perspective(&combat_action);
    match combat_action.category.as_ref() {
        "Mentis" => {
            if let Some(aff) = FType::from_name(&combat_action.skill) {
                attack_afflictions(agent_states, &combat_action.target, vec![aff], after);
            } else {
                match combat_action.skill.as_ref() {
                    "Mesmerize" => {
                        if perspective != Perspective::Target {
                            for_agent(agent_states, &combat_action.target, &|me| {
                                if me.is(FType::Blindness) {
                                    me.set_flag(FType::Blindness, false);
                                } else {
                                    me.set_flag(FType::WritheTransfix, true);
                                }
                            });
                        }
                    }
                    _ => {}
                }
            }
        }
        "Corpus" => match combat_action.skill.as_ref() {
            "Purify" => {
                let observations = after.clone();
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &move |me: &mut AgentState| {
                        apply_or_infer_balance(me, (BType::ClassCure1, 20.0), &observations);
                    },
                );
            }
            "Gash" => {
                let observations = after.clone();
                let first_person = combat_action.caster.eq(&agent_states.me);
                let hints = agent_states
                    .get_player_hint(&combat_action.caster, &"CALLED_VENOMS".to_string());
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &move |me: &mut AgentState| {
                        apply_or_infer_balance(me, (BType::Balance, 3.), &observations);
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
            _ => {}
        },
        "Sanguis" => match combat_action.skill.as_ref() {
            "Trepidation" => {
                if let Some(aff) = FType::from_name(&combat_action.annotation) {
                    attack_afflictions(agent_states, &combat_action.caster, vec![aff], after);
                }
            }
            "Curse" => {
                attack_afflictions(
                    agent_states,
                    &combat_action.target,
                    vec![FType::BloodCurse],
                    after,
                );
            }
            "Spew" => for_agent(agent_states, &combat_action.target, &|me| {
                me.set_flag(FType::Blindness, false);
                me.set_flag(FType::Deafness, false);
            }),
            "Poison" => {
                attack_afflictions(
                    agent_states,
                    &combat_action.target,
                    vec![FType::BloodPoison],
                    after,
                );
            }
            _ => {}
        },
        _ => {}
    }
    Ok(())
}
