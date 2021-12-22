use crate::curatives::{MENTAL_AFFLICTIONS, RANDOM_CURES};
use crate::db::AetDatabaseModule;
use crate::observables::*;
use crate::timeline::*;
use crate::types::*;
use num_enum::TryFromPrimitive;
use regex::Regex;
use std::collections::HashMap;
pub mod archivist;
pub mod ascendril;
pub mod carnifex;
pub mod indorani;
pub mod lords;
pub mod luminary;
pub mod mirrors;
pub mod monk;
pub mod praenomen;
pub mod sciomancer;
pub mod sentinel;
pub mod shaman;
pub mod shapeshifter;
pub mod syssin;
pub mod templar;
pub mod teradrim;
pub mod wayfarer;
pub mod zealot;
use serde::{Deserialize, Serialize};

use self::mirrors::normalize_combat_action;

#[derive(Debug, Serialize, Clone, Display, TryFromPrimitive, PartialEq)]
#[repr(u8)]
pub enum Class {
    Carnifex,
    Indorani,
    Praenomen,
    Teradrim,
    Monk,
    Sentinel,
    Shaman,
    Ascendril,
    Luminary,
    Templar,
    Zealot,
    Archivists,
    Sciomancer,
    Syssin,
    Shapeshifter,
    Wayfarer,
    Lord,
    // Mirrors
    Revenant,     // Templar
    Warden,       // Warden
    Earthcaller,  // Luminary
    Oneiromancer, // Indorani
}

impl Class {
    pub fn from_str(string: &str) -> Option<Class> {
        match string.as_ref() {
            // Bloodloch
            "Carnifex" => Some(Class::Carnifex),
            "Indorani" => Some(Class::Indorani),
            "Praenomen" => Some(Class::Praenomen),
            "Teradrim" => Some(Class::Teradrim),
            // Duiran
            "Monk" => Some(Class::Monk),
            "Sentinel" => Some(Class::Sentinel),
            "Shaman" => Some(Class::Shaman),
            // Enorian
            "Ascendril" => Some(Class::Ascendril),
            "Luminary" => Some(Class::Luminary),
            "Templar" => Some(Class::Templar),
            "Zealot" => Some(Class::Zealot),
            // Spinesreach
            "Archivist" => Some(Class::Archivists),
            "Sciomancer" => Some(Class::Sciomancer),
            "Syssin" => Some(Class::Syssin),
            // Unaffiliated
            "Shapeshifter" => Some(Class::Shapeshifter),
            "Wayfarer" => Some(Class::Wayfarer),
            "Titan Lord" => Some(Class::Lord),
            "Chaos Lord" => Some(Class::Lord),
            "Lord" => Some(Class::Lord),
            // Mirrors
            "Revenant" => Some(Class::Revenant),
            "Warden" => Some(Class::Warden),
            "Earthcaller" => Some(Class::Earthcaller),
            "Oneiromancer" => Some(Class::Oneiromancer),
            _ => None,
        }
    }
    pub fn to_str(&self) -> &str {
        match self {
            // Bloodloch
            Class::Carnifex => "Carnifex",
            Class::Indorani => "Indorani",
            Class::Praenomen => "Praenomen",
            Class::Teradrim => "Teradrim",
            // Duiran
            Class::Monk => "Monk",
            Class::Sentinel => "Sentinel",
            Class::Shaman => "Shaman",
            // Enorian
            Class::Templar => "Templar",
            Class::Zealot => "Zealot",
            Class::Luminary => "Luminary",
            Class::Ascendril => "Ascendril",
            // Spinesreach
            Class::Archivists => "Archivists",
            Class::Sciomancer => "Sciomancer",
            Class::Syssin => "Syssin",
            // Bloodloch
            Class::Shapeshifter => "Shapeshifter",
            Class::Wayfarer => "Wayfarer",
            Class::Lord => "Lord",
            // Mirrors
            Class::Revenant => "Revenant",
            Class::Warden => "Warden",
            Class::Earthcaller => "Earthcaller",
            Class::Oneiromancer => "Oneiromancer",
            _ => "Unknown",
        }
    }
    pub fn is_mirror(&self) -> bool {
        match self {
            Class::Revenant | Class::Warden | Class::Earthcaller | Class::Oneiromancer => true,
            _ => false,
        }
    }
    pub fn normal(&self) -> Self {
        match self {
            Class::Revenant => Class::Templar,
            Class::Warden => Class::Carnifex,
            Class::Earthcaller => Class::Luminary,
            Class::Oneiromancer => Class::Indorani,
            _ => self.clone(),
        }
    }
}

pub fn get_skill_class(category: &String) -> Option<Class> {
    match category.as_ref() {
        // Bloodloch
        "Savagery" | "Deathlore" | "Warhounds" => Some(Class::Carnifex),
        "Necromancy" | "Tarot" | "Domination" => Some(Class::Indorani),
        "Corpus" | "Mentis" | "Sanguis" => Some(Class::Praenomen),
        "Terramancy" | "Animation" | "Desiccation" => Some(Class::Teradrim),
        // Duiran
        "Tekura" | "Kaido" | "Telepathy" => Some(Class::Monk),
        "Dhuriv" | "Woodlore" | "Tracking" => Some(Class::Sentinel),
        "Primality" | "Shamanism" | "Naturalism" => Some(Class::Shaman),
        // Enorian
        "Elemancy" | "Arcanism" | "Thaumaturgy" => Some(Class::Ascendril),
        "Spirituality" | "Devotion" | "Illumination" => Some(Class::Luminary),
        "Battlefury" | "Righteousness" | "Bladefire" => Some(Class::Templar),
        "Zeal" | "Purification" | "Psionics" => Some(Class::Zealot),
        // Spinesreach
        "Geometrics" | "Numerology" | "Bioessence" => Some(Class::Archivists),
        "Sciomancy" | "Sorcery" | "Gravitation" => Some(Class::Sciomancer),
        "Assassination" | "Subterfuge" | "Hypnosis" => Some(Class::Syssin),
        // Unaffiliated
        "Ferality" | "Shapeshifting" | "Vocalizing" => Some(Class::Shapeshifter),
        "Tenacity" | "Wayfaring" | "Fury" => Some(Class::Wayfarer),
        "Titan" | "Chaos" => Some(Class::Lord),
        // Mirrors
        "Riving" | "Chirography" | "Manifestation" => Some(Class::Revenant),
        "Warding" | "Ancestry" | "Communion" => Some(Class::Warden),
        "Oneiromancy" | "Hyalincuru" | "Contracts" => Some(Class::Oneiromancer),
        "Subjugation" | "Apocalyptia" | "Tectonics" => Some(Class::Earthcaller),
        _ => None,
    }
}

pub fn has_special_cure(class: &Class, affliction: FType) -> bool {
    if class.is_mirror() {
        return has_special_cure(&class.normal(), affliction);
    }
    match (affliction, class) {
        (FType::Asthma, Class::Monk) => true,
        (FType::Asthma, Class::Syssin) => true,
        (FType::Paresis, Class::Zealot) => true,
        _ => false,
    }
}

pub fn is_affected_by(class: &Class, affliction: FType) -> bool {
    if class.is_mirror() {
        return is_affected_by(&class.normal(), affliction);
    }
    match (affliction, class) {
        (FType::Clumsiness, Class::Syssin) => true,
        (FType::Clumsiness, Class::Templar) => true,
        (FType::Clumsiness, Class::Carnifex) => true,
        (FType::Clumsiness, Class::Sentinel) => true,
        (FType::Clumsiness, Class::Wayfarer) => true,
        (FType::Clumsiness, Class::Teradrim) => true,
        (FType::Clumsiness, Class::Zealot) => true,
        (FType::Peace, Class::Luminary) => true,
        (FType::Disfigurement, Class::Sentinel) => true,
        (FType::Disfigurement, Class::Carnifex) => true,
        (FType::Disfigurement, Class::Luminary) => true,
        (FType::Disfigurement, Class::Indorani) => true,
        (FType::Disfigurement, Class::Teradrim) => true,
        (FType::Lethargy, Class::Syssin) => true,
        (FType::Lethargy, Class::Sentinel) => true,
        (FType::Lethargy, Class::Carnifex) => true,
        (FType::Lethargy, Class::Templar) => true,
        _ => false,
    }
}

lazy_static! {
    static ref DIAGNOSING: Regex = Regex::new(r"diagnose").unwrap();
    pub static ref DIAGNOSE_TIME: String = "DIAGNOSE_TIME".to_string();
    pub static ref DIAGNOSE_FRESHNESS: f32 = 5.0;
}

pub fn handle_sent(command: &String, agent_states: &mut AetTimelineState) {
    syssin::handle_sent(command, agent_states);
    if let Some(captures) = DIAGNOSING.captures(command) {
        let me = agent_states.me.clone();
        let time = agent_states.time;
        agent_states.add_player_hint(&me, &DIAGNOSE_TIME.to_string(), time.to_string());
    }
}

pub fn get_attack(
    timeline: &AetTimeline,
    me: &String,
    target: &String,
    strategy: &String,
    db: Option<&impl AetDatabaseModule>,
) -> String {
    if let Some(class) = db.and_then(|db| db.get_class(me)) {
        match class {
            Class::Sentinel => sentinel::get_attack(timeline, target, strategy, db),
            Class::Syssin => syssin::get_attack(timeline, target, strategy, db),
            Class::Zealot => zealot::get_attack(timeline, target, strategy, db),
            _ => syssin::get_attack(timeline, target, strategy, db),
        }
    } else {
        syssin::get_attack(timeline, target, strategy, db)
    }
}

pub fn get_needed_parry(
    timeline: &AetTimeline,
    me: &String,
    target: &String,
    strategy: &String,
    db: Option<&impl AetDatabaseModule>,
) -> Option<LType> {
    if let Ok(parry) = get_preferred_parry(timeline, me, target, strategy, db) {
        let me = timeline.state.borrow_agent(me);
        if let Some(current) = me.parrying {
            if current == parry {
                None
            } else {
                Some(parry)
            }
        } else {
            Some(parry)
        }
    } else {
        None
    }
}

