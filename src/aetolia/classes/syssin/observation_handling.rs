use super::*;
use crate::aetolia::alpha_beta::ActionPlanner;
use crate::aetolia::classes::*;
use crate::aetolia::curatives::get_cure_depth;
use crate::aetolia::observables::*;
use crate::aetolia::timeline::*;
use crate::aetolia::timeline::*;
use crate::aetolia::topper::*;
use crate::aetolia::types::*;
use regex::Regex;

lazy_static! {
    static ref SUGGESTION: Regex = Regex::new(r"suggest (\w+) ([^;%]+)").unwrap();
    static ref FLAY: Regex = Regex::new(r"flay (\w+)($|;;| (\w+) ?(\w+)?$)").unwrap();
    static ref TRIGGER: Regex = Regex::new(r"trigger (.*)").unwrap();
    static ref ACTION: Regex = Regex::new(r"action (.*)").unwrap();
    pub static ref ERADICATE_PLAN: Regex = Regex::new(r"eradicate (((\w+),?)+)").unwrap();
}

lazy_static! {
    pub static ref BEDAZZLE_AFFS: Vec<FType> = vec![
        FType::Vomiting,
        FType::Stuttering,
        FType::BlurryVision,
        FType::Dizziness,
        FType::Weariness,
        FType::Laxity,
    ];
}

lazy_static! {
    static ref FLAY_ORDER: Vec<FType> = vec![
        FType::Shielded,
        FType::Rebounding,
        FType::Fangbarrier,
        FType::Speed,
        FType::Cloak,
    ];
}

