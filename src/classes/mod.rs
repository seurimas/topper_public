use crate::curatives::MENTAL_AFFLICTIONS;
use crate::observables::*;
use crate::timeline::*;
use crate::topper::*;
use crate::types::*;
use std::collections::HashMap;
pub mod carnifex;
pub mod syssin;
pub mod zealot;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
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
        _ => None,
    }
}

pub fn has_special_cure(class: &Class, affliction: FType) -> bool {
    match (affliction, class) {
        (FType::Asthma, Class::Monk) => true,
        (FType::Asthma, Class::Syssin) => true,
        (FType::Paresis, Class::Zealot) => true,
        _ => false,
    }
}

pub fn is_affected_by(class: &Class, affliction: FType) -> bool {
    match (affliction, class) {
        (FType::Clumsiness, Class::Syssin) => true,
        (FType::Clumsiness, Class::Templar) => true,
        (FType::Clumsiness, Class::Carnifex) => true,
        (FType::Clumsiness, Class::Wayfarer) => true,
        (FType::Clumsiness, Class::Monk) => true,
        (FType::Peace, Class::Luminary) => true,
        (FType::Disfigurement, Class::Archivists) => true,
        _ => false,
    }
}

pub fn handle_sent(command: &String, agent_states: &mut TimelineState) {
    syssin::handle_sent(command, agent_states);
}

pub fn get_attack(topper: &mut Topper, target: &String, strategy: &String) -> String {
    if let Some(class) = topper.get_timeline().get_my_class() {
        match class {
            Class::Zealot => zealot::get_attack(topper, target, strategy),
            Class::Syssin => syssin::get_attack(&topper.get_timeline(), target, strategy),
            _ => syssin::get_attack(&topper.get_timeline(), target, strategy),
        }
    } else {
        syssin::get_attack(&topper.get_timeline(), target, strategy)
    }
}

pub fn get_needed_parry(
    timeline: &Timeline,
    me: &String,
    target: &String,
    strategy: &String,
) -> Option<LType> {
    if let Ok(parry) = get_preferred_parry(timeline, me, target, strategy) {
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

pub fn get_preferred_parry(
    timeline: &Timeline,
    me: &String,
    target: &String,
    strategy: &String,
) -> Result<LType, String> {
    if let Some(class) = timeline.get_class(target) {
        match class {
            Class::Zealot => zealot::get_preferred_parry(timeline, me, target, strategy),
            Class::Wayfarer => Ok(LType::RightLegDamage),
            _ => Ok(LType::HeadDamage),
        }
    } else {
        Ok(LType::HeadDamage)
    }
}

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut TimelineState,
    before: &Vec<Observation>,
    after: &Vec<Observation>,
) -> Result<(), String> {
    match combat_action.category.as_ref() {
        "Subterfuge" | "Assassination" | "Hypnosis" => {
            syssin::handle_combat_action(combat_action, agent_states, before, after)
        }
        "Savagery" | "Deathlore" | "Warhound" => {
            carnifex::handle_combat_action(combat_action, agent_states, before, after)
        }
        "Purification" | "Zeal" | "Psionics" => {
            zealot::handle_combat_action(combat_action, agent_states, before, after)
        }
        "Survival" => match combat_action.skill.as_ref() {
            "Focus" => {
                let mut me = agent_states.get_agent(&combat_action.caster);
                let mut duration = 5.0;
                if me.is(FType::NumbedSkin) {
                    duration += 5.0;
                }
                if me.is(FType::Laxity) {
                    duration += 2.0;
                }
                apply_or_infer_cures(&mut me, MENTAL_AFFLICTIONS.to_vec(), after)?;
                apply_or_infer_balance(&mut me, (BType::Focus, duration), after);
                agent_states.set_agent(&combat_action.caster, me);
                Ok(())
            }
            _ => Ok(()),
        },
        "Tattoos" => match combat_action.skill.as_ref() {
            "Shield" => {
                let mut me = agent_states.get_agent(&combat_action.caster);
                me.set_flag(FType::Shielded, true);
                apply_or_infer_balance(&mut me, (BType::Equil, 4.0), after);
                agent_states.set_agent(&combat_action.caster, me);
                Ok(())
            }
            "Tree" => {
                let mut me = agent_states.get_agent(&combat_action.caster);
                let mut duration = 10.0;
                if me.is(FType::NumbedSkin) {
                    duration += 5.0;
                }
                if me.is(FType::Laxity) {
                    duration += 2.0;
                }
                if me.is(FType::Paresis) || me.is(FType::Paralysis) {
                    me.set_flag(FType::Paresis, false);
                    me.set_flag(FType::Paralysis, false);
                    warn!("Missed Paresis cure!");
                }
                apply_or_infer_balance(&mut me, (BType::Tree, duration), after);
                agent_states.set_agent(&combat_action.caster, me);
                Ok(())
            }
            _ => Ok(()),
        },
        "Hunting" => match combat_action.skill.as_ref() {
            "Regenerate" => {
                let mut me = agent_states.get_agent(&combat_action.caster);
                me.regenerate();
                apply_or_infer_balance(&mut me, (BType::Regenerate, 15.0), after);
                agent_states.set_agent(&combat_action.caster, me);
                Ok(())
            }
            "Renew" => {
                let mut me = agent_states.get_agent(&combat_action.caster);
                apply_or_infer_balance(&mut me, (BType::Renew, 20.0), after);
                agent_states.set_agent(&combat_action.caster, me);
                Ok(())
            }
            _ => Ok(()),
        },
        _ => Ok(()),
    }
}

lazy_static! {
    static ref AFFLICT_VENOMS: HashMap<FType, &'static str> = {
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

/*
pub fn get_venom(affliction: FType) -> Option<&'static str> {
    if let Some(venom) = AFFLICT_VENOMS.get(&affliction) {
        Some(*venom)
    } else {
        None
    }
}
*/

pub fn get_venoms(afflictions: Vec<FType>, count: usize, target: &AgentState) -> Vec<&'static str> {
    let mut venoms = Vec::new();
    for affliction in afflictions.iter() {
        if !target.is(*affliction) & !(*affliction == FType::Paresis && target.is(FType::Paralysis))
        {
            if let Some(venom) = AFFLICT_VENOMS.get(affliction) {
                venoms.push(*venom);
            }
            if count == venoms.len() {
                break;
            }
        }
    }
    venoms
}

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
            ready.push(buffer);
        }
    }
}

pub fn remove_through(you: &mut AgentState, end: FType, order: &Vec<FType>) {
    for flag in order.iter() {
        you.set_flag(*flag, false);
        if *flag == end {
            break;
        }
    }
}

pub struct RestoreAction {
    caster: String,
}

impl RestoreAction {
    pub fn new(caster: String) -> Self {
        RestoreAction { caster }
    }
}

impl ActiveTransition for RestoreAction {
    fn simulate(&self, timeline: &Timeline) -> Vec<ProbableEvent> {
        vec![ProbableEvent::new(
            vec![Observation::CombatAction(CombatAction {
                caster: self.caster.clone(),
                category: "Survival".to_string(),
                skill: "Restoration".to_string(),
                target: "".to_string(),
                annotation: "".to_string(),
            })],
            1,
        )]
    }
    fn act(&self, timeline: &Timeline) -> ActivateResult {
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
    fn simulate(&self, timeline: &Timeline) -> Vec<ProbableEvent> {
        ProbableEvent::certain(vec![Observation::Parry(
            self.caster.clone(),
            self.limb.to_string(),
        )])
    }
    fn act(&self, timeline: &Timeline) -> ActivateResult {
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
    fn simulate(&self, timeline: &Timeline) -> Vec<ProbableEvent> {
        vec![ProbableEvent::new(
            vec![Observation::CombatAction(CombatAction {
                caster: self.caster.clone(),
                category: "Survival".to_string(),
                skill: "Regenerate".to_string(),
                target: "".to_string(),
                annotation: "".to_string(),
            })],
            1,
        )]
    }
    fn act(&self, timeline: &Timeline) -> ActivateResult {
        Ok(format!("regenerate"))
    }
}
