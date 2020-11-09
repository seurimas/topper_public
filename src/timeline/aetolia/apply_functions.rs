use crate::classes::{get_skill_class, handle_combat_action, handle_sent, Class, VENOM_AFFLICTS};
use crate::curatives::{
    handle_simple_cure_action, remove_in_order, top_aff, CALORIC_TORSO_ORDER, PILL_CURE_ORDERS,
    PILL_DEFENCES, SALVE_CURE_ORDERS, SMOKE_CURE_ORDERS,
};
use crate::timeline::aetolia::types::*;
use crate::timeline::CType;
use crate::types::*;
use log::warn;
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;

pub fn apply_observation(
    timeline: &mut AetTimelineState,
    observation: &AetObservation,
    before: &Vec<AetObservation>,
    after: &Vec<AetObservation>,
) -> Result<(), String> {
    match observation {
        AetObservation::Sent(command) => {
            handle_sent(command, timeline);
        }
        AetObservation::CombatAction(combat_action) => {
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
            if timeline.get_agent(who).get_count(FType::Ablaze) <= 1 {
                timeline.set_flag_for_agent(who, &"Ablaze".to_string(), false)?;
            }
        }
        AetObservation::Afflicted(affliction) => {
            if affliction.eq("sapped_strength") {
                timeline.tick_counter_up_for_agent(&timeline.me.clone(), affliction);
            } else {
                timeline.set_flag_for_agent(&timeline.me.clone(), affliction, true)?;
            }
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
            println!("{} {}", who, affliction);
            timeline.set_flag_for_agent(who, affliction, true)?;
        }
        AetObservation::Dodges(who) => {
            let mut me = timeline.get_agent(who);
            me.dodge_state.register_dodge();
            timeline.set_agent(who, me)
        }
        AetObservation::WoundStart(who) => {
            let mut me = timeline.get_agent(who);
            for after in after.iter() {
                match after {
                    AetObservation::Wound(limb, damage) => {
                        if let Ok(limb) = get_limb_damage(limb) {
                            me.set_limb_damage(limb, (damage * 100.0) as CType);
                        }
                    }
                    _ => {}
                }
            }
            timeline.set_agent(who, me);
        }
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
                let mut me = timeline.get_agent(who);
                me.set_balance(BType::Rebounding, 0.0);
                timeline.set_agent(who, me);
            }
        }
        AetObservation::LimbDamage(what, much) => {
            println!("{} {}", what, much);
            timeline.adjust_agent_limb(&timeline.me.clone(), what, *much)?;
        }
        AetObservation::LimbHeal(what, much) => {
            timeline.adjust_agent_limb(&timeline.me.clone(), what, -much)?;
        }
        AetObservation::LimbDone(what) => {
            timeline.finish_agent_restore(&timeline.me.clone(), what)?;
        }
        AetObservation::Stand(who) => {
            timeline.set_flag_for_agent(who, &"fallen".to_string(), false);
            if timeline.get_agent(who).is(FType::Backstrain) {
                let after = after.clone();
                for_agent_closure(
                    timeline,
                    who,
                    Box::new(move |you| {
                        apply_limb_damage(
                            you,
                            (LType::TorsoDamage, 10.0, you.is(FType::Stiffness)),
                            &after,
                        );
                    }),
                );
            }
        }
        AetObservation::Fall(who) => {
            timeline.set_flag_for_agent(who, &"fallen".to_string(), true);
        }
        AetObservation::ParryStart(who, what) => {
            let mut me = timeline.get_agent(who);
            let limb = get_limb_damage(what)?;
            me.set_parrying(limb);
            timeline.set_agent(who, me);
        }
        AetObservation::Parry(who, what) => {
            let mut me = timeline.get_agent(who);
            let limb = get_limb_damage(what)?;
            me.set_parrying(limb);
            timeline.set_agent(who, me);
            if timeline.get_agent(who).is(FType::SoreWrist) {
                let after = after.clone();
                for_agent_closure(
                    timeline,
                    who,
                    Box::new(move |you| {
                        apply_limb_damage(
                            you,
                            (LType::LeftArmDamage, 5.0, you.is(FType::Stiffness)),
                            &after,
                        );
                        apply_limb_damage(
                            you,
                            (LType::LeftArmDamage, 5.0, you.is(FType::Stiffness)),
                            &after,
                        );
                    }),
                );
            }
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
            let mut me = timeline.get_agent(who);
            me.wield_multi(left, right);
            timeline.set_agent(who, me);
        }
        AetObservation::Unwield { who, what: _, hand } => {
            let left = hand.eq("left");
            let right = hand.eq("right");
            let mut me = timeline.get_agent(who);
            me.unwield_multi(left, right);
            timeline.set_agent(who, me);
        }
        AetObservation::DualWield { who, left, right } => {
            let mut me = timeline.get_agent(who);
            me.wield_multi(Some(left.clone()), Some(right.clone()));
            timeline.set_agent(who, me);
        }
        AetObservation::TwoHandedWield(who, what) => {
            let mut me = timeline.get_agent(who);
            me.wield_two_hands(what.clone());
            timeline.set_agent(who, me);
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
            let mut you = timeline.get_agent(who);
            apply_or_infer_relapse(&mut you, after)?;
            timeline.set_agent(who, you);
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
    } else if let Some(Hypnosis::Aff(_affliction)) = who.hypno_state.hypnosis_stack.get(0) {
        println!("Expected {:?} but got {:?}!", top_hypno, after.get(0));
    }
    Ok(())
}

