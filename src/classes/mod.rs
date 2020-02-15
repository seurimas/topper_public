use crate::actions::*;
use crate::curatives::MENTAL_AFFLICTIONS;
use crate::io::*;
use crate::timeline::*;
use crate::types::*;
use std::collections::HashMap;
pub mod carnifex;
pub mod syssin;
pub mod zealot;

pub fn handle_sent(command: &String, agent_states: &mut TimelineState) {
    syssin::handle_sent(command, agent_states);
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
                let duration = if me.is(FType::MentalFatigue) {
                    10.0
                } else {
                    5.0
                };
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
                me.set_flag(FType::Shield, true);
                apply_or_infer_balance(&mut me, (BType::Equil, 4.0), after);
                agent_states.set_agent(&combat_action.caster, me);
                Ok(())
            }
            "Tree" => {
                let mut me = agent_states.get_agent(&combat_action.caster);
                let duration = if me.is(FType::NumbedSkin) { 15.0 } else { 10.0 };
                apply_or_infer_balance(&mut me, (BType::Tree, duration), after);
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
        val.insert(FType::Pacifism, "ouabain");
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

pub fn get_venom(affliction: FType) -> Option<&'static str> {
    if let Some(venom) = AFFLICT_VENOMS.get(&affliction) {
        Some(*venom)
    } else {
        None
    }
}

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

pub fn get_attack(topper: &Topper, target: &String, strategy: &String) -> String {
    syssin::get_attack(topper, target, strategy)
}
