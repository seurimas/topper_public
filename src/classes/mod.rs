use crate::actions::*;
use crate::timeline::*;
use crate::types::*;
use std::collections::HashMap;
pub mod syssin;

pub fn get_offensive_actions(class: Option<&String>) -> Vec<StateAction> {
    vec![]
}

pub fn handle_combat_action(combat_action: &CombatAction, agent_states: &mut TimelineState) {
    match combat_action.category.as_ref() {
        "Subterfuge" | "Assassination" | "Hypnosis" => {
            syssin::handle_combat_action(combat_action, agent_states)
        }
        _ => {}
    }
}

const EPTETH_A: (&'static str, FType) = ("epteth", FType::BrokenLeftLeg);
const EPTETH_B: (&'static str, FType) = ("epteth", FType::BrokenRightLeg);
const EPSETH_A: (&'static str, FType) = ("epseth", FType::BrokenLeftArm);
const EPSETH_B: (&'static str, FType) = ("epseth", FType::BrokenRightArm);
const SLIKE: (&'static str, FType) = ("slike", FType::Anorexia);
const KALMIA: (&'static str, FType) = ("kalmia", FType::Slickness);
const JALK: (&'static str, FType) = ("jalk", FType::Asthma);

const VENOMS: [(&'static str, FType); 7] =
    [EPTETH_A, EPTETH_B, EPSETH_A, EPSETH_B, SLIKE, KALMIA, JALK];

lazy_static! {
    static ref AFFLICT_VENOMS: HashMap<FType, &'static str> = {
        let mut val = HashMap::new();
        val.insert(FType::Clumsiness, "xentio");
        val.insert(FType::Blindness, "oleander");
        val.insert(FType::Recklessness, "eurypteria");
        val.insert(FType::Asthma, "kalmia");
        val.insert(FType::Shyness, "digitalis");
        val.insert(FType::Allergies, "darkshade");
        val.insert(FType::Paralysis, "curare");
        val.insert(FType::BrokenLeftArm, "epteth");
        val.insert(FType::BrokenRightArm, "epteth");
        val.insert(FType::Sensitivity, "prefarar");
        val.insert(FType::Disfigurement, "monkshood");
        val.insert(FType::Vomiting, "euphorbia");
        val.insert(FType::Deafness, "colocasia");
        // val.insert(FType::CureBlind, "oculus");
        val.insert(FType::Haemophilia, "hepafarin");
        val.insert(FType::Stuttering, "jalk");
        val.insert(FType::Weariness, "vernalius");
        val.insert(FType::BrokenRightLeg, "epseth");
        val.insert(FType::BrokenLeftLeg, "epseth");
        val.insert(FType::Dizziness, "larkspur");
        val.insert(FType::Anorexia, "slike");
        val.insert(FType::Voyria, "voyria");
        val.insert(FType::Deadening, "vardrax");
        val.insert(FType::Squelched, "selarnia");
        val.insert(FType::Slickness, "gecko");
        val.insert(FType::ThinBlood, "scytherus");
        val.insert(FType::Pacifism, "ouabain");
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
        val.insert("curare".into(), FType::Paralysis);
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
        val.insert("ouabain".into(), FType::Pacifism);
        val
    };
}

pub fn get_venoms(afflictions: Vec<FType>, count: usize, target: &AgentState) -> Vec<&'static str> {
    let mut venoms = Vec::new();
    for affliction in afflictions.iter() {
        if !target.is(*affliction) {
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
