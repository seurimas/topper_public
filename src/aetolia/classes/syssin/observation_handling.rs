use crate::aetolia::alpha_beta::ActionPlanner;
use crate::aetolia::classes::*;
use crate::aetolia::curatives::get_cure_depth;
use crate::aetolia::observables::*;
use crate::aetolia::timeline::*;
use crate::aetolia::topper::*;
use crate::aetolia::types::*;
use regex::Regex;
use super::*;
use crate::aetolia::timeline::*;

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
            let hints = agent_states.get_player_hint(&combat_action.caster, &"CALLED_VENOMS".to_string());
            for_agent_pair_closure(agent_states, &combat_action.caster, &combat_action.target,
                Box::new(move |me, you| {
                apply_weapon_hits(
                    me,
                    you,
                    &observations,
                    first_person,
                    &hints,
                );
                apply_or_infer_balance(me, (BType::Balance, 2.65), &observations);
            }));
        }
        "Slit" => {
            let observations = after.clone();
            let first_person = combat_action.caster.eq(&agent_states.me);
            let hints = agent_states.get_player_hint(&combat_action.caster, &"CALLED_VENOMS".to_string());
            for_agent_pair_closure(agent_states, &combat_action.caster, &combat_action.target,
                Box::new(move |me, you| {
                apply_weapon_hits(
                    me,
                    you,
                    &observations,
                    first_person,
                    &hints,
                );
                apply_or_infer_balance(me, (BType::Balance, 1.88), &observations);
            }));
        }
        "Shrugging" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            apply_or_infer_balance(&mut me, (BType::ClassCure1, 20.0), after);
            agent_states.set_agent(&combat_action.caster, me);
        }
        "Bite" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            let mut you = agent_states.get_agent(&combat_action.target);
            me.set_balance(BType::Balance, 1.9);
            if let Some(AetObservation::Parry(who, _what)) = after.get(1) {
                if !who.eq(&combat_action.target) {
                    apply_venom(&mut you, &combat_action.annotation, false)?;
                }
            } else if let Some(AetObservation::Absorbed(who, _what)) = after.get(1) {
                if !who.eq(&combat_action.target) {
                    apply_venom(&mut you, &combat_action.annotation, false)?;
                }
            } else if let Some(AetObservation::PurgeVenom(who, _what)) = after.get(1) {
                if !who.eq(&combat_action.target) {
                    apply_venom(&mut you, &combat_action.annotation, false)?;
                }
            } else {
                apply_venom(&mut you, &combat_action.annotation, false)?;
            }
            apply_or_infer_balance(&mut me, (BType::Balance, 1.9), after);
            agent_states.set_agent(&combat_action.caster, me);
            agent_states.set_agent(&combat_action.target, you);
        }
        "Sleight" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            let mut you = agent_states.get_agent(&combat_action.target);
            match combat_action.annotation.as_ref() {
                "Void" => {
                    apply_or_infer_balance(&mut me, (BType::Secondary, 6.0), after);
                    you.set_flag(FType::Void, true);
                }
                _ => {}
            }
            agent_states.set_agent(&combat_action.caster, me);
            agent_states.set_agent(&combat_action.target, you);
        }
        "Marks" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            let mut you = agent_states.get_agent(&combat_action.target);
            match combat_action.annotation.as_ref() {
                "Numbness" => {
                    apply_or_infer_balance(&mut me, (BType::Balance, 3.0), after);
                    apply_or_infer_balance(&mut me, (BType::Secondary, 3.0), after);
                    you.set_flag(FType::NumbedSkin, true);
                }
                "Fatigue" => {
                    apply_or_infer_balance(&mut me, (BType::Balance, 3.0), after);
                    apply_or_infer_balance(&mut me, (BType::Secondary, 3.0), after);
                    you.set_flag(FType::MentalFatigue, true);
                }
                _ => {}
            }
            agent_states.set_agent(&combat_action.caster, me);
            agent_states.set_agent(&combat_action.target, you);
        }
        "Flay" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            let mut you = agent_states.get_agent(&combat_action.target);
            me.set_balance(BType::Balance, 1.9);
            apply_or_infer_balance(&mut me, (BType::Balance, 1.9), after);
            if combat_action.annotation.eq(&"rebounding") || combat_action.annotation.eq(&"shield")
            {
                apply_weapon_hits(
                    &mut me,
                    &mut you,
                    after,
                    combat_action.caster.eq(&agent_states.me),
                    &agent_states
                        .get_player_hint(&combat_action.caster, &"CALLED_VENOMS".to_string()),
                )?;
            }
            match combat_action.annotation.as_ref() {
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
            if infer_flay_target(&combat_action.target, agent_states).is_none() {
                remove_through(
                    &mut you,
                    match combat_action.annotation.as_ref() {
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
            agent_states.set_agent(&combat_action.caster, me);
            agent_states.set_agent(&combat_action.target, you);
        }
        "Hypnotise" => {
            let mut you = agent_states.get_agent(&combat_action.target);
            you.hypno_state.hypnotize();
            agent_states.set_agent(&combat_action.target, you);
        }
        "Desway" => {
            let mut you = agent_states.get_agent(&combat_action.target);
            you.hypno_state.desway();
            agent_states.set_agent(&combat_action.target, you);
        }
        "Seal" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            let mut you = agent_states.get_agent(&combat_action.target);
            apply_or_infer_balance(&mut me, (BType::Equil, 3.0), after);
            you.hypno_state.seal(3.0);
            agent_states.set_agent(&combat_action.caster, me);
            agent_states.set_agent(&combat_action.target, you);
        }
        "Suggest" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            let mut you = agent_states.get_agent(&combat_action.target);
            apply_or_infer_balance(&mut me, (BType::Equil, 2.25), after);
            you.hypno_state
                .push_suggestion(infer_suggestion(&combat_action.target, agent_states));
            agent_states.set_agent(&combat_action.caster, me);
            agent_states.set_agent(&combat_action.target, you);
        }
        "Fizzle" => {
            let mut me = agent_states.get_agent(&combat_action.target);
            me.hypno_state.pop_suggestion(false);
            agent_states.set_agent(&combat_action.target, me);
        }
        "Snap" => {
            if let Some(target) =
                agent_states.get_player_hint(&combat_action.caster, &"snap".into())
            {
                let mut you = agent_states.get_agent(&target);
                if you.hypno_state.sealed.is_some() {
                    you.hypno_state.activate();
                }
                agent_states.set_agent(&target, you);
            } else if !combat_action.target.eq(&"".to_string()) {
                let mut you = agent_states.get_agent(&combat_action.target);
                if you.hypno_state.sealed.is_some() {
                    you.hypno_state.activate();
                }
                agent_states.set_agent(&combat_action.target, you);
            }
        }
        "Bedazzle" => {
            let mut you = agent_states.get_agent(&combat_action.target);
            apply_or_infer_random_afflictions(&mut you, after)?;
            agent_states.set_agent(&combat_action.target, you);
            let mut me = agent_states.get_agent(&combat_action.caster);
            apply_or_infer_balance(&mut me, (BType::Balance, 2.75), after);
            agent_states.set_agent(&combat_action.caster, me);
        }
        "Fire" => {
            let mut you = agent_states.get_agent(&combat_action.target);
            apply_or_infer_suggestion(&mut you, after)?;
            agent_states.set_agent(&combat_action.target, you)
        }
        _ => {}
    }
    Ok(())
}