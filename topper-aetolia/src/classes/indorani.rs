use crate::timeline::*;
use crate::types::*;

lazy_static! {
    pub static ref SUN_AFFS: Vec<FType> = vec![
        FType::Paresis,
        FType::Clumsiness,
        FType::Asthma,
        FType::Vomiting,
        FType::Lethargy,
        FType::Sensitivity,
        FType::Superstition,
        FType::Hypersomnia,
    ];
    pub static ref MOON_AFFS: Vec<FType> = vec![
        FType::Stupidity,
        FType::Confusion,
        FType::Recklessness,
        FType::Impatience,
        FType::Epilepsy,
        FType::Berserking,
        FType::Weariness,
        FType::Anorexia,
    ];
}

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut AetTimelineState,
    _before: &Vec<AetObservation>,
    after: &Vec<AetObservation>,
) -> Result<(), String> {
    match combat_action.skill.as_ref() {
        "Chilled" => {
            for_agent(agent_states, &combat_action.caster, &|you| {
                if you.is(FType::Insulation) {
                    you.set_flag(FType::Insulation, false);
                } else if !you.is(FType::Shivering) {
                    you.set_flag(FType::Shivering, true);
                } else if !you.is(FType::Frozen) {
                    you.set_flag(FType::Frozen, true);
                }
            });
        }
        "Croned" => {
            let broken_limb = match combat_action.annotation.as_ref() {
                "left arm" => FType::LeftArmCrippled,
                "right arm" => FType::RightArmCrippled,
                "left leg" => FType::LeftLegCrippled,
                "right leg" => FType::RightLegCrippled,
                _ => FType::SIZE, // I don't want to panic
            };
            attack_afflictions(
                agent_states,
                &combat_action.caster,
                vec![broken_limb],
                after,
            );
        }
        "Sun" | "Moon" => {
            if !combat_action.annotation.eq("dodge") {
                let observations = after.clone();
                let perspective = agent_states.get_perspective(&combat_action);
                let sun = combat_action.skill.eq("Sun");
                if perspective != Perspective::Bystander {
                    for_agent_uncertain_closure(
                        agent_states,
                        &combat_action.target,
                        Box::new(move |you| {
                            apply_or_infer_random_afflictions(
                                you,
                                &observations,
                                perspective,
                                Some((
                                    1,
                                    (if sun {
                                        SUN_AFFS.iter()
                                    } else {
                                        MOON_AFFS.iter()
                                    })
                                    .filter(|aff| !you.is(**aff))
                                    .map(|aff| *aff)
                                    .collect(),
                                )),
                            )
                        }),
                    );
                } else {
                    let hints = agent_states
                        .get_player_hint(&combat_action.caster, &"CALLED_VENOMS".to_string())
                        .unwrap_or("".to_string());
                    for_agent(agent_states, &combat_action.target, &move |you| {
                        let mut afflictions = Vec::new();
                        if let Some(captures) = CALLED_VENOMS_TWO.captures(&hints) {
                            afflictions.push(captures.get(1).unwrap().as_str().to_string());
                            afflictions.push(captures.get(2).unwrap().as_str().to_string());
                        } else if let Some(captures) = CALLED_VENOM.captures(&hints) {
                            afflictions.push(captures.get(1).unwrap().as_str().to_string());
                        }
                        for affliction in afflictions {
                            if let Some(affliction) = FType::from_name(&affliction) {
                                if sun && SUN_AFFS.contains(&affliction) {
                                    you.set_flag(affliction, true);
                                }
                                if !sun && MOON_AFFS.contains(&affliction) {
                                    you.set_flag(affliction, true);
                                }
                            }
                        }
                    })
                }
            }
        }
        _ => {}
    }
    Ok(())
}
