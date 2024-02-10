use crate::bt::BehaviorController;
use crate::curatives::{SafetyAlert, MENTAL_AFFLICTIONS, RANDOM_CURES};
use crate::db::AetDatabaseModule;
use crate::non_agent::AetNonAgent;
use crate::observables::*;
use crate::timeline::*;
use crate::types::*;
use num_enum::TryFromPrimitive;
use regex::Regex;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
pub mod archivist;
pub mod ascendril;
pub mod bard;
pub mod carnifex;
pub mod group;
pub mod indorani;
pub mod infiltrator;
pub mod lords;
pub mod luminary;
pub mod mirrors;
pub mod monk;
pub mod praenomen;
pub mod predator;
pub mod sciomancer;
pub mod sentinel;
pub mod shaman;
pub mod shapeshifter;
pub mod templar;
pub mod teradrim;
pub mod wayfarer;
pub mod zealot;
use serde::{Deserialize, Serialize};

use self::archivist::get_archivist_alerts;
use self::mirrors::normalize_combat_action;

pub struct FitnessAction {
    pub caster: String,
}

impl FitnessAction {
    pub fn new(caster: String) -> Self {
        FitnessAction { caster }
    }
}

impl ActiveTransition for FitnessAction {
    fn simulate(&self, _timeline: &AetTimeline) -> Vec<ProbableEvent> {
        Vec::new()
    }
    fn act(&self, _timeline: &AetTimeline) -> ActivateResult {
        Ok("fitness".to_string())
    }
}

#[derive(
    Debug, Serialize, Deserialize, Clone, Copy, Display, TryFromPrimitive, PartialEq, Eq, Hash,
)]
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
    Infiltrator,
    Shapeshifter,
    Wayfarer,
    Bard,
    Predator,
    Lord,
    // Mirrors
    Revenant,     // Templar
    Warden,       // Carnifex
    Earthcaller,  // Luminary
    Oneiromancer, // Indorani
    Alchemist,    // Shaman
    Tidesage,     // Teradrim
    Akkari,       // Praenomen
    Ravager,      // Zealot
    Runecarver,   // Sciomancer
    Bloodborn,    // Ascendril
    Voidseer,     // Archivist
    Executor,     // Sentinel
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
            "Infiltrator" => Some(Class::Infiltrator),
            // Unaffiliated
            "Shapeshifter" => Some(Class::Shapeshifter),
            "Wayfarer" => Some(Class::Wayfarer),
            "Bard" => Some(Class::Bard),
            "Predator" => Some(Class::Predator),
            "Titan Lord" => Some(Class::Lord),
            "Chaos Lord" => Some(Class::Lord),
            "Lord" => Some(Class::Lord),
            // Mirrors
            "Revenant" => Some(Class::Revenant),
            "Warden" => Some(Class::Warden),
            "Earthcaller" => Some(Class::Earthcaller),
            "Oneiromancer" => Some(Class::Oneiromancer),
            "Tidesage" => Some(Class::Tidesage),
            "Alchemist" => Some(Class::Alchemist),
            "Akkari" => Some(Class::Akkari),
            "Ravager" => Some(Class::Ravager),
            "Runecarver" => Some(Class::Runecarver),
            "Bloodborn" => Some(Class::Bloodborn),
            "Voidseer" => Some(Class::Voidseer),
            "Executor" => Some(Class::Executor),
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
            Class::Infiltrator => "Infiltrator",
            // Neutral
            Class::Shapeshifter => "Shapeshifter",
            Class::Wayfarer => "Wayfarer",
            Class::Bard => "Bard",
            Class::Predator => "Predator",
            Class::Lord => "Lord",
            // Mirrors
            Class::Revenant => "Revenant",
            Class::Warden => "Warden",
            Class::Earthcaller => "Earthcaller",
            Class::Oneiromancer => "Oneiromancer",
            Class::Tidesage => "Tidesage",
            Class::Alchemist => "Alchemist",
            Class::Akkari => "Akkari",
            Class::Ravager => "Ravager",
            Class::Runecarver => "Runecarver",
            Class::Bloodborn => "Bloodborn",
            Class::Voidseer => "Voidseer",
            Class::Executor => "Executor",
            _ => "Unknown",
        }
    }
    pub fn is_mirror(&self) -> bool {
        match self {
            Class::Revenant
            | Class::Warden
            | Class::Earthcaller
            | Class::Oneiromancer
            | Class::Tidesage
            | Class::Alchemist
            | Class::Akkari
            | Class::Ravager
            | Class::Runecarver
            | Class::Bloodborn
            | Class::Voidseer
            | Class::Executor => true,
            _ => false,
        }
    }
    pub fn normal(&self) -> Self {
        match self {
            Class::Revenant => Class::Templar,
            Class::Warden => Class::Carnifex,
            Class::Earthcaller => Class::Luminary,
            Class::Oneiromancer => Class::Indorani,
            Class::Tidesage => Class::Teradrim,
            Class::Alchemist => Class::Shaman,
            Class::Akkari => Class::Praenomen,
            Class::Ravager => Class::Zealot,
            Class::Runecarver => Class::Sciomancer,
            Class::Bloodborn => Class::Ascendril,
            Class::Voidseer => Class::Archivists,
            Class::Executor => Class::Sentinel,
            _ => self.clone(),
        }
    }

    pub fn get_safety_alerts(&self, agent: &AgentState) -> Vec<SafetyAlert> {
        match self {
            Class::Archivists => get_archivist_alerts(agent),
            _ => Vec::new(),
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
        "Assassination" | "Subterfuge" | "Hypnosis" => Some(Class::Infiltrator),
        // Unaffiliated
        "Ferality" | "Shapeshifting" | "Vocalizing" => Some(Class::Shapeshifter),
        "Tenacity" | "Wayfaring" | "Fury" => Some(Class::Wayfarer),
        "Weaving" | "Performance" | "Songcalling" => Some(Class::Bard),
        "Knifeplay" | "Predation" | "Beastmastery" => Some(Class::Predator),
        "Titan" | "Chaos" => Some(Class::Lord),
        // Mirrors
        "Riving" | "Chirography" | "Manifestation" => Some(Class::Revenant),
        "Warding" | "Ancestry" | "Communion" => Some(Class::Warden),
        "Oneiromancy" | "Hyalincuru" | "Contracts" => Some(Class::Oneiromancer),
        "Subjugation" | "Apocalyptia" | "Tectonics" => Some(Class::Earthcaller),
        "Alchemy" | "Experimentation" | "Botany" => Some(Class::Alchemist),
        "Wavebreaking" | "Synthesis" | "Inundation" => Some(Class::Tidesage),
        "Ascendance" | "Dictum" | "Discipline" => Some(Class::Akkari),
        "Brutality" | "Ravaging" | "Egotism" => Some(Class::Ravager),
        "Malediction" | "Runecarving" | "Sporulation" => Some(Class::Runecarver),
        "Humourism" | "Esoterica" | "Hematurgy" => Some(Class::Bloodborn),
        "Enlightenment" | "Cultivation" | "Voidgazing" => Some(Class::Voidseer),
        "Bladedancing" | "Artifice" | "Subversion" => Some(Class::Executor),
        _ => None,
    }
}

pub fn has_special_cure(class: &Class, affliction: FType) -> bool {
    if class.is_mirror() {
        return has_special_cure(&class.normal(), affliction);
    }
    match (affliction, class) {
        (FType::Asthma, Class::Monk) => true,
        (FType::Asthma, Class::Infiltrator) => true,
        (FType::Paresis, Class::Zealot) => true,
        _ => false,
    }
}

pub fn is_affected_by(class: Class, affliction: FType) -> bool {
    if class.is_mirror() {
        return is_affected_by(class.normal(), affliction);
    }
    match (affliction, class) {
        (FType::Clumsiness, Class::Infiltrator) => true,
        (FType::Clumsiness, Class::Bard) => true,
        (FType::Clumsiness, Class::Templar) => true,
        (FType::Clumsiness, Class::Sentinel) => true,
        (FType::Clumsiness, Class::Wayfarer) => true,
        (FType::Clumsiness, Class::Teradrim) => true,
        (FType::Clumsiness, Class::Predator) => true,
        (FType::Clumsiness, Class::Zealot) => true,
        (FType::Weariness, Class::Carnifex) => true,
        (FType::Weariness, Class::Sciomancer) => true,
        (FType::Peace, Class::Luminary) => true,
        (FType::Disfigurement, Class::Sentinel) => true,
        (FType::Disfigurement, Class::Carnifex) => true,
        (FType::Disfigurement, Class::Luminary) => true,
        (FType::Disfigurement, Class::Indorani) => true,
        (FType::Disfigurement, Class::Teradrim) => true,
        (FType::Lethargy, Class::Bard) => true,
        (FType::Lethargy, Class::Infiltrator) => true,
        (FType::Lethargy, Class::Sentinel) => true,
        (FType::Lethargy, Class::Carnifex) => true,
        (FType::Lethargy, Class::Templar) => true,
        // Class cures get blocked!
        (FType::Paresis, Class::Indorani) => true,
        (FType::Paresis, Class::Sciomancer) => true,
        (FType::Paresis, Class::Ascendril) => true,
        (FType::Paresis, Class::Teradrim) => true,
        _ => false,
    }
}

lazy_static! {
    static ref DIAGNOSING: Regex = Regex::new(r"diagnose").unwrap();
    pub static ref DIAGNOSE_TIME: String = "DIAGNOSE_TIME".to_string();
    pub static ref DIAGNOSE_FRESHNESS: f32 = 5.0;
}