pub fn infer_flay_target(
    name: &String,
    agent_states: &mut AetTimelineState,
) -> Option<(FType, String)> {
    if let Some(flay) = agent_states.get_player_hint(name, &"flay".into()) {
        if let Some(captures) = FLAY.captures(&flay) {
            if let Some(def_name) = captures.get(3) {
                Some((
                    FType::from_name(&def_name.as_str().to_string()).unwrap_or(FType::Rebounding),
                    captures
                        .get(4)
                        .map(|venom_match| venom_match.as_str())
                        .unwrap_or("")
                        .to_string(),
                ))
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

pub fn infer_suggestion(name: &String, agent_states: &mut AetTimelineState) -> Hypnosis {
    if let Some(suggestion) = agent_states.get_player_hint(name, &"suggestion".into()) {
        if let Some(captures) = ACTION.captures(&suggestion) {
            Hypnosis::Action(captures.get(1).unwrap().as_str().to_string())
        } else if let Some(captures) = TRIGGER.captures(&suggestion) {
            Hypnosis::Trigger(captures.get(1).unwrap().as_str().to_string())
        } else if suggestion.eq("bulimia") {
            Hypnosis::Bulimia
        } else if suggestion.eq("eradicate") {
            Hypnosis::Eradicate
        } else if let Some(aff) = FType::from_name(&suggestion) {
            println!("Good {:?}", aff);
            Hypnosis::Aff(aff)
        } else {
            println!("Bad {}", suggestion);
            Hypnosis::Aff(FType::Impatience)
        }
    } else {
        println!("Bad, no hint");
        Hypnosis::Aff(FType::Impatience)
    }
}

pub fn handle_sent(command: &String, agent_states: &mut AetTimelineState) {
    if let Some(captures) = SUGGESTION.captures(command) {
        agent_states.add_player_hint(
            captures.get(1).unwrap().as_str(),
            &"suggestion",
            captures
                .get(2)
                .unwrap()
                .as_str()
                .to_string()
                .to_ascii_lowercase(),
        );
    }
    if let Some(captures) = FLAY.captures(command) {
        agent_states.add_player_hint(
            captures.get(1).unwrap().as_str(),
            &"flay",
            captures.get(0).unwrap().as_str().to_string(),
        );
    }
}

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut AetTimelineState,
    _before: &Vec<AetObservation>,
    after: &Vec<AetObservation>,
) -> Result<(), String> {
    match combat_action.skill.as_ref() {
        "Doublestab" => {
            let observations = after.clone();
            let first_person = combat_action.caster.eq(&agent_states.me);
            let hints =
                agent_states.get_player_hint(&combat_action.caster, &"CALLED_VENOMS".to_string());
            for_agent_closure(
                agent_states,
                &combat_action.caster,
                Box::new(move |me| {
                    apply_or_infer_balance(me, (BType::Balance, 2.65), &observations);
                }),
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
        "Slit" => {
            let observations = after.clone();
            let first_person = combat_action.caster.eq(&agent_states.me);
            let hints =
                agent_states.get_player_hint(&combat_action.caster, &"CALLED_VENOMS".to_string());
            for_agent_closure(
                agent_states,
                &combat_action.caster,
                Box::new(move |me| {
                    apply_or_infer_balance(me, (BType::Balance, 1.88), &observations);
                }),
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
        "Shrugging" => {
            let observations = after.clone();
            for_agent_closure(
                agent_states,
                &combat_action.caster,
                Box::new(move |me| {
                    apply_or_infer_balance(me, (BType::ClassCure1, 20.0), &observations);
                }),
            );
        }
        "Bite" => {
            let observations = after.clone();
            let venom = combat_action.annotation.clone();
            if let Some(AetObservation::Parry(who, _what)) = observations.get(1) {
                if !who.eq(&combat_action.target) {
                    for_agent_closure(
                        agent_states,
                        &combat_action.target,
                        Box::new(move |you| {
                            apply_venom(you, &venom, false);
                        }),
                    );
                }
            } else if let Some(AetObservation::Absorbed(who, _what)) = observations.get(1) {
                if !who.eq(&combat_action.target) {
                    for_agent_closure(
                        agent_states,
                        &combat_action.target,
                        Box::new(move |you| {
                            apply_venom(you, &venom, false);
                        }),
                    );
                }
            } else if let Some(AetObservation::PurgeVenom(who, _what)) = observations.get(1) {
                if !who.eq(&combat_action.target) {
                    for_agent_closure(
                        agent_states,
                        &combat_action.target,
                        Box::new(move |you| {
                            apply_venom(you, &venom, false);
                        }),
                    );
                }
            } else {
                for_agent_closure(
                    agent_states,
                    &combat_action.target,
                    Box::new(move |you| {
                        apply_venom(you, &venom, false);
                    }),
                );
            }
            for_agent_closure(
                agent_states,
                &combat_action.caster,
                Box::new(move |me| {
                    apply_or_infer_balance(me, (BType::Balance, 1.9), &observations);
                }),
            );
        }
        "Sleight" => {
            match combat_action.annotation.as_ref() {
                "Void" => {
                    for_agent(agent_states, &combat_action.target, |you| {
                        you.set_flag(FType::Void, true);
                    });
                }
                _ => {}
            }
            let observations = after.clone();
            for_agent_closure(
                agent_states,
                &combat_action.caster,
                Box::new(move |me| {
                    apply_or_infer_balance(me, (BType::Secondary, 6.0), &observations);
                }),
            );
        }
        "Marks" => {
            let observations = after.clone();
            for_agent_closure(
                agent_states,
                &combat_action.caster,
                Box::new(move |me| {
                    apply_or_infer_balance(me, (BType::Balance, 3.0), &observations);
                    apply_or_infer_balance(me, (BType::Secondary, 3.0), &observations);
                }),
            );
            let mark = match combat_action.annotation.as_ref() {
                "Numbness" => FType::NumbedSkin,
                "Fatigue" => FType::MentalFatigue,
                _ => FType::Thorns,
            };
            for_agent_closure(
                agent_states,
                &combat_action.target,
                Box::new(move |you| {
                    you.set_flag(mark, true);
                }),
            );
        }
        "Flay" => {
            let targetless = infer_flay_target(&combat_action.target, agent_states).is_none();
            let observations = after.clone();
            let first_person = combat_action.caster.eq(&agent_states.me);
            let hints =
                agent_states.get_player_hint(&combat_action.caster, &"CALLED_VENOMS".to_string());
            let annotation = combat_action.annotation.clone();
            for_agent_closure(
                agent_states,
                &combat_action.caster,
                Box::new(move |me| {
                    apply_or_infer_balance(me, (BType::Balance, 1.9), &observations);
                }),
            );
            let observations = after.clone();
            if combat_action.annotation.eq(&"rebounding") || combat_action.annotation.eq(&"shield")
            {
                apply_weapon_hits(
                    agent_states,
                    &combat_action.caster,
                    &combat_action.target,
                    after,
                    first_person,
                    &hints,
                );
            }
            for_agent_closure(
                agent_states,
                &combat_action.target,
                Box::new(move |you| {
                    match annotation.as_ref() {
                        "rebounding" => {
                            you.set_flag(FType::Rebounding, false);
                        }
                        "failure-rebounding" => {
                            you.set_flag(FType::Rebounding, false);
                        }
                        "fangbarrier" => {
                            you.set_flag(FType::Fangbarrier, false);
                        }
                        "failure-fangbarrier" => {
                            you.set_flag(FType::Fangbarrier, false);
                        }
                        "shield" => {
                            you.set_flag(FType::Shielded, false);
                        }
                        "failure-shield" => {
                            you.set_flag(FType::Shielded, false);
                        }
                        "speed" => {
                            you.set_flag(FType::Speed, false);
                        }
                        "cloak" => {
                            you.set_flag(FType::Cloak, false);
                        }
                        _ => {}
                    }
                    if targetless {
                        remove_through(
                            you,
                            match annotation.as_ref() {
                                "rebounding" => FType::Rebounding,
                                "fangbarrier" => FType::Fangbarrier,
                                "shield" => FType::Shielded,
                                "speed" => FType::Speed,
                                "cloak" => FType::Cloak,
                                _ => FType::Cloak,
                            },
                            &FLAY_ORDER.to_vec(),
                        )
                    }
                }),
            );
        }
        "Hypnotise" => {
            for_agent(agent_states, &combat_action.target, |you| {
                you.hypno_state.hypnotize();
            });
        }
        "Desway" => {
            for_agent(agent_states, &combat_action.target, |you| {
                you.hypno_state.desway();
            });
        }
        "Seal" => {
            let observations = after.clone();
            for_agent_closure(
                agent_states,
                &combat_action.caster,
                Box::new(move |me| {
                    apply_or_infer_balance(me, (BType::Equil, 3.0), &observations);
                }),
            );
            for_agent(agent_states, &combat_action.target, |you| {
                you.hypno_state.seal(3.0);
            });
        }
        "Suggest" => {
            let observations = after.clone();
            for_agent_closure(
                agent_states,
                &combat_action.caster,
                Box::new(move |me| {
                    apply_or_infer_balance(me, (BType::Equil, 2.25), &observations);
                }),
            );
            let suggestion = infer_suggestion(&combat_action.target, agent_states);
            for_agent_closure(
                agent_states,
                &combat_action.target,
                Box::new(move |you| {
                    you.hypno_state.push_suggestion(suggestion.clone());
                }),
            );
        }
        "Fizzle" => {
            for_agent(agent_states, &combat_action.target, |you| {
                you.hypno_state.pop_suggestion(false);
            });
        }
        "Snap" => {
            if let Some(target) =
                agent_states.get_player_hint(&combat_action.caster, &"snap".into())
            {
                for_agent(agent_states, &combat_action.target, |you| {
                    if you.hypno_state.sealed.is_some() {
                        you.hypno_state.activate();
                    }
                });
            } else if !combat_action.target.eq(&"".to_string()) {
                for_agent(agent_states, &combat_action.target, |you| {
                    if you.hypno_state.sealed.is_some() {
                        you.hypno_state.activate();
                    }
                });
            }
        }
        "Bedazzle" => {
            let observations = after.clone();
            for_agent_closure(
                agent_states,
                &combat_action.target,
                Box::new(move |you| {
                    apply_or_infer_random_afflictions(you, &observations);
                }),
            );
            let observations = after.clone();
            for_agent_closure(
                agent_states,
                &combat_action.caster,
                Box::new(move |me| {
                    apply_or_infer_balance(me, (BType::Balance, 2.75), &observations);
                }),
            );
        }
        "Fire" => {
            let observations = after.clone();
            for_agent_closure(
                agent_states,
                &combat_action.target,
                Box::new(move |you| {
                    apply_or_infer_suggestion(you, &observations);
                }),
            );
        }
        _ => {}
    }
    Ok(())
}
