use crate::curatives::top_missing_aff;
use crate::curatives::SafetyAlert;
use crate::curatives::MENTAL_AFFLICTIONS;
use crate::curatives::PHYSICAL_AFFLICTIONS;
use crate::timeline::*;
use crate::types::*;

lazy_static! {
    pub static ref CIRCLE_AFFS: Vec<FType> = vec![
        FType::Merciful,
        FType::Masochism,
        FType::Berserking,
        FType::Recklessness,
    ];
    pub static ref SQUARE_AFFS: Vec<FType> = vec![
        FType::Dizziness,
        FType::Faintness,
        FType::Epilepsy,
        FType::Shyness,
    ];
    pub static ref TRIANGLE_AFFS: Vec<FType> = vec![
        FType::Laxity,
        FType::LoversEffect,
        FType::Peace,
        FType::Magnanimity,
    ];
}

const MADNESS_CD: f32 = 7.5;
const CONJOIN_CD: f32 = 20.0;
const CONJOIN_DUR: f32 = 13.0;
const AFTERIMAGE_DELAY: f32 = 5.125;
const LEMNISCATE_GRACE: f32 = 9.0;
const LEMNISCATE_COUNT: usize = 15;
const SEALING_DUR: f32 = 31.0;
const HEX_DELAY: f32 = 6.25;

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut AetTimelineState,
    _before: &Vec<AetObservation>,
    after: &Vec<AetObservation>,
) -> Result<(), String> {
    match combat_action.skill.as_ref() {
        "Circle" | "Square" | "Triangle" => {
            let affs = if "Circle".eq(&combat_action.skill) {
                CIRCLE_AFFS.clone()
            } else if "Square".eq(&combat_action.skill) {
                SQUARE_AFFS.clone()
            } else {
                TRIANGLE_AFFS.clone()
            };
            let afflicted = match after.get(1) {
                Some(AetObservation::Afflicted(affliction)) => FType::from_name(affliction),
                _ => Default::default(),
            };
            let second_person = combat_action.target.eq(&agent_states.me);
            for_agent(agent_states, &combat_action.target, &move |you| {
                if let Some(afflicted) = afflicted {
                    for aff in affs.iter() {
                        if afflicted != *aff {
                            if second_person && !you.is(*aff) {
                                you.observe_flag(*aff, true);
                                you.hidden_state.add_guess(*aff);
                            }
                        } else {
                            you.toggle_flag(*aff, true);
                            return;
                        }
                    }
                } else if let Some(aff) = top_missing_aff(you, &affs) {
                    you.set_flag(aff, true);
                }
            });
        }
        "Shape" => {
            let observations = after.clone();
            let perspective = agent_states.get_perspective(&combat_action);
            if perspective != Perspective::Bystander {
                for_agent_uncertain_closure(
                    agent_states,
                    &combat_action.target,
                    Box::new(move |you| {
                        let mut possible_affs = Vec::new();
                        if let Some(circle_aff) = top_missing_aff(you, &CIRCLE_AFFS.to_vec()) {
                            possible_affs.push(circle_aff);
                        }
                        if let Some(square_aff) = top_missing_aff(you, &SQUARE_AFFS.to_vec()) {
                            possible_affs.push(square_aff);
                        }
                        if let Some(triangle_aff) = top_missing_aff(you, &TRIANGLE_AFFS.to_vec()) {
                            possible_affs.push(triangle_aff);
                        }
                        apply_or_infer_random_afflictions(
                            you,
                            &observations,
                            perspective,
                            Some((1, possible_affs)),
                        )
                    }),
                );
            }
        }
        _ => {}
    }
    Ok(())
}

pub fn get_archivist_alerts(agent: &AgentState) -> Vec<SafetyAlert> {
    if agent.affs_count(&MENTAL_AFFLICTIONS.to_vec()) >= 3 {
        if agent.affs_count(&PHYSICAL_AFFLICTIONS.to_vec()) >= 2 {
            let mut physical_affs = vec![];
            for aff in PHYSICAL_AFFLICTIONS.iter() {
                if agent.is(*aff) {
                    physical_affs.push(*aff);
                }
            }
            return vec![SafetyAlert::InstakillThreat(physical_affs)];
        }
    }
    vec![]
}