pub fn handle_sent(command: &String, agent_states: &mut AetTimelineState) {
    infiltrator::handle_sent(command, agent_states);
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
            Class::Infiltrator => infiltrator::get_attack(timeline, target, strategy, db),
            Class::Bard => bard::get_attack(timeline, target, strategy, db),
            Class::Zealot => zealot::get_attack(timeline, target, strategy, db),
            Class::Predator => predator::get_attack(timeline, target, strategy, db),
            Class::Monk => monk::get_attack(timeline, target, strategy, db),
            _ => infiltrator::get_attack(timeline, target, strategy, db),
        }
    } else {
        infiltrator::get_attack(timeline, target, strategy, db)
    }
}

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut AetTimelineState,
    before: &Vec<AetObservation>,
    after: &Vec<AetObservation>,
    db: Option<&impl AetDatabaseModule>,
) -> Result<(), String> {
    if let Some(class) = get_skill_class(&combat_action.category) {
        if class.is_mirror() {
            return handle_combat_action(
                &combat_action.normalized(),
                agent_states,
                before,
                after,
                db,
            );
        }
    }
    if !combat_action.target.is_empty() && !combat_action.caster.eq(&combat_action.target) {
        let my_room = agent_states.borrow_me().room_id;
        let my_elevation = agent_states.borrow_me().elevation;
        for_agent(agent_states, &combat_action.target, &|me| {
            me.register_hit(Some(&combat_action.caster));
            me.room_id = my_room;
            me.elevation = my_elevation;
            if me.is(FType::Pacifism) && me.balanced(BType::Pacifism) {
                me.toggle_flag(FType::Pacifism, false);
            }
        });
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
            infiltrator::handle_combat_action(combat_action, agent_states, before, after)
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
        "Weaving" | "Performance" | "Songcalling" => {
            bard::handle_combat_action(combat_action, agent_states, before, after)
        }
        "Knifeplay" | "Predation" | "Beastmastery" => {
            predator::handle_combat_action(combat_action, agent_states, before, after, db)
        }
        "Purification" | "Zeal" | "Psionics" => {
            zealot::handle_combat_action(combat_action, agent_states, before, after)
        }
        "Survival" => match combat_action.skill.as_ref() {
            "Focus" => {
                let observations = after.clone();
                let first_person = agent_states.me.eq(&combat_action.caster);
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &move |me: &mut AgentState| {
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
                    },
                );
                Ok(())
            }
            _ => Ok(()),
        },
        "Tattoos" => match combat_action.skill.as_ref() {
            "Shield" => {
                let observations = after.clone();
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &|me: &mut AgentState| {
                        me.set_flag(FType::Shielded, true);
                    },
                );
                if !combat_action.annotation.eq("proc") {
                    let observations = after.clone();
                    for_agent(
                        agent_states,
                        &combat_action.caster,
                        &move |me: &mut AgentState| {
                            apply_or_infer_balance(me, (BType::Equil, 4.0), &observations);
                        },
                    );
                }
                Ok(())
            }
            "Hammer" => {
                for_agent(agent_states, &combat_action.target, &|you| {
                    you.set_flag(FType::Shielded, false);
                });
                Ok(())
            }
            "Tree" => {
                let observations = after.clone();
                let perspective = agent_states.get_perspective(&combat_action);
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &move |me: &mut AgentState| {
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
                    },
                );
                Ok(())
            }
            _ => Ok(()),
        },
        "Enchantment" => match combat_action.skill.as_ref() {
            "Icewall" => {
                let observations = after.clone();
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &move |me: &mut AgentState| {
                        me.observe_flag(FType::Superstition, false);
                    },
                );
                Ok(())
            }
            _ => Ok(()),
        },
        "Relic" => match combat_action.skill.as_ref() {
            "Tailstrike" => {
                attack_afflictions(
                    agent_states,
                    &combat_action.target,
                    vec![FType::Slickness],
                    after,
                );
                Ok(())
            }
            _ => Ok(()),
        },
        "Hunting" => match combat_action.skill.as_ref() {
            "Fitness" => {
                let observations = after.clone();
                let first_person = agent_states.me.eq(&combat_action.caster);
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &move |me: &mut AgentState| {
                        apply_or_infer_cures(me, vec![FType::Asthma], &observations, first_person);
                        apply_or_infer_balance(me, (BType::Fitness, 20.0), &observations);
                    },
                );
                Ok(())
            }
            "Regenerate" => {
                let observations = after.clone();
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &move |me: &mut AgentState| {
                        me.regenerate();
                        apply_or_infer_balance(me, (BType::Regenerate, 15.0), &observations);
                    },
                );
                Ok(())
            }
            "Meditate" => {
                let observations = after.clone();
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &move |me: &mut AgentState| {
                        me.observe_flag(FType::Impatience, false);
                    },
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
                for_agent(agent_states, &combat_action.caster, &move |you| {
                    you.tick_flag_up(FType::Ablaze);
                    if you.get_count(FType::Ablaze) < minimum_stacks {
                        you.set_count(FType::Ablaze, minimum_stacks);
                    } else if you.get_count(FType::Ablaze) > maximum_stacks {
                        you.set_count(FType::Ablaze, maximum_stacks);
                    }
                });
                Ok(())
            }
            "dizziness" => {
                for_agent(agent_states, &combat_action.caster, &|you| {
                    you.toggle_flag(FType::Fallen, true);
                    you.observe_flag(FType::Dizziness, true);
                });
                Ok(())
            }
            "stupidity" => {
                for_agent(agent_states, &combat_action.caster, &|you| {
                    you.toggle_flag(FType::Fallen, true);
                    you.observe_flag(FType::Stupidity, true);
                });
                Ok(())
            }
            "narcolepsy" => {
                for_agent(agent_states, &combat_action.caster, &|you| {
                    you.observe_flag(FType::Narcolepsy, true);
                    if you.is(FType::Insomnia) {
                        you.toggle_flag(FType::Insomnia, false);
                    } else if !you.is(FType::Asleep) {
                        you.toggle_flag(FType::Asleep, true);
                    }
                });
                Ok(())
            }
            "vomiting" => {
                for_agent(agent_states, &combat_action.caster, &|you| {
                    you.observe_flag(FType::Vomiting, true);
                });
                Ok(())
            }
            "self_loathing" => {
                let flings = combat_action.annotation.eq("flings");
                let count = if combat_action.annotation.eq("first") {
                    2
                } else if combat_action.annotation.eq("furrow") {
                    3
                } else {
                    // Fling restarts it
                    1
                };
                for_agent(agent_states, &combat_action.caster, &|you| {
                    you.observe_flag_count(FType::SelfLoathing, count);
                    if count == 2 {
                        you.set_balance(BType::SelfLoathing, 8.)
                    } else if count == 3 {
                        you.set_balance(BType::SelfLoathing, 4.)
                    } else if count == 1 {
                        you.set_balance(BType::SelfLoathing, 24.)
                    }
                    if you.bard_board.emotion_state.primary == Some(Emotion::Fear) {
                        you.set_flag(FType::Worrywart, true);
                    }
                    if flings {
                        you.toggle_flag(FType::Fallen, true);
                    }
                });
                Ok(())
            }
            "broken legs" => {
                for_agent(agent_states, &combat_action.caster, &|you| {
                    you.toggle_flag(FType::Fallen, true);
                    you.observe_flag(FType::LeftLegCrippled, true);
                    you.observe_flag(FType::RightLegCrippled, true);
                });
                Ok(())
            }
            _ => Ok(()),
        },
        _ => Ok(()),
    }
}