pub fn apply_venom(who: &mut AgentState, venom: &String) -> Result<(), String> {
    if who.is(FType::ThinBlood) {
        who.push_toxin(venom.clone());
    }
    if venom == "prefarar" && who.is(FType::Deafness) {
        who.set_flag(FType::Deafness, false);
    } else if venom == "oculus" && who.is(FType::Blindness) {
        who.set_flag(FType::Deafness, false);
    } else if venom == "epseth" {
        if who.is(FType::LeftLegBroken) {
            who.set_flag(FType::RightLegBroken, true);
        } else {
            who.set_flag(FType::LeftLegBroken, true);
        }
    } else if venom == "epteth" {
        if who.is(FType::LeftArmBroken) {
            who.set_flag(FType::RightArmBroken, true);
        } else {
            who.set_flag(FType::LeftArmBroken, true);
        }
    } else if let Some(affliction) = VENOM_AFFLICTS.get(venom) {
        who.set_flag(*affliction, true);
    } else if venom == "camus" {
        who.set_stat(SType::Health, who.get_stat(SType::Health) - 1000);
    } else if venom == "delphinium" && who.is(FType::Insomnia) {
        who.set_flag(FType::Insomnia, false);
    } else if venom == "delphinium" {
        who.set_flag(FType::Asleep, true);
    } else {
        return Err(format!("Could not determine effect of {}", venom));
    }
    Ok(())
}

lazy_static! {
    static ref CALLED_VENOM: Regex = Regex::new(r"(\w+)").unwrap();
}

lazy_static! {
    static ref CALLED_VENOMS_TWO: Regex = Regex::new(r"(\w+),? (\w+)").unwrap();
}