pub fn get_restore_parry(timeline: &AetTimeline, me: &String) -> Option<LType> {
    let me = timeline.state.borrow_agent(me);
    if let Some((restoring, _duration, _regenerating)) = me.get_restoring() {
        if restoring == LType::LeftLegDamage {
            Some(LType::RightLegDamage)
        } else if restoring == LType::RightLegDamage {
            Some(LType::LeftLegDamage)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn get_top_parry(timeline: &AetTimeline, me: &String) -> Option<LType> {
    let me = timeline.state.borrow_agent(me);
    let mut top_non_restoring = None;
    for limb in LIMBS.to_vec() {
        let limb_state = me.get_limb_state(limb);
        if let Some((top_damage, _top_limb)) = top_non_restoring {
            if !limb_state.is_restoring && limb_state.damage > top_damage {
                top_non_restoring = Some((limb_state.damage, limb));
            }
        } else if !limb_state.is_restoring && limb_state.damage > 8.0 {
            top_non_restoring = Some((limb_state.damage, limb));
        }
    }
    top_non_restoring.map(|top| top.1)
}

pub fn get_preferred_parry(
    timeline: &AetTimeline,
    me: &String,
    target: &String,
    strategy: &String,
    db: Option<&impl AetDatabaseModule>,
) -> Result<LType, String> {
    if let Some(parry) = get_restore_parry(timeline, me) {
        Ok(parry)
    } else if let Some(mut class) = db.and_then(|db| db.get_class(target)) {
        if class.is_mirror() {
            class = class.normal();
        }
        match class {
            Class::Shapeshifter => {
                let myself = timeline.state.borrow_agent(me);
                let limbs_state = myself.get_limbs_state();
                if limbs_state.left_leg.broken && !limbs_state.left_leg.damaged {
                    Ok(LType::LeftLegDamage)
                } else if limbs_state.right_leg.broken && !limbs_state.right_leg.damaged {
                    Ok(LType::RightLegDamage)
                } else if limbs_state.left_arm.broken && !limbs_state.left_arm.damaged {
                    Ok(LType::LeftArmDamage)
                } else if limbs_state.right_arm.broken && !limbs_state.right_arm.damaged {
                    Ok(LType::RightArmDamage)
                } else {
                    Ok(get_top_parry(timeline, me).unwrap_or(LType::HeadDamage))
                }
            }
            Class::Zealot => {
                let them = timeline.state.borrow_agent(target);
                match them.channel_state {
                    ChannelState::Heelrush(limb, _) => Ok(limb),
                    _ => Ok(get_top_parry(timeline, me).unwrap_or(LType::TorsoDamage)),
                }
            }
            Class::Sentinel => {
                let myself = timeline.state.borrow_agent(me);
                if myself.is(FType::Heartflutter) {
                    Ok(LType::TorsoDamage)
                } else if !myself.is(FType::Impatience) {
                    Ok(LType::HeadDamage)
                } else {
                    Ok(get_top_parry(timeline, me).unwrap_or(LType::HeadDamage))
                }
            }
            Class::Wayfarer => {
                let myself = timeline.state.borrow_agent(me);
                let limbs_state = myself.get_limbs_state();
                if limbs_state.left_leg.damaged
                    || limbs_state.right_leg.damaged
                    || limbs_state.left_arm.damaged
                    || limbs_state.right_arm.damaged
                {
                    if limbs_state.head.damage > 20.0 {
                        Ok(LType::HeadDamage)
                    } else {
                        Ok(get_top_parry(timeline, me).unwrap_or(LType::HeadDamage))
                    }
                } else {
                    Ok(get_top_parry(timeline, me).unwrap_or(LType::HeadDamage))
                }
            }
            _ => Ok(get_top_parry(timeline, me).unwrap_or(LType::HeadDamage)),
        }
    } else {
        Ok(get_top_parry(timeline, me).unwrap_or(LType::HeadDamage))
    }
}

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut AetTimelineState,
    before: &Vec<AetObservation>,
    after: &Vec<AetObservation>,
) -> Result<(), String> {
    if let Some(class) = get_skill_class(&combat_action.category) {
        if class.is_mirror() {
            return handle_combat_action(&combat_action.normalized(), agent_states, before, after);
        }
    }
    match combat_action.category.as_ref() {
        "Geometrics" | "Numerology" | "Bioessence" => {
            archivist::handle_combat_action(combat_action, agent_states, before, after)
        }
        "Elemancy" | "Arcanism" | "Thaumaturgy" => {
            ascendril::handle_combat_action(combat_action, agent_states, before, after)
        }
        "Savagery" | "Deathlore" | "Warhound" => {
            carnifex::handle_combat_action(combat_action, agent_states, before, after)
        }
        "Necromancy" | "Tarot" | "Domination" => {
            indorani::handle_combat_action(combat_action, agent_states, before, after)
        }
        "Chaos" | "Titan" => {
            lords::handle_combat_action(combat_action, agent_states, before, after)
        }
        "Spirituality" | "Devotion" | "Illumination" => {
            luminary::handle_combat_action(combat_action, agent_states, before, after)
        }
        "Tekura" | "Kaido" | "Telepathy" => {
            monk::handle_combat_action(combat_action, agent_states, before, after)
        }
        "Corpus" | "Mentis" | "Sanguis" => {
            praenomen::handle_combat_action(combat_action, agent_states, before, after)
        }
        "Sciomancy" | "Sorcery" | "Gravitation" => {
            sciomancer::handle_combat_action(combat_action, agent_states, before, after)
        }
        "Dhuriv" | "Woodlore" | "Tracking" => {
            sentinel::handle_combat_action(combat_action, agent_states, before, after)
        }
        "Primality" | "Shamanism" | "Naturalism" => {
            shaman::handle_combat_action(combat_action, agent_states, before, after)
        }
        "Ferality" | "Shapeshifting" | "Vocalizing" => {
            shapeshifter::handle_combat_action(combat_action, agent_states, before, after)
        }
        "Subterfuge" | "Assassination" | "Hypnosis" => {
            syssin::handle_combat_action(combat_action, agent_states, before, after)
        }
        "Battlefury" | "Righteousness" | "Bladefire" => {
            templar::handle_combat_action(combat_action, agent_states, before, after)
        }
        "Terramancy" | "Animation" | "Desiccation" => {
            teradrim::handle_combat_action(combat_action, agent_states, before, after)
        }
        "Tenacity" | "Wayfaring" | "Fury" => {
            wayfarer::handle_combat_action(combat_action, agent_states, before, after)
        }
        "Purification" | "Zeal" | "Psionics" => {
            zealot::handle_combat_action(combat_action, agent_states, before, after)
        }
        "Survival" => match combat_action.skill.as_ref() {
            "Focus" => {
                let observations = after.clone();
                let first_person = agent_states.me.eq(&combat_action.caster);
                for_agent_closure(
                    agent_states,
                    &combat_action.caster,
                    Box::new(move |me| {
                        let mut duration = 5.0;
                        if me.is(FType::MentalFatigue) {
                            duration += 5.0;
                        }
                        if me.is(FType::Laxity) {
                            duration += 2.0;
                        }
                        apply_or_infer_cures(
                            me,
                            MENTAL_AFFLICTIONS.to_vec(),
                            &observations,
                            first_person,
                        );
                        apply_or_infer_balance(me, (BType::Focus, duration), &observations);
                    }),
                );
                Ok(())
            }
            _ => Ok(()),
        },
        "Tattoos" => match combat_action.skill.as_ref() {
            "Shield" => {
                let observations = after.clone();
                for_agent(agent_states, &combat_action.caster, |me| {
                    me.set_flag(FType::Shielded, true);
                });
                if !combat_action.annotation.eq("proc") {
                    let observations = after.clone();
                    for_agent_closure(
                        agent_states,
                        &combat_action.caster,
                        Box::new(move |me| {
                            apply_or_infer_balance(me, (BType::Equil, 4.0), &observations);
                        }),
                    );
                }
                Ok(())
            }
            "Hammer" => {
                for_agent(agent_states, &combat_action.target, |you| {
                    you.set_flag(FType::Shielded, false);
                });
                Ok(())
            }
            "Tree" => {
                let observations = after.clone();
                let perspective = agent_states.get_perspective(&combat_action);
                for_agent_closure(
                    agent_states,
                    &combat_action.caster,
                    Box::new(move |me| {
                        let mut duration = 10.0;
                        if me.is(FType::NumbedSkin) {
                            duration += 5.0;
                        }
                        if me.is(FType::Laxity) {
                            duration += 2.0;
                        }
                        me.observe_flag(FType::Paresis, false);
                        me.observe_flag(FType::Paralysis, false);
                        apply_or_infer_balance(me, (BType::Tree, duration), &observations);
                        apply_or_strike_random_cure(
                            me,
                            &observations,
                            perspective,
                            (1, RANDOM_CURES.to_vec()),
                        );
                    }),
                );
                Ok(())
            }
            _ => Ok(()),
        },
        "Enchantment" => match combat_action.skill.as_ref() {
            "Icewall" => {
                let observations = after.clone();
                for_agent_closure(
                    agent_states,
                    &combat_action.caster,
                    Box::new(move |me| {
                        me.observe_flag(FType::Superstition, false);
                    }),
                );
                Ok(())
            }
            _ => Ok(()),
        },
        "Hunting" => match combat_action.skill.as_ref() {
            "Regenerate" => {
                let observations = after.clone();
                for_agent_closure(
                    agent_states,
                    &combat_action.caster,
                    Box::new(move |me| {
                        me.regenerate();
                        apply_or_infer_balance(me, (BType::Regenerate, 15.0), &observations);
                    }),
                );
                Ok(())
            }
            "Renew" => {
                let observations = after.clone();
                let perspective = agent_states.get_perspective(&combat_action);
                for_agent_closure(
                    agent_states,
                    &combat_action.caster,
                    Box::new(move |me| {
                        me.observe_flag(FType::Impairment, false);
                        apply_or_infer_balance(me, (BType::Renew, 20.0), &observations);
                        apply_or_strike_random_cure(
                            me,
                            &observations,
                            perspective,
                            (1, RANDOM_CURES.to_vec()),
                        );
                    }),
                );
                Ok(())
            }
            "Meditate" => {
                let observations = after.clone();
                for_agent_closure(
                    agent_states,
                    &combat_action.caster,
                    Box::new(move |me| {
                        me.observe_flag(FType::Impatience, false);
                    }),
                );
                Ok(())
            }
            _ => Ok(()),
        },
        "Aff" => match combat_action.skill.as_ref() {
            "ablaze" => {
                let (minimum_stacks, maximum_stacks) = match combat_action.annotation.as_ref() {
                    "Flames" => (2, 4),
                    "Hot flames" => (5, 8),
                    "White-hot flames" => (9, 12),
                    "Deadly flames" => (13, 120),
                    _ => (1, 120),
                };
                for_agent_closure(
                    agent_states,
                    &combat_action.caster,
                    Box::new(move |you| {
                        you.tick_flag_up(FType::Ablaze);
                        if you.get_count(FType::Ablaze) < minimum_stacks {
                            you.set_count(FType::Ablaze, minimum_stacks);
                        } else if you.get_count(FType::Ablaze) > maximum_stacks {
                            you.set_count(FType::Ablaze, maximum_stacks);
                        }
                    }),
                );
                Ok(())
            }
            "dizziness" => {
                for_agent(agent_states, &combat_action.caster, |you| {
                    you.toggle_flag(FType::Fallen, true);
                    you.observe_flag(FType::Dizziness, true);
                });
                Ok(())
            }
            "stupidity" => {
                for_agent(agent_states, &combat_action.caster, |you| {
                    you.toggle_flag(FType::Fallen, true);
                    you.observe_flag(FType::Stupidity, true);
                });
                Ok(())
            }
            "vomiting" => {
                for_agent(agent_states, &combat_action.caster, |you| {
                    you.observe_flag(FType::Vomiting, true);
                });
                Ok(())
            }
            "broken legs" => {
                for_agent(agent_states, &combat_action.caster, |you| {
                    you.toggle_flag(FType::Fallen, true);
                    you.observe_flag(FType::LeftLegBroken, true);
                    you.observe_flag(FType::RightLegBroken, true);
                });
                Ok(())
            }
            _ => Ok(()),
        },
        _ => Ok(()),
    }
}

lazy_static! {
    pub static ref AFFLICT_VENOMS: HashMap<FType, &'static str> = {
        let mut val = HashMap::new();
        val.insert(FType::Clumsiness, "xentio");
        val.insert(FType::Blindness, "oleander");
        val.insert(FType::Recklessness, "eurypteria");
        val.insert(FType::Asthma, "kalmia");
        val.insert(FType::Shyness, "digitalis");
        val.insert(FType::Allergies, "darkshade");
        val.insert(FType::Paresis, "curare");
        val.insert(FType::LeftArmBroken, "epteth");
        val.insert(FType::RightArmBroken, "epteth");
        val.insert(FType::Sensitivity, "prefarar");
        val.insert(FType::Disfigurement, "monkshood");
        val.insert(FType::Vomiting, "euphorbia");
        val.insert(FType::Deafness, "colocasia");
        // val.insert(FType::CureBlind, "oculus");
        val.insert(FType::Haemophilia, "hepafarin");
        val.insert(FType::Stuttering, "jalk");
        val.insert(FType::Weariness, "vernalius");
        val.insert(FType::RightLegBroken, "epseth");
        val.insert(FType::LeftLegBroken, "epseth");
        val.insert(FType::Dizziness, "larkspur");
        val.insert(FType::Anorexia, "slike");
        val.insert(FType::Voyria, "voyria");
        val.insert(FType::Deadening, "vardrax");
        val.insert(FType::Squelched, "selarnia");
        val.insert(FType::Slickness, "gecko");
        val.insert(FType::ThinBlood, "scytherus");
        val.insert(FType::Peace, "ouabain");
        val.insert(FType::Stupidity, "aconite");
        val
    };
}

lazy_static! {
    pub static ref VENOM_AFFLICTS: HashMap<String, FType> = {
        let mut val = HashMap::new();
        val.insert("xentio".into(), FType::Clumsiness);
        val.insert("oleander".into(), FType::Blindness);
        val.insert("eurypteria".into(), FType::Recklessness);
        val.insert("kalmia".into(), FType::Asthma);
        val.insert("digitalis".into(), FType::Shyness);
        val.insert("darkshade".into(), FType::Allergies);
        val.insert("curare".into(), FType::Paresis);
        val.insert("prefarar".into(), FType::Sensitivity);
        val.insert("monkshood".into(), FType::Disfigurement);
        val.insert("euphorbia".into(), FType::Vomiting);
        val.insert("colocasia".into(), FType::Deafness);
        val.insert("hepafarin".into(), FType::Haemophilia);
        val.insert("jalk".into(), FType::Stuttering);
        val.insert("vernalius".into(), FType::Weariness);
        val.insert("larkspur".into(), FType::Dizziness);
        val.insert("slike".into(), FType::Anorexia);
        val.insert("voyria".into(), FType::Voyria);
        val.insert("vardrax".into(), FType::Deadening);
        val.insert("selarnia".into(), FType::Squelched);
        val.insert("gecko".into(), FType::Slickness);
        val.insert("scytherus".into(), FType::ThinBlood);
        val.insert("ouabain".into(), FType::Peace);
        val.insert("aconite".into(), FType::Stupidity);
        val
    };
}

pub fn remove_through(you: &mut AgentState, end: FType, order: &Vec<FType>) {
    for flag in order.iter() {
        you.set_flag(*flag, false);
        if *flag == end {
            break;
        }
    }
}

pub fn is_susceptible(target: &AgentState, affliction: &FType) -> bool {
    !target.is(*affliction) && !(*affliction == FType::Paresis && target.is(FType::Paralysis))
}

#[macro_export]
macro_rules! affliction_stacker {
    ($name:ident, $stack:expr, $returned:ty) => {
        pub fn $name(afflictions: Vec<FType>, count: usize, target: &AgentState) -> Vec<$returned> {
            let mut venoms = Vec::new();
            for affliction in afflictions.iter() {
                if !target.is(*affliction)
                    & !(*affliction == FType::Paresis && target.is(FType::Paralysis))
                {
                    if let Some(venom) = $stack.get(affliction) {
                        venoms.push(*venom);
                    }
                    if count == venoms.len() {
                        break;
                    }
                }
            }
            venoms
        }
    };
}
affliction_stacker!(get_venoms, AFFLICT_VENOMS, &'static str);

pub fn add_buffers<'s>(ready: &mut Vec<&'s str>, buffers: &Vec<&'s str>) {
    for buffer in buffers.iter() {
        let mut found = false;
        for venom in ready.iter() {
            if venom == buffer {
                found = true;
                break;
            }
        }
        if !found {
            ready.insert(0, buffer);
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum VenomPlan {
    Stick(FType),
    StickThisIfThat(FType, FType),
    OnTree(FType),
    OneOf(FType, FType),
    IfDo(FType, Box<VenomPlan>),
    IfNotDo(FType, Box<VenomPlan>),
}

impl VenomPlan {
    pub fn affliction(&self) -> FType {
        match self {
            VenomPlan::Stick(aff)
            | VenomPlan::StickThisIfThat(aff, _)
            | VenomPlan::OnTree(aff)
            | VenomPlan::OneOf(aff, _) => *aff,
            VenomPlan::IfDo(_pred, plan) | VenomPlan::IfNotDo(_pred, plan) => plan.affliction(),
        }
    }
}

pub fn get_simple_plan(afflictions: Vec<FType>) -> Vec<VenomPlan> {
    afflictions
        .iter()
        .map(|aff| VenomPlan::Stick(*aff))
        .collect()
}

#[macro_export]
macro_rules! affliction_plan_stacker {
    ($add_name:ident, $get_name:ident, $stack:expr, $returned:ty) => {
        pub fn $add_name(item: &VenomPlan, target: &AgentState, venoms: &mut Vec<$returned>) {
            match item {
                VenomPlan::Stick(aff) => {
                    if is_susceptible(target, aff) {
                        if let Some(venom) = $stack.get(aff) {
                            venoms.insert(0, *venom);
                        }
                    }
                }
                VenomPlan::StickThisIfThat(this, that) => {
                    if target.is(*this) && is_susceptible(target, that) {
                        if let Some(venom) = $stack.get(that) {
                            venoms.insert(0, *venom);
                        }
                    }
                }
                VenomPlan::OnTree(aff) => {
                    if (target.balanced(BType::Tree) || target.get_balance(BType::Tree) < 1.5)
                        && is_susceptible(target, aff)
                    {
                        if let Some(venom) = $stack.get(aff) {
                            venoms.insert(0, *venom);
                        }
                    }
                }
                VenomPlan::OneOf(priority, secondary) => {
                    if let (Some(priority_venom), Some(secondary_venom)) =
                        ($stack.get(priority), $stack.get(secondary))
                    {
                        if target.is(*priority) && !target.is(*secondary) {
                            venoms.insert(0, *secondary_venom);
                        } else if !target.is(*priority) {
                            venoms.insert(0, *priority_venom);
                        }
                    }
                }
                VenomPlan::IfDo(when, plan) => {
                    if target.is(*when) {
                        $add_name(plan, target, venoms);
                    }
                }
                VenomPlan::IfNotDo(when, plan) => {
                    if !target.is(*when) {
                        $add_name(plan, target, venoms);
                    }
                }
            }
        }

        pub fn $get_name(
            plan: &Vec<VenomPlan>,
            count: usize,
            target: &AgentState,
        ) -> Vec<$returned> {
            let mut venoms = Vec::new();
            for item in plan.iter() {
                $add_name(item, target, &mut venoms);
                if count == venoms.len() {
                    break;
                }
            }
            venoms
        }
    };
}
affliction_plan_stacker!(
    add_venom_from_plan,
    get_venoms_from_plan,
    AFFLICT_VENOMS,
    &'static str
);

pub struct RestoreAction {
    caster: String,
}

impl RestoreAction {
    pub fn new(caster: String) -> Self {
        RestoreAction { caster }
    }
}

impl ActiveTransition for RestoreAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        vec![ProbableEvent::new(
            vec![AetObservation::CombatAction(CombatAction {
                caster: self.caster.clone(),
                category: "Survival".to_string(),
                skill: "Restoration".to_string(),
                target: "".to_string(),
                annotation: "".to_string(),
            })],
            1,
        )]
    }
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(format!("restore"))
    }
}

pub struct ParryAction {
    caster: String,
    limb: LType,
}

impl ParryAction {
    pub fn new(caster: String, limb: LType) -> Self {
        ParryAction { caster, limb }
    }
}

impl ActiveTransition for ParryAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        ProbableEvent::certain(vec![AetObservation::Parry(
            self.caster.clone(),
            self.limb.to_string(),
        )])
    }
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(format!("parry {}", self.limb.to_string()))
    }
}