#[derive(Debug, Display, Serialize, Deserialize, PartialEq, Clone)]
pub enum LockType {
    // Just asthma/slickness/anorexia
    Soft,
    // Asthma, slickness, anorexia, paralysis, and stupidity
    Buffered,
    // Asthma, slickness, anorexia, paralysis, and focus blocked
    Hard,
    // Only the venom affs
    HardVenom,
}

impl LockType {
    pub fn affs(&self) -> Vec<FType> {
        match self {
            LockType::Soft => vec![FType::Anorexia, FType::Slickness, FType::Asthma],
            LockType::Buffered => vec![
                FType::Anorexia,
                FType::Slickness,
                FType::Stupidity,
                FType::Asthma,
                FType::Paresis,
            ],
            LockType::HardVenom => vec![
                FType::Anorexia,
                FType::Slickness,
                FType::Asthma,
                FType::Paresis,
            ],
            LockType::Hard => vec![
                FType::Anorexia,
                FType::Slickness,
                FType::Asthma,
                FType::Paresis,
                FType::Impatience,
            ],
        }
    }

    pub fn affs_to_lock(&self, agent: &AgentState) -> usize {
        let needed_affs = self.affs();
        let affs_on_target = agent.affs_count(&needed_affs);
        needed_affs.len() - affs_on_target
    }
}

pub type VenomType = &'static str;

pub fn get_stack<'s>(
    timeline: &AetTimeline,
    attack_class: &'static str,
    target: &String,
    strategy: &String,
    db: Option<&impl AetDatabaseModule>,
) -> Option<Vec<VenomPlan>> {
    let stack = if strategy.eq("class") {
        if let Some(class) = db.and_then(|db| db.get_class(target)) {
            class.normal().to_str().to_string()
        } else {
            "aggro".to_string()
        }
    } else {
        strategy.clone()
    };
    let mut stack_name = format!("{}_{}", attack_class, strategy);
    get_stack_from_file(&attack_class.to_string(), &stack).or_else(|| {
        db.and_then(|db| {
            db.get_venom_plan(&stack_name).or_else(|| {
                get_stack_from_file(&attack_class.to_string(), &"aggro".to_string())
                    .or_else(|| db.get_venom_plan(&format!("{}_aggro", attack_class)))
            })
        })
    })
}

pub static mut LOAD_STACK_FUNC: Option<fn(&String, &String) -> String> = None;

lazy_static! {
    pub static ref LOADED_VENOM_PLANS: RwLock<HashMap<String, Option<Vec<VenomPlan>>>> =
        { RwLock::new(HashMap::new()) };
}

pub fn clear_aff_stacks() {
    let mut stacks = LOADED_VENOM_PLANS.write().unwrap();
    stacks.clear();
}

pub fn get_stack_from_file(class: &String, stack_name: &String) -> Option<Vec<VenomPlan>> {
    {
        let stacks = LOADED_VENOM_PLANS.read().unwrap();
        if let Some(stack) = stacks.get(&format!("{}_{}", class, stack_name)) {
            return stack.clone();
        }
    }
    {
        let mut trees = LOADED_VENOM_PLANS.write().unwrap();
        let stack_json = unsafe { LOAD_STACK_FUNC.unwrap()(class, stack_name) };
        println!(
            "Loading {}'s {} stack ({})",
            class,
            stack_name,
            stack_json.len()
        );
        match serde_json::from_str::<Vec<VenomPlan>>(&stack_json) {
            Ok(stack_def) => {
                trees.insert(format!("{}_{}", class, stack_name), Some(stack_def.clone()));
                Some(stack_def)
            }
            Err(err) => {
                println!("Failed to load {}/{}: {:?}", class, stack_name, err);
                trees.insert(format!("{}_{}", class, stack_name), None);
                None
            }
        }
    }
}

