use crate::classes::{
    get_skill_class, handle_combat_action, handle_sent, Class, DIAGNOSE_FRESHNESS, DIAGNOSE_TIME,
    VENOM_AFFLICTS,
};
use crate::curatives::{
    handle_simple_cure_action, remove_in_order, top_aff, CALORIC_TORSO_ORDER, PILL_CURE_ORDERS,
    PILL_DEFENCES, SALVE_CURE_ORDERS, SMOKE_CURE_ORDERS,
};
use crate::db::AetDatabaseModule;
use crate::non_agent::AetNonAgent;
use crate::timeline::*;
use crate::types::*;
use log::warn;
use regex::Regex;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use topper_core::observations::strip_ansi;
use topper_core::timeline::db::DatabaseModule;
use topper_core::timeline::CType;

#[cfg(test)]
#[path = "./tests/apply_tests.rs"]
mod apply_tests;

pub fn apply_observation(
    timeline: &mut AetTimelineState,
    observation: &AetObservation,
    lines: &Vec<(String, u32)>,
    before: &Vec<AetObservation>,
    after: &Vec<AetObservation>,
    db: Option<&impl AetDatabaseModule>,
) -> Result<(), String> {
    match observation {
        AetObservation::Sent(command) => {
            handle_sent(command, timeline);
        }
        AetObservation::CombatAction(combat_action) => {
            if let (Some(db), Some(class)) = (db, get_skill_class(&combat_action.category)) {
                db.set_class(&combat_action.caster, class);
                for_agent(timeline, &combat_action.caster, &|me| {
                    me.class_state
                        .initialize_for_normalized_class(class.normal());
                });
            }
            handle_combat_action(combat_action, timeline, before, after)?;
        }
        AetObservation::Proc(combat_action) => {
            handle_combat_action(combat_action, timeline, before, after)?;
        }
        AetObservation::SimpleCureAction(simple_cure) => {
            handle_simple_cure_action(simple_cure, timeline, before, after)?;
        }
        AetObservation::DiscernedCure(who, affliction) => {
            timeline.set_flag_for_agent(who, affliction, false)?;
        }
        AetObservation::Cured(affliction) => {
            timeline.set_flag_for_agent(&timeline.me.clone(), affliction, false)?;
        }
        AetObservation::FlameShield(who) => {
            if timeline.borrow_agent(who).get_count(FType::Ablaze) <= 1 {
                timeline.set_flag_for_agent(who, &"Ablaze".to_string(), false)?;
            }
        }
        AetObservation::Undithered => {
            timeline.for_agent(&timeline.me.clone(), &|me: &mut AgentState| {
                me.assume_bard(&|bard| {
                    bard.dithering = 0;
                });
            });
        }
        AetObservation::Afflicted(affliction) => {
            if affliction.eq("sapped_strength") {
                timeline.tick_counter_up_for_agent(&timeline.me.clone(), affliction);
            } else {
                timeline.set_flag_for_agent(&timeline.me.clone(), affliction, true)?;
            }
        }
        AetObservation::Discovered(affliction) => {
            let flag_name = affliction.clone();
            let who_am_i = timeline.me.clone();
            timeline.for_agent(&who_am_i, &move |me: &mut AgentState| {
                if let Some(aff_flag) = FType::from_name(&flag_name) {
                    me.observe_flag(aff_flag, true);
                }
            });
        }
        AetObservation::OtherAfflicted(who, affliction) => {
            if before.len() > 0 {
                if let Some(AetObservation::DiscernedCure(b_who, b_afflict)) =
                    before.get(before.len() - 1)
                {
                    if b_who.eq(who) && b_afflict.eq(affliction) {
                        return Ok(());
                    }
                }
            }
            timeline.set_flag_for_agent(who, affliction, true)?;
        }
        AetObservation::Balance(balance, duration) => {
            let who_am_i = timeline.me.clone();
            for_agent(timeline, &who_am_i, &|me: &mut AgentState| {
                me.set_balance(BType::from_name(&balance), *duration);
            });
        }
        AetObservation::BalanceBack(balance) => {
            let who_am_i = timeline.me.clone();
            for_agent(timeline, &who_am_i, &|me: &mut AgentState| {
                me.set_balance(BType::from_name(&balance), 0.0);
            });
        }
        AetObservation::Dodges(who) => {
            for_agent(timeline, who, &|me: &mut AgentState| {
                me.dodge_state.register_dodge();
            });
        }
        AetObservation::ListStart(list_type, who) => match list_type.as_ref() {
            "Wounds" => {
                let observations = after.clone();
                for_agent(timeline, who, &move |me: &mut AgentState| {
                    for after in observations.iter() {
                        match after {
                            AetObservation::ListItem(list_type, limb, damage, _) => {
                                if list_type.eq("Wounds") {
                                    if let (Ok(limb), Ok(damage)) =
                                        (get_limb_damage(limb), damage.parse::<f32>())
                                    {
                                        me.set_limb_damage(limb, (damage * 100.0) as CType);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                });
            }
            "Diagnose" => {
                if !timeline.is_hint_time_fresh(
                    who,
                    &DIAGNOSE_TIME.to_string(),
                    *DIAGNOSE_FRESHNESS,
                ) {
                    println!("Found diagnose illusion!");
                    return Ok(());
                }
                let afflictions = after
                    .iter()
                    .filter_map(|item| match item {
                        AetObservation::ListItem(list_type, what, _, _) => {
                            if list_type.eq("Diagnose") {
                                FType::from_name(what)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    })
                    .collect::<HashSet<FType>>();
                timeline.for_agent(who, &move |me: &mut AgentState| {
                    me.hidden_state.clear_unknown();
                    for affliction in FType::afflictions() {
                        me.observe_flag(affliction, afflictions.contains(&affliction));
                    }
                });
            }
            "ColdRead" => {
                let emotions = after
                    .iter()
                    .filter_map(|item| match item {
                        AetObservation::ListItem(list_type, emotion, value, _) => {
                            let emotion = {
                                let mut c = emotion.chars();
                                match c.next() {
                                    None => String::new(),
                                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                                }
                            };
                            if let (Some(emotion), Some(value)) =
                                (emotion.parse().ok(), value.parse().ok())
                            {
                                Some((emotion, value))
                            } else {
                                None
                            }
                        }
                        _ => None,
                    })
                    .collect();
                let primary = after
                    .iter()
                    .filter_map(|item| match item {
                        AetObservation::ListItem(list_type, is_primary, emotion, _) => {
                            if is_primary.eq("Primary") {
                                Emotion::try_from_name(emotion)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    })
                    .next();
                timeline.for_agent(who, &move |me: &mut AgentState| {
                    me.bard_board.set_emotions(primary, &emotions);
                });
            }
            "Pipes" => {
                let pipes = after
                    .iter()
                    .filter_map(|item| match item {
                        AetObservation::ListItem(list_type, id, herb_amount, artifact) => {
                            let id = id.parse().ok();
                            let herb = herb_from_string(herb_amount);
                            let puffs = herb_amount
                                .rfind(" ")
                                .map(|idx| {
                                    let number_start = idx + 1;
                                    &herb_amount[number_start..]
                                })
                                .and_then(|number| number.parse().ok());
                            return Some((
                                herb.to_string(),
                                Pipe {
                                    id: id.unwrap_or_default(),
                                    artifact: artifact.eq("A"),
                                    lit: 0,
                                    puffs: puffs.unwrap_or_default(),
                                },
                            ));
                        }
                        _ => None,
                    })
                    .collect::<Vec<(String, Pipe)>>();
                timeline.for_agent(&timeline.me.clone(), &move |me: &mut AgentState| {
                    let mut found_yarrow = None;
                    let mut found_willow = None;
                    let mut found_reishi = None;
                    for (herb, pipe) in &pipes {
                        if herb.eq("yarrow") {
                            found_yarrow = Some(pipe);
                        } else if herb.eq("willow") {
                            found_willow = Some(pipe);
                        } else if herb.eq("reishi") {
                            found_reishi = Some(pipe);
                        }
                    }
                    for (herb, pipe) in &pipes {
                        if herb.eq("empty") {
                            if found_yarrow.is_none() {
                                found_yarrow = Some(pipe);
                            } else if found_willow.is_none() {
                                found_willow = Some(pipe);
                            } else if found_reishi.is_none() {
                                found_reishi = Some(pipe);
                            }
                        }
                    }
                    if let Some(pipe) = found_yarrow {
                        me.pipe_state.initialize("yarrow", pipe.clone());
                    }
                    if let Some(pipe) = found_willow {
                        me.pipe_state.initialize("willow", pipe.clone());
                    }
                    if let Some(pipe) = found_reishi {
                        me.pipe_state.initialize("reishi", pipe.clone());
                    }
                    println!(
                        "Pipes: {:?} {:?} {:?}",
                        found_yarrow, found_willow, found_reishi
                    );
                });
            }
            "Allies" => {
                let mut tta = -3;
                let mut allies = Vec::new();
                for (line, _num) in lines {
                    if tta == -3 && !line.contains("You claim these people as allies") {
                        continue;
                    }
                    tta += 1;
                    if tta >= 0 {
                        if line.contains("--") {
                            break;
                        }
                        allies.push(strip_ansi(line));
                    }
                }
                timeline.non_agent_states.insert(
                    format!("{}_allies", timeline.me),
                    AetNonAgent::Players(allies),
                );
            }
            "Enemies" => {
                let mut tta = -3;
                let mut enemies = Vec::new();
                for (line, _num) in lines {
                    if tta == -3 && !line.contains("You claim these people as foes") {
                        continue;
                    }
                    tta += 1;
                    if tta >= 0 {
                        if line.contains("--") {
                            break;
                        }
                        enemies.push(strip_ansi(line));
                    }
                }
                timeline.non_agent_states.insert(
                    format!("{}_enemies", timeline.me),
                    AetNonAgent::Players(enemies),
                );
            }
            _ => {}
        },
        AetObservation::Stripped(defense) => {
            timeline.set_flag_for_agent(&timeline.me.clone(), defense, false)?;
        }
        AetObservation::LostRebound(who) => {
            timeline.set_flag_for_agent(who, &"Rebounding".to_string(), false)?;
        }
        AetObservation::LostShield(who) => {
            timeline.set_flag_for_agent(who, &"Shielded".to_string(), false)?;
        }
        AetObservation::LostFangBarrier(who) => {
            timeline.set_flag_for_agent(who, &"Fangbarrier".to_string(), false)?;
        }
        AetObservation::Gained(who, defence) => {
            timeline.set_flag_for_agent(who, defence, true)?;
            if defence.eq("rebounding") {
                for_agent(timeline, who, &|me: &mut AgentState| {
                    me.set_balance(BType::Rebounding, 0.0);
                });
            }
        }
        AetObservation::LimbDamage(what, much) => {
            timeline.adjust_agent_limb(&timeline.me.clone(), what, *much)?;
        }
        AetObservation::LimbHeal(what, much) => {
            timeline.adjust_agent_limb(&timeline.me.clone(), what, -much)?;
        }
        AetObservation::LimbDone(what) => {
            timeline.finish_agent_restore(&timeline.me.clone(), what)?;
        }
        AetObservation::Stand(who) => {
            timeline.set_flag_for_agent(who, &"asleep".to_string(), false);
            timeline.set_flag_for_agent(who, &"fallen".to_string(), false);
            if timeline.borrow_agent(who).is(FType::Backstrain) {
                let after = after.clone();
                for_agent(timeline, who, &move |you| {
                    apply_limb_damage(
                        you,
                        (LType::TorsoDamage, 10.0, you.is(FType::Stiffness)),
                        &after,
                    );
                });
            }
        }
        AetObservation::Fall(who) => {
            timeline.set_flag_for_agent(who, &"fallen".to_string(), true);
        }
        AetObservation::ParryStart(who, what) => {
            let limb = get_limb_damage(what)?;
            for_agent(timeline, who, &move |me: &mut AgentState| {
                me.set_parrying(limb);
            });
        }
        AetObservation::Parry(who, what) => {
            let limb = get_limb_damage(what)?;
            let who = who.clone();
            let after = after.clone();
            for_agent(timeline, &who, &move |you: &mut AgentState| {
                you.set_parrying(limb);
                if you.is(FType::SoreWrist) {
                    apply_limb_damage(
                        you,
                        (LType::LeftArmDamage, 4.0, you.is(FType::Stiffness)),
                        &after,
                    );
                    apply_limb_damage(
                        you,
                        (LType::LeftArmDamage, 4.0, you.is(FType::Stiffness)),
                        &after,
                    );
                }
            });
        }
        AetObservation::Wield { who, what, hand } => {
            let left = if hand.eq("left") {
                Some(what.clone())
            } else {
                None
            };
            let right = if hand.eq("right") {
                Some(what.clone())
            } else {
                None
            };
            for_agent(timeline, &who, &move |me: &mut AgentState| {
                me.wield_multi(left.clone(), right.clone());
            });
        }
        AetObservation::Unwield { who, what: _, hand } => {
            let left = hand.eq("left");
            let right = hand.eq("right");
            for_agent(timeline, &who, &move |me: &mut AgentState| {
                me.unwield_multi(left, right);
            });
        }
        AetObservation::DualWield { who, left, right } => {
            let left = left.clone();
            let right = right.clone();
            for_agent(timeline, &who, &move |me: &mut AgentState| {
                me.wield_multi(Some(left.clone()), Some(right.clone()));
            });
        }
        AetObservation::TwoHandedWield(who, what) => {
            let what = what.clone();
            for_agent(timeline, &who, &move |me: &mut AgentState| {
                me.wield_two_hands(what.clone());
            });
        }
        AetObservation::TickAff(who, what) => {
            timeline.tick_counter_up_for_agent(who, what)?;
        }
        AetObservation::Relapse(who) => {
            if before.len() == 0 {
                // Just don't do the next check.
            } else if before.contains(&AetObservation::Relapse(who.to_string())) {
                // We've already handled this block.
                return Ok(());
            }
            let after = after.clone();
            for_agent_uncertain_closure(
                timeline,
                &who,
                Box::new(move |you| apply_or_infer_relapse(you, &after)),
            );
        }
        AetObservation::FillPipe(herb_str) => {
            let herb = herb_from_string(herb_str).to_string();
            for_agent(
                timeline,
                &timeline.me.clone(),
                &move |me: &mut AgentState| {
                    me.pipe_state.refill(&herb);
                },
            );
        }
        _ => {}
    }
    Ok(())
}

pub fn apply_or_infer_suggestion(
    who: &mut AgentState,
    after: &Vec<AetObservation>,
) -> Result<(), String> {
    let top_hypno = who.hypno_state.fire();
    if let Some(AetObservation::OtherAfflicted(_who, affliction)) = after.get(1) {
        if let Some(affliction) = FType::from_name(affliction) {
            who.set_flag(affliction, true);
        }
    } else if let Some(Hypnosis::Aff(_affliction)) = top_hypno {
        println!("Expected {:?} but got {:?}!", top_hypno, after.get(0));
    }
    Ok(())
}

pub fn apply_venom(who: &mut AgentState, venom: &String, relapse: bool) -> Result<(), String> {
    let mut guessed_aff = None;
    if who.is(FType::ThinBlood) && !relapse {
        who.push_toxin(venom.clone());
    }
    if venom == "prefarar" && who.is(FType::Deafness) {
        who.set_flag(FType::Deafness, false);
    } else if venom == "oculus" && who.is(FType::Blindness) {
        who.set_flag(FType::Deafness, false);
    } else if venom == "epseth" {
        if who.is(FType::LeftLegBroken) {
            who.set_flag(FType::RightLegBroken, true);
            guessed_aff = Some(FType::RightLegBroken);
        } else {
            who.set_flag(FType::LeftLegBroken, true);
            guessed_aff = Some(FType::LeftLegBroken);
        }
    } else if venom == "epteth" {
        if who.is(FType::LeftArmBroken) {
            who.set_flag(FType::RightArmBroken, true);
            guessed_aff = Some(FType::RightArmBroken);
        } else {
            who.set_flag(FType::LeftArmBroken, true);
            guessed_aff = Some(FType::LeftArmBroken);
        }
    } else if let Some(affliction) = VENOM_AFFLICTS.get(venom) {
        who.set_flag(*affliction, true);
        guessed_aff = Some(*affliction);
    } else if venom == "asp" || venom == "loki" {
        who.hidden_state.add_unknown();
    } else if venom == "camus" {
        who.set_stat(SType::Health, who.get_stat(SType::Health) - 1000);
    } else if venom == "delphinium" && who.is(FType::Insomnia) {
        who.set_flag(FType::Insomnia, false);
    } else if venom == "delphinium" && !who.is(FType::Asleep) {
        who.set_flag(FType::Asleep, true);
    } else if venom == "delphinium" {
        who.set_flag(FType::Instawake, false);
    } else if venom == "wasi" {
        // Revenant
        who.set_flag(FType::Rebounding, false);
    } else if (venom == "azu" || venom == "cripple") && !who.is(FType::Crippled) {
        // Revenant
        who.set_flag(FType::Crippled, true);
    } else if (venom == "azu" || venom == "cripple") && who.is(FType::Crippled) {
        // Revenant
        who.set_flag(FType::PhysicalDisruption, true);
    } else if (venom == "dirne" || venom == "disrupt") && !who.is(FType::PhysicalDisruption) {
        // Revenant
        who.set_flag(FType::PhysicalDisruption, true);
    } else if (venom == "dirne" || venom == "disrupt") && who.is(FType::PhysicalDisruption) {
        // Revenant
        who.set_flag(FType::MentalDisruption, true);
    } else {
        return Err(format!("Could not determine effect of {}", venom));
    }
    if relapse {
        if let Some(guessed_aff) = guessed_aff {
            who.add_guess(guessed_aff);
        }
    }
    Ok(())
}

lazy_static! {
    pub static ref CALLED_VENOM: Regex = Regex::new(r"(\w+)").unwrap();
    pub static ref CALLED_VENOMS_TWO: Regex = Regex::new(r"(\w+),? (\w+)").unwrap();
    pub static ref CALLED_VENOMS_THREE: Regex = Regex::new(r"(\w+),? (\w+),? (\w+)").unwrap();
}

pub fn apply_weapon_hits(
    agent_states: &mut AetTimelineState,
    me: &String,
    you: &String,
    observations: &Vec<AetObservation>,
    first_person: bool,
    venom_hints: &Option<String>,
) -> Result<(), String> {
    if first_person {
        for i in 0..observations.len() {
            if let Some(AetObservation::Devenoms(venom)) = observations.get(i) {
                if let Some(AetObservation::Rebounds) = observations.get(i - 1) {
                    agent_states.for_agent(you, &|you: &mut AgentState| {
                        you.observe_flag(FType::Rebounding, true);
                    });
                    let venom = venom.clone();
                    agent_states.for_agent(me, &move |me: &mut AgentState| {
                        apply_venom(me, &venom, false);
                    });
                } else {
                    if let Some(AetObservation::PurgeVenom(_, _v2)) = observations.get(i + 1) {
                    } else {
                        let venom = venom.clone();
                        agent_states.for_agent(you, &move |you| {
                            apply_venom(you, &venom, false);
                        });
                    }
                }
            } else if let Some(AetObservation::CombatAction(_)) = observations.get(i) {
                if i > 0 {
                    break;
                }
            }
        }
    } else if let Some(venom_hints) = venom_hints {
        let mut venoms = Vec::new();
        if let Some(captures) = CALLED_VENOMS_TWO.captures(&venom_hints) {
            venoms.push(captures.get(1).unwrap().as_str().to_string());
            venoms.push(captures.get(2).unwrap().as_str().to_string());
        } else if let Some(captures) = CALLED_VENOM.captures(&venom_hints) {
            venoms.push(captures.get(1).unwrap().as_str().to_string());
        } else {
            return Ok(());
        }
        if let Some(AetObservation::Dodges(_)) = observations.get(1) {
            venoms.pop();
        } else if let Some(AetObservation::Dodges(_)) = observations.get(1) {
            venoms.pop();
        }
        agent_states.for_agent(you, &move |you| {
            for venom in venoms.iter() {
                apply_venom(you, &venom, false);
            }
        });
    }
    Ok(())
}

pub fn attack_hit(observations: &Vec<AetObservation>) -> bool {
    for (i, observation) in observations.iter().enumerate() {
        match (i, observation) {
            (0, AetObservation::CombatAction(_)) => {}
            (_, AetObservation::CombatAction(_)) => {
                // If we see another combat message, assume we're good to apply limb damage.
                return true;
            }
            (_, AetObservation::Dodges(_)) => {
                // Attack dodged.
                return false;
            }
            (_, AetObservation::Misses(_)) => {
                // Attack missed.
                return false;
            }
            (_, AetObservation::Absorbed(_, _)) => {
                // Attack absorbed.
                return false;
            }
            (_, AetObservation::Parry(_, _)) => {
                return false;
            }
            _ => {}
        }
    }
    return true;
}

pub fn limb_damaged(observations: &Vec<AetObservation>, limb: LType) -> bool {
    let limb_string = limb.to_string();
    for (i, observation) in observations.iter().enumerate() {
        match (i, observation) {
            (_, AetObservation::Damaged(_who, what)) => {
                if limb_string.eq(what) {
                    return true;
                }
            }
            _ => {}
        }
    }
    return false;
}

pub fn limb_mangled(observations: &Vec<AetObservation>, limb: LType) -> bool {
    let limb_string = limb.to_string();
    for (i, observation) in observations.iter().enumerate() {
        match (i, observation) {
            (_, AetObservation::Mangled(_who, what)) => {
                if limb_string.eq(what) {
                    return true;
                }
            }
            _ => {}
        }
    }
    return false;
}

pub fn apply_limb_damage(
    target: &mut AgentState,
    expected_value: (LType, f32, bool),
    observations: &Vec<AetObservation>,
) -> Result<(), String> {
    let (limb_hit, damage_dealt, should_break) = expected_value;
    for (i, observation) in observations.iter().enumerate() {
        match (i, observation) {
            (0, AetObservation::CombatAction(_)) => {}
            (_, AetObservation::CombatAction(_)) => {
                // If we see another combat message, assume we're good to apply limb damage.
                break;
            }
            (_, AetObservation::LimbDamage(limb, amount)) => {
                // If we find actual limb damage, we're the target and don't need to infer.
                return Ok(());
            }
            _ => {}
        }
    }
    if attack_hit(observations) {
        target
            .limb_damage
            .adjust_limb(limb_hit, (damage_dealt * 100.0) as CType);
        if should_break {
            if limb_damaged(observations, limb_hit) {
                if target.limb_damage.get_damage(limb_hit) <= DAMAGED_VALUE {
                    println!(
                        "{:?} break at {}",
                        limb_hit,
                        target.limb_damage.get_damage(limb_hit)
                    );
                }
                target.limb_damage.set_limb_damaged(limb_hit, true);
            } else if !target.limb_damage.damaged(limb_hit)
                && target.limb_damage.get_damage(limb_hit) > DAMAGED_VALUE
            {
                println!(
                    "No {:?} break at {}",
                    limb_hit,
                    target.limb_damage.get_damage(limb_hit)
                );
                target.set_limb_damage(limb_hit, DAMAGED_VALUE);
            }
            if limb_mangled(observations, limb_hit) {
                if target.limb_damage.get_damage(limb_hit) <= MANGLED_VALUE {
                    println!(
                        "{:?} mangle at {}",
                        limb_hit,
                        target.limb_damage.get_damage(limb_hit)
                    );
                }
                target.limb_damage.set_limb_mangled(expected_value.0, true);
            } else if !target.limb_damage.mangled(limb_hit)
                && target.limb_damage.get_damage(limb_hit) > MANGLED_VALUE
            {
                println!(
                    "No {:?} mangle at {}",
                    limb_hit,
                    target.limb_damage.get_damage(limb_hit)
                );
                target.set_limb_damage(limb_hit, MANGLED_VALUE);
            }
        }
    }
    Ok(())
}

pub fn apply_or_infer_relapse(
    who: &mut AgentState,
    after: &Vec<AetObservation>,
) -> Option<Vec<AgentState>> {
    who.observe_flag(FType::ThinBlood, true);
    let mut relapse_count = 1;
    let mut name = "";
    for observation in after.iter() {
        if let AetObservation::Relapse(next_name) = observation {
            if name.eq("") {
                name = next_name;
            } else if name.eq(next_name) {
                relapse_count += 1;
            } else {
                break;
            }
        }
    }
    match who.get_relapses(relapse_count) {
        RelapseResult::Concrete(venoms, expired) => {
            for venom in venoms.iter() {
                apply_venom(who, &venom, true);
            }
            if expired > 0 {
                who.branch_state.strike();
            }
        }
        RelapseResult::Uncertain(len, venoms, expired) => {
            let possibilities = topper_core::combinatorics::combinations(venoms.as_slice(), len);
            if possibilities.len() > 0 {
                println!("Branching {} more times...", possibilities.len() - 1);
            } else {
                println!("Pruning branch for no possible relapses.");
            }
            let mut branches = Vec::new();
            for relapse_sets in possibilities.iter() {
                let mut branch = who.clone();
                for (time, venom) in relapse_sets.iter() {
                    branch.relapses.drop_relapse(*time, venom);
                    apply_venom(&mut branch, venom, true);
                }
                branches.push(branch);
            }
            if expired > 0 {
                who.branch_state.strike();
            }
            if branches.len() > 0 {
                return Some(branches);
            }
        }
        RelapseResult::None => {
            who.branch_state.strike();
        }
    }
    None
}

pub fn apply_or_infer_balance(
    who: &mut AgentState,
    expected_value: (BType, f32),
    observations: &Vec<AetObservation>,
) {
    for observation in observations.iter() {
        match observation {
            AetObservation::Balance(btype, duration) => {
                if BType::from_name(&btype) == expected_value.0 {
                    return;
                }
            }
            _ => {}
        }
    }
    who.set_balance(expected_value.0, expected_value.1);
}

pub fn apply_or_infer_combo_balance(
    who: &mut AgentState,
    expected_value: (BType, f32),
    observations: &Vec<AetObservation>,
) {
    for observation in observations.iter() {
        match observation {
            AetObservation::Balance(btype, duration) => {
                if BType::from_name(&btype) == expected_value.0 {
                    return;
                }
            }
            _ => {}
        }
    }
    who.set_balance(expected_value.0, expected_value.1);
}

pub fn apply_or_strike_random_cure(
    who: &mut AgentState,
    after: &Vec<AetObservation>,
    perspective: Perspective,
    (len, affs): (usize, Vec<FType>),
) {
    let mut discerned = 0;
    for (i, observation) in after.iter().enumerate() {
        match (i, observation) {
            (0, AetObservation::CombatAction(_)) => {}
            (_, AetObservation::CombatAction(_)) => {
                // We're into the next CombatAction!
                break;
            }
            (_, AetObservation::Cured(aff_name)) => {
                if let Some(aff) = FType::from_name(&aff_name) {
                    who.toggle_flag(aff, false);
                }
                discerned += 1;
            }
            (_, AetObservation::DiscernedCure(_, aff_name)) => {
                if let Some(aff) = FType::from_name(&aff_name) {
                    who.toggle_flag(aff, false);
                }
                discerned += 1;
            }
            _ => {}
        }
    }
    if discerned < len {
        for aff in affs {
            who.observe_flag(aff, false);
        }
    }
}

pub fn apply_or_infer_random_afflictions(
    who: &mut AgentState,
    after: &Vec<AetObservation>,
    perspective: Perspective,
    possible_affs: Option<(usize, Vec<FType>)>,
) -> Option<Vec<AgentState>> {
    let mut discerned = false;
    for (i, observation) in after.iter().enumerate() {
        match (i, observation) {
            (0, AetObservation::CombatAction(_)) => {}
            (_, AetObservation::CombatAction(_)) => {
                // We're into the next CombatAction!
                break;
            }
            (_, AetObservation::Afflicted(aff_name)) => {
                if perspective == Perspective::Target {
                    if let Some(aff) = FType::from_name(&aff_name) {
                        who.toggle_flag(aff, true);
                    }
                    discerned = true;
                }
            }
            (_, AetObservation::Stripped(def_name)) => {
                if perspective == Perspective::Target {
                    if let Some(def) = FType::from_name(&def_name) {
                        who.toggle_flag(def, false);
                    }
                    discerned = true;
                }
            }
            (_, AetObservation::DiscernedAfflict(aff_name)) => {
                if perspective != Perspective::Target {
                    if let Some(aff) = FType::from_name(&aff_name) {
                        who.toggle_flag(aff, true);
                    }
                    discerned = true;
                }
            }
            _ => {}
        }
    }
    if discerned {
        None
    } else if let Some((len, affs)) = possible_affs {
        let possibilities = topper_core::combinatorics::combinations(affs.as_slice(), len);
        if possibilities.len() > 0 {
            println!("Branching {} more times...", possibilities.len() - 1);
        } else {
            println!("No branching!");
            return None;
        }
        let mut branches = Vec::new();
        for aff_sets in possibilities.iter() {
            let mut branch = who.clone();
            for aff in aff_sets.iter() {
                branch.set_flag(*aff, true);
                branch.add_guess(*aff);
            }
            branches.push(branch);
        }
        Some(branches)
    } else {
        None
    }
}

pub fn apply_or_infer_cures(
    who: &mut AgentState,
    cures: Vec<FType>,
    after: &Vec<AetObservation>,
    first_person: bool,
) -> Result<(), String> {
    let mut found_cures = Vec::new();
    if first_person {
        for observation in after.iter() {
            match observation {
                AetObservation::Cured(aff_name) => {
                    if let Some(aff) = FType::from_name(&aff_name) {
                        who.toggle_flag(aff, false);
                        if aff == FType::ThinBlood {
                            who.clear_relapses();
                        } else if aff == FType::Void {
                            who.set_flag(FType::Weakvoid, true);
                        }
                        found_cures.push(aff);
                    }
                }
                AetObservation::Stripped(def_name) => {
                    if let Some(def) = FType::from_name(&def_name) {
                        who.toggle_flag(def, false);
                        found_cures.push(def);
                    }
                }
                _ => {}
            }
        }
        if found_cures.len() == 0 {
            for aff in cures.iter() {
                who.observe_flag(*aff, false);
            }
        }
    } else {
        remove_in_order(cures, who);
    }
    Ok(())
}

pub fn apply_or_infer_cure(
    who: &mut AgentState,
    cure: &SimpleCure,
    after: &Vec<AetObservation>,
    first_person: bool,
) -> Result<Vec<FType>, String> {
    let mut found_cures = Vec::new();
    if let Some(AetObservation::Cured(aff_name)) = after.get(1) {
        if let Some(aff) = FType::from_name(&aff_name) {
            match cure {
                SimpleCure::Pill(pill_name) => {
                    who.observe_flag(FType::Anorexia, false);
                    if aff != FType::Void && aff != FType::Weakvoid {
                        if let Some(order) = PILL_CURE_ORDERS.get(pill_name) {
                            for pill_aff in order.iter() {
                                if *pill_aff == aff {
                                    break;
                                } else {
                                    who.observe_flag(*pill_aff, false);
                                }
                            }
                        }
                    }
                }
                SimpleCure::Salve(salve_name, salve_loc) => {
                    who.observe_flag(FType::Slickness, false);
                    if aff != FType::Void && aff != FType::Weakvoid {
                        if let Some(order) =
                            SALVE_CURE_ORDERS.get(&(salve_name.to_string(), salve_loc.to_string()))
                        {
                            for salve_aff in order.iter() {
                                if *salve_aff == aff {
                                    break;
                                } else {
                                    who.observe_flag(*salve_aff, false);
                                }
                            }
                        }
                    }
                }
                SimpleCure::Smoke(herb_name) => {
                    who.observe_flag(FType::Asthma, false);
                    if aff != FType::Void && aff != FType::Weakvoid {
                        if let Some(order) = SMOKE_CURE_ORDERS.get(herb_name) {
                            for herb_aff in order.iter() {
                                if *herb_aff == aff {
                                    break;
                                } else {
                                    who.observe_flag(*herb_aff, false);
                                }
                            }
                        }
                    }
                }
            }
            who.toggle_flag(aff, false);
            found_cures.push(aff);
        }
    } else if let Some(AetObservation::Stripped(def_name)) = after.get(1) {
        if let Some(def) = FType::from_name(&def_name) {
            match cure {
                SimpleCure::Pill(pill_name) => {
                    who.observe_flag(FType::Anorexia, false);
                    if let Some(order) = PILL_CURE_ORDERS.get(pill_name) {
                        for pill_aff in order.iter() {
                            if *pill_aff == def {
                                break;
                            } else {
                                who.observe_flag(*pill_aff, false);
                            }
                        }
                    }
                }
                SimpleCure::Salve(salve_name, salve_loc) => {
                    who.observe_flag(FType::Slickness, false);
                    if let Some(order) =
                        SALVE_CURE_ORDERS.get(&(salve_name.to_string(), salve_loc.to_string()))
                    {
                        for salve_aff in order.iter() {
                            if *salve_aff == def {
                                break;
                            } else {
                                who.observe_flag(*salve_aff, false);
                            }
                        }
                    }
                }
                SimpleCure::Smoke(herb_name) => {
                    who.observe_flag(FType::Asthma, false);
                    if let Some(order) = SMOKE_CURE_ORDERS.get(herb_name) {
                        for herb_aff in order.iter() {
                            if *herb_aff == def {
                                break;
                            } else {
                                who.observe_flag(*herb_aff, false);
                            }
                        }
                    }
                }
            }
            who.toggle_flag(def, false);
            found_cures.push(def);
        }
    } else {
        match cure {
            SimpleCure::Pill(pill_name) => {
                who.observe_flag(FType::Anorexia, false);
                if pill_name == "anabiotic" {
                } else if let Some(order) = PILL_CURE_ORDERS.get(pill_name) {
                    if first_person {
                        for pill_aff in order.iter() {
                            who.observe_flag(*pill_aff, false);
                        }
                    } else {
                        let cured = top_aff(who, order.to_vec());
                        remove_in_order(order.to_vec(), who);
                        if cured == Some(FType::ThinBlood) {
                            who.clear_relapses();
                        }
                    }
                } else if let Some(defence) = PILL_DEFENCES.get(pill_name) {
                    if *defence == FType::Insomnia && who.is(FType::Hypersomnia) {
                    } else {
                        who.set_flag(*defence, true);
                    }
                } else {
                    return Err(format!("Could not find pill {}", pill_name));
                }
            }
            SimpleCure::Salve(salve_name, salve_loc) => {
                who.observe_flag(FType::Slickness, false);
                if salve_name == "caloric" {
                    if who.some(CALORIC_TORSO_ORDER.to_vec()) {
                        remove_in_order(CALORIC_TORSO_ORDER.to_vec(), who);
                    } else {
                        who.set_flag(FType::Insulation, true);
                    }
                } else if salve_name == "mass" {
                    who.set_flag(FType::Density, true);
                } else if salve_name == "restoration" {
                    let limb = get_limb_damage(salve_loc)?;
                    who.limb_damage.start_restore(limb, first_person);
                } else if let Some(order) =
                    SALVE_CURE_ORDERS.get(&(salve_name.to_string(), salve_loc.to_string()))
                {
                    if let Ok(limb) = get_limb_damage(salve_loc) {
                        if !who.limb_damage.damaged(limb) {
                            if !first_person {
                                remove_in_order(order.to_vec(), who);
                            }
                        } else {
                            println!("{} fizzled on {}", salve_name, salve_loc);
                        }
                    } else {
                        if first_person {
                            for salve_aff in order.iter() {
                                who.observe_flag(*salve_aff, false);
                            }
                        } else {
                            remove_in_order(order.to_vec(), who);
                        }
                    }
                } else {
                    return Err(format!("Could not find {} on {}", salve_name, salve_loc));
                }
            }
            SimpleCure::Smoke(herb_name) => {
                who.observe_flag(FType::Asthma, false);
                if let Some(order) = SMOKE_CURE_ORDERS.get(herb_name) {
                    if first_person {
                        for smoke_aff in order.iter() {
                            who.observe_flag(*smoke_aff, false);
                        }
                    } else {
                        remove_in_order(order.to_vec(), who);
                    }
                } else if herb_name == "reishi" {
                    if who.is(FType::Besilence) {
                        who.toggle_flag(FType::Besilence, false);
                    } else {
                        who.set_balance(BType::Rebounding, 6.25);
                    }
                } else {
                    return Err(format!("Could not find smoke {}", herb_name));
                }
            } // _ => {}
        }
    }
    Ok(found_cures)
}

pub fn for_agent(agent_states: &mut AetTimelineState, target: &String, act: &Fn(&mut AgentState)) {
    agent_states.for_agent(target, act)
}

pub fn for_agent_uncertain(
    agent_states: &mut AetTimelineState,
    target: &String,
    act: fn(&mut AgentState) -> Option<Vec<AgentState>>,
) {
    agent_states.for_agent_uncertain(target, act)
}

pub fn for_agent_uncertain_closure(
    agent_states: &mut AetTimelineState,
    target: &String,
    act: Box<dyn Fn(&mut AgentState) -> Option<Vec<AgentState>>>,
) {
    agent_states.for_agent_uncertain_closure(target, act)
}

pub fn attack_afflictions(
    agent_states: &mut AetTimelineState,
    target: &String,
    affs: Vec<FType>,
    after: &Vec<AetObservation>,
) {
    if attack_hit(after) {
        agent_states.for_agent(target, &move |you: &mut AgentState| {
            for aff in affs.clone().iter() {
                you.set_flag(*aff, true);
            }
        });
    }
}

pub fn attack_strip(
    agent_states: &mut AetTimelineState,
    target: &String,
    defs: Vec<FType>,
    after: &Vec<AetObservation>,
) {
    if attack_hit(after) {
        agent_states.for_agent(target, &move |you: &mut AgentState| {
            for def in defs.clone().iter() {
                you.set_flag(*def, false);
            }
        });
    }
}

pub fn attack_strip_or_afflict(
    agent_states: &mut AetTimelineState,
    target: &String,
    aff_defs: Vec<FType>,
    after: &Vec<AetObservation>,
) {
    if attack_hit(after) {
        agent_states.for_agent(target, &move |you: &mut AgentState| {
            for aff_def in aff_defs.clone().iter() {
                if !aff_def.is_affliction() && you.is(*aff_def) {
                    you.set_flag(*aff_def, false);
                    break;
                } else if aff_def.is_affliction() && !you.is(*aff_def) {
                    you.set_flag(*aff_def, true);
                    break;
                }
            }
        });
    }
}

pub fn apply_combo_balance(
    agent_states: &mut AetTimelineState,
    caster: &String,
    expected: (BType, f32),
    after: &Vec<AetObservation>,
) {
    let observations = after.clone();
    for_agent(agent_states, caster, &move |me: &mut AgentState| {
        apply_or_infer_combo_balance(me, expected, &observations);
    });
}

pub fn attack_limb_damage(
    agent_states: &mut AetTimelineState,
    target: &String,
    expected: (LType, f32, bool),
    after: &Vec<AetObservation>,
) {
    let observations = after.clone();
    for_agent(agent_states, target, &move |you| {
        apply_limb_damage(you, expected, &observations);
    });
}