pub struct RegenerateAction {
    caster: String,
}

impl RegenerateAction {
    pub fn new(caster: String) -> Self {
        RegenerateAction { caster }
    }
}

impl ActiveTransition for RegenerateAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        vec![ProbableEvent::new(
            vec![AetObservation::CombatAction(CombatAction {
                caster: self.caster.clone(),
                category: "Survival".to_string(),
                skill: "Regenerate".to_string(),
                target: "".to_string(),
                annotation: "".to_string(),
            })],
            1,
        )]
    }
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(format!("regenerate"))
    }
}

pub struct ShieldAction {
    caster: String,
}

impl ShieldAction {
    pub fn new(caster: &str) -> Self {
        ShieldAction {
            caster: caster.to_string(),
        }
    }
}

impl ActiveTransition for ShieldAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        vec![ProbableEvent::new(
            vec![AetObservation::CombatAction(CombatAction {
                caster: self.caster.clone(),
                category: "Tattoos".to_string(),
                skill: "Shield".to_string(),
                target: "".to_string(),
                annotation: "".to_string(),
            })],
            1,
        )]
    }
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(format!("stand;;touch shield"))
    }
}

pub struct Action {
    command: String,
}

impl Action {
    pub fn new(command: String) -> Self {
        Action { command }
    }
}

impl ActiveTransition for Action {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        vec![]
    }
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(self.command.clone())
    }
}