fn get_controller(
    attack_class: &'static str,
    me: &String,
    target: &String,
    timeline: &topper_core::timeline::Timeline<AetObservation, AetPrompt, AgentState, AetNonAgent>,
    strategy: &String,
    db: Option<&impl AetDatabaseModule>,
) -> BehaviorController {
    BehaviorController {
        plan: ActionPlan::new(me),
        target: Some(target.clone()),
        aff_priorities: get_stack(timeline, attack_class, target, strategy, db),
        allies: timeline
            .state
            .non_agent_states
            .get(&format!("{}_allies", me))
            .map(|ally_list| {
                if let AetNonAgent::Players(ally_list) = ally_list {
                    let mut ally_aggros = HashMap::new();
                    let my_room = timeline.state.borrow_me().room_id;
                    for ally in ally_list {
                        let ally_state = timeline.state.borrow_agent(ally);
                        if ally_state.room_id == my_room {
                            ally_aggros.insert(ally.clone(), ally_state.get_aggro());
                        }
                    }
                    ally_aggros
                } else {
                    panic!("Non-player list in allies spot!")
                }
            })
            .unwrap_or_default(),
        ..Default::default()
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
        val.insert(FType::LeftArmCrippled, "epteth");
        val.insert(FType::RightArmCrippled, "epteth");
        val.insert(FType::Sensitivity, "prefarar");
        val.insert(FType::Disfigurement, "monkshood");
        val.insert(FType::Vomiting, "euphorbia");
        val.insert(FType::Deafness, "colocasia");
        // val.insert(FType::CureBlind, "oculus");
        val.insert(FType::Haemophilia, "hepafarin");
        val.insert(FType::Stuttering, "jalk");
        val.insert(FType::Weariness, "vernalius");
        val.insert(FType::RightLegCrippled, "epseth");
        val.insert(FType::LeftLegCrippled, "epseth");
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
    pub static ref AFF_TO_AFF_MAP: HashMap<FType, FType> = {
        let mut val = HashMap::new();
        for aff in FType::afflictions() {
            val.insert(aff, aff);
        }
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

pub fn is_susceptible(target: &AgentState, affliction: &FType, afflicted: &Vec<FType>) -> bool {
    !target.is(*affliction)
        && !(*affliction == FType::Paresis && target.is(FType::Paralysis))
        && !afflicted.contains(affliction)
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
    OnTree(FType),
    OffTree(FType),
    OnFocus(FType),
    OffFocus(FType),
    OnFitness(FType),
    OffFitness(FType),
    OneOf(FType, FType),
    IfDo(FType, Box<VenomPlan>),
    IfNotDo(FType, Box<VenomPlan>),
    IfClassHates(FType, Box<VenomPlan>),
    IfNotClassHates(FType, Box<VenomPlan>),
}

impl VenomPlan {
    pub fn affliction(&self) -> FType {
        match self {
            VenomPlan::Stick(aff)
            | VenomPlan::OnTree(aff)
            | VenomPlan::OffTree(aff)
            | VenomPlan::OnFocus(aff)
            | VenomPlan::OffFocus(aff)
            | VenomPlan::OnFitness(aff)
            | VenomPlan::OffFitness(aff)
            | VenomPlan::OneOf(aff, _) => *aff,
            VenomPlan::IfDo(_pred, plan) | VenomPlan::IfNotDo(_pred, plan) => plan.affliction(),
            VenomPlan::IfClassHates(aff, plan) | VenomPlan::IfNotClassHates(aff, plan) => {
                plan.affliction()
            }
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
        pub fn $add_name(
            item: &VenomPlan,
            target: &AgentState,
            venoms: &mut Vec<$returned>,
            afflicted: &mut Vec<FType>,
        ) {
            match item {
                VenomPlan::Stick(aff) => {
                    if is_susceptible(target, aff, afflicted) {
                        if let Some(venom) = $stack.get(aff) {
                            venoms.insert(0, *venom);
                            afflicted.push(*aff);
                        }
                    }
                }
                VenomPlan::OnTree(aff) => {
                    if (target.balanced(BType::Tree) || target.get_balance(BType::Tree) < 1.5)
                        && is_susceptible(target, aff, afflicted)
                    {
                        if let Some(venom) = $stack.get(aff) {
                            venoms.insert(0, *venom);
                            afflicted.push(*aff);
                        }
                    }
                }
                VenomPlan::OffTree(aff) => {
                    if !(target.balanced(BType::Tree) || target.get_balance(BType::Tree) < 1.5)
                        && is_susceptible(target, aff, afflicted)
                    {
                        if let Some(venom) = $stack.get(aff) {
                            venoms.insert(0, *venom);
                            afflicted.push(*aff);
                        }
                    }
                }
                VenomPlan::OnFocus(aff) => {
                    if (target.balanced(BType::Focus) || target.get_balance(BType::Focus) < 1.5)
                        && is_susceptible(target, aff, afflicted)
                    {
                        if let Some(venom) = $stack.get(aff) {
                            venoms.insert(0, *venom);
                            afflicted.push(*aff);
                        }
                    }
                }
                VenomPlan::OffFocus(aff) => {
                    if !(target.balanced(BType::Focus) || target.get_balance(BType::Focus) < 1.5)
                        && is_susceptible(target, aff, afflicted)
                    {
                        if let Some(venom) = $stack.get(aff) {
                            venoms.insert(0, *venom);
                            afflicted.push(*aff);
                        }
                    }
                }
                VenomPlan::OnFitness(aff) => {
                    if (target.balanced(BType::Fitness) || target.get_balance(BType::Fitness) < 1.5)
                        && is_susceptible(target, aff, afflicted)
                    {
                        if let Some(venom) = $stack.get(aff) {
                            venoms.insert(0, *venom);
                            afflicted.push(*aff);
                        }
                    }
                }
                VenomPlan::OffFitness(aff) => {
                    if !(target.balanced(BType::Fitness)
                        || target.get_balance(BType::Fitness) < 1.5)
                        && is_susceptible(target, aff, afflicted)
                    {
                        if let Some(venom) = $stack.get(aff) {
                            venoms.insert(0, *venom);
                            afflicted.push(*aff);
                        }
                    }
                }
                VenomPlan::OneOf(priority, secondary) => {
                    if let (Some(priority_venom), Some(secondary_venom)) =
                        ($stack.get(priority), $stack.get(secondary))
                    {
                        if !is_susceptible(target, priority, afflicted)
                            && is_susceptible(target, secondary, afflicted)
                        {
                            venoms.insert(0, *secondary_venom);
                            afflicted.push(*secondary);
                        } else if is_susceptible(target, priority, afflicted) {
                            venoms.insert(0, *priority_venom);
                            afflicted.push(*priority);
                        }
                    }
                }
                VenomPlan::IfDo(when, plan) => {
                    if !is_susceptible(target, when, afflicted) {
                        $add_name(plan, target, venoms, afflicted);
                    }
                }
                VenomPlan::IfNotDo(when, plan) => {
                    if is_susceptible(target, when, afflicted) {
                        $add_name(plan, target, venoms, afflicted);
                    }
                }
                VenomPlan::IfClassHates(when, plan) => {
                    if let Some(class) = target.get_normalized_class() {
                        if is_affected_by(class, *when) {
                            $add_name(plan, target, venoms, afflicted);
                        }
                    }
                }
                VenomPlan::IfNotClassHates(when, plan) => {
                    if let Some(class) = target.get_normalized_class() {
                        if !is_affected_by(class, *when) {
                            $add_name(plan, target, venoms, afflicted);
                        }
                    } else {
                        $add_name(plan, target, venoms, afflicted);
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
            let mut afflicted = Vec::new();
            for item in plan.iter() {
                $add_name(item, target, &mut venoms, &mut afflicted);
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
affliction_plan_stacker!(add_aff_from_plan, get_affs_from_plan, AFF_TO_AFF_MAP, FType);

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