pub fn apply_weapon_hits(
    attacker: &mut AgentState,
    target: &mut AgentState,
    observations: &Vec<AetObservation>,
    first_person: bool,
    venom_hints: Option<String>,
) -> Result<(), String> {
    if first_person {
        for i in 0..observations.len() {
            if let Some(AetObservation::Devenoms(venom)) = observations.get(i) {
                if let Some(AetObservation::Rebounds) = observations.get(i - 1) {
                    target.set_flag(FType::Rebounding, true);
                    apply_venom(attacker, venom)?;
                } else {
                    if let Some(AetObservation::PurgeVenom(_, _v2)) = observations.get(i + 1) {
                    } else {
                        apply_venom(target, venom)?;
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
        }
        if let Some(AetObservation::Dodges(_)) = observations.get(1) {
            println!("Dodged");
            venoms.pop();
        } else if let Some(AetObservation::Dodges(_)) = observations.get(1) {
            println!("Dodged");
            venoms.pop();
        }
        println!("Caught {:?}", venoms);
        for venom in venoms.iter() {
            apply_venom(target, venom)?;
        }
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
) -> Result<(), String> {
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
    println!("{} {}", relapse_count, name);
    match who.get_relapses(relapse_count) {
        RelapseResult::Concrete(venoms) => {
            println!("Relapses: {:?}", venoms);
            for venom in venoms.iter() {
                apply_venom(who, &venom)?;
            }
        }
        RelapseResult::Uncertain(venoms) => {
            println!("Possible relapses: {:?}", venoms);
        }
        RelapseResult::None => {
            println!("No relapses found???");
        }
    }
    Ok(())
}

pub fn apply_or_infer_balance(
    who: &mut AgentState,
    expected_value: (BType, f32),
    observations: &Vec<AetObservation>,
) {
    for observation in observations.iter() {
        match observation {
            AetObservation::Balance(btype, duration) => {
                who.set_balance(BType::from_name(&btype), *duration);
                return;
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
                who.set_balance(BType::from_name(&btype), *duration);
                return;
            }
            _ => {}
        }
    }
    who.set_balance(expected_value.0, expected_value.1);
}

pub fn apply_or_infer_random_afflictions(
    who: &mut AgentState,
    after: &Vec<AetObservation>,
) -> Result<(), String> {
    for observation in after.iter() {
        match observation {
            AetObservation::DiscernedAfflict(aff_name) => {
                if let Some(aff) = FType::from_name(&aff_name) {
                    who.set_flag(aff, true);
                }
            }
            _ => {}
        }
    }
    Ok(())
}

pub fn apply_or_infer_cures(
    who: &mut AgentState,
    cures: Vec<FType>,
    after: &Vec<AetObservation>,
) -> Result<(), String> {
    let mut found_cures = Vec::new();
    for observation in after.iter() {
        match observation {
            AetObservation::Cured(aff_name) => {
                if let Some(aff) = FType::from_name(&aff_name) {
                    who.set_flag(aff, false);
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
                    who.set_flag(def, false);
                    found_cures.push(def);
                }
            }
            _ => {}
        }
    }
    if found_cures.len() == 0 {
        remove_in_order(cures)(who);
    }
    Ok(())
}

pub fn apply_or_infer_cure(
    who: &mut AgentState,
    cure: &SimpleCure,
    after: &Vec<AetObservation>,
) -> Result<Vec<FType>, String> {
    let mut found_cures = Vec::new();
    if let Some(AetObservation::Cured(aff_name)) = after.get(1) {
        if let Some(aff) = FType::from_name(&aff_name) {
            who.set_flag(aff, false);
            found_cures.push(aff);
        }
    } else if let Some(AetObservation::DiscernedCure(_you, aff_name)) = after.get(1) {
        if let Some(aff) = FType::from_name(&aff_name) {
            who.set_flag(aff, false);
            if aff == FType::Void {
                who.set_flag(FType::Weakvoid, true);
            }
            found_cures.push(aff);
        }
    } else if let Some(AetObservation::Stripped(def_name)) = after.get(1) {
        if let Some(def) = FType::from_name(&def_name) {
            who.set_flag(def, false);
            found_cures.push(def);
        }
    } else {
        match cure {
            SimpleCure::Pill(pill_name) => {
                if who.is(FType::Anorexia) {
                    who.set_flag(FType::Anorexia, false);
                    warn!("Missed Anorexia cure!");
                }
                if pill_name == "anabiotic" {
                } else if let Some(order) = PILL_CURE_ORDERS.get(pill_name) {
                    let cured = top_aff(who, order.to_vec());
                    remove_in_order(order.to_vec())(who);
                    if cured == Some(FType::ThinBlood) {
                        who.clear_relapses();
                    }
                } else if let Some(defence) = PILL_DEFENCES.get(pill_name) {
                    who.set_flag(*defence, true);
                } else {
                    return Err(format!("Could not find pill {}", pill_name));
                }
            }
            SimpleCure::Salve(salve_name, salve_loc) => {
                if who.is(FType::Slickness) {
                    who.set_flag(FType::Slickness, false);
                    warn!("Missed Slickness cure!");
                }
                if salve_name == "caloric" {
                    if who.some(CALORIC_TORSO_ORDER.to_vec()) {
                        remove_in_order(CALORIC_TORSO_ORDER.to_vec())(who);
                    } else {
                        who.set_flag(FType::Insulation, true);
                    }
                } else if salve_name == "mass" {
                    who.set_flag(FType::Density, true);
                } else if salve_name == "restoration" {
                    let limb = get_limb_damage(salve_loc)?;
                    who.set_restoring(limb);
                } else if let Some(order) =
                    SALVE_CURE_ORDERS.get(&(salve_name.to_string(), salve_loc.to_string()))
                {
                    if let Ok(limb) = get_limb_damage(salve_loc) {
                        if !who.limb_damage.damaged(limb) {
                            remove_in_order(order.to_vec())(who);
                        } else {
                            println!("{} fizzled on {}", salve_name, salve_loc);
                        }
                    } else {
                        remove_in_order(order.to_vec())(who);
                    }
                } else {
                    return Err(format!("Could not find {} on {}", salve_name, salve_loc));
                }
            }
            SimpleCure::Smoke(herb_name) => {
                if who.is(FType::Asthma) {
                    who.set_flag(FType::Asthma, false);
                    warn!("Missed Asthma cure!");
                }
                if let Some(order) = SMOKE_CURE_ORDERS.get(herb_name) {
                    remove_in_order(order.to_vec())(who);
                } else if herb_name == "reishi" {
                    who.set_balance(BType::Rebounding, 6.25);
                } else {
                    return Err(format!("Could not find smoke {}", herb_name));
                }
            } // _ => {}
        }
    }
    Ok(found_cures)
}

pub fn for_agent(agent_states: &mut AetTimelineState, target: &String, act: fn(&mut AgentState)) {
    let mut you = agent_states.get_agent(target);
    act(&mut you);
    agent_states.set_agent(target, you);
}

pub fn for_agent_closure(
    agent_states: &mut AetTimelineState,
    target: &String,
    act: Box<dyn Fn(&mut AgentState)>,
) {
    let mut you = agent_states.get_agent(target);
    act(&mut you);
    agent_states.set_agent(target, you);
}

pub fn attack_afflictions(
    agent_states: &mut AetTimelineState,
    target: &String,
    affs: Vec<FType>,
    after: &Vec<AetObservation>,
) {
    if attack_hit(after) {
        let mut you = agent_states.get_agent(target);
        for aff in affs.iter() {
            you.set_flag(*aff, true);
        }
        agent_states.set_agent(target, you);
    }
}

pub fn attack_strip(
    agent_states: &mut AetTimelineState,
    target: &String,
    defs: Vec<FType>,
    after: &Vec<AetObservation>,
) {
    if attack_hit(after) {
        let mut you = agent_states.get_agent(target);
        for def in defs.iter() {
            you.set_flag(*def, false);
        }
        agent_states.set_agent(target, you);
    }
}

pub fn attack_strip_or_afflict(
    agent_states: &mut AetTimelineState,
    target: &String,
    aff_defs: Vec<FType>,
    after: &Vec<AetObservation>,
) {
    if attack_hit(after) {
        let mut you = agent_states.get_agent(target);
        for aff_def in aff_defs.iter() {
            if !aff_def.is_affliction() && you.is(*aff_def) {
                you.set_flag(*aff_def, false);
                break;
            } else if aff_def.is_affliction() && !you.is(*aff_def) {
                you.set_flag(*aff_def, true);
                break;
            }
        }
        agent_states.set_agent(target, you);
    }
}
