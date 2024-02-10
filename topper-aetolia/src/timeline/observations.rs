use crate::timeline::types::*;
use crate::timeline::*;
use regex::Regex;
use std::fs::read_dir;
use std::fs::{DirEntry, File};
use std::io::BufReader;
use topper_core::observations::*;

#[cfg(test)]
#[path = "./tests/observations_tests.rs"]
mod observer_tests;

lazy_static! {
    static ref DAMAGE_TYPE: Regex = Regex::new("crippled|broken|mangled|dislocated").unwrap();
    static ref DAMAGE_LIMB: Regex =
        Regex::new("left leg|right leg|left arm|right arm|head|torso").unwrap();
}

fn parse_discern(discerned: String) -> String {
    let damaged_type = DAMAGE_TYPE.captures(&discerned);
    let damaged_limb = DAMAGE_LIMB.captures(&discerned);
    if let (Some(damaged_type), Some(damaged_limb)) = (damaged_type, damaged_limb) {
        format!(
            "{}_{}",
            damaged_limb.get(0).unwrap().as_str().replace(" ", "_"),
            damaged_type.get(0).unwrap().as_str()
        )
    } else {
        discerned.replace(" ", "_").to_string()
    }
}

pub fn aet_observation_creator(
    observation_name: &String,
    arguments: Vec<String>,
) -> AetObservation {
    match observation_name.as_ref() {
        "CombatAction" => CombatAction::observation(
            &arguments.get(0).unwrap(),
            &arguments.get(1).unwrap(),
            &arguments.get(2).unwrap(),
            &arguments.get(3).unwrap(),
            &arguments.get(4).unwrap(),
        ),
        "Proc" => CombatAction::proc_observation(
            &arguments.get(0).unwrap(),
            &arguments.get(1).unwrap(),
            &arguments.get(2).unwrap(),
            &arguments.get(3).unwrap(),
            &arguments.get(4).unwrap(),
        ),
        "SimpleCure" => {
            AetObservation::SimpleCureAction(match arguments.get(1).unwrap().as_ref() {
                "Smoke" => {
                    SimpleCureAction::smoke(&arguments.get(0).unwrap(), &arguments.get(2).unwrap())
                }
                "Pill" => {
                    SimpleCureAction::pill(&arguments.get(0).unwrap(), &arguments.get(2).unwrap())
                }
                "Salve" => SimpleCureAction::salve(
                    &arguments.get(0).unwrap(),
                    &arguments.get(2).unwrap(),
                    &arguments.get(3).unwrap(),
                ),
                _ => panic!("Bad SimpleCure: {:?}", arguments),
            })
        }
        "DualWield" => AetObservation::DualWield {
            who: arguments.get(0).unwrap().to_string(),
            left: arguments.get(1).unwrap().to_string(),
            right: arguments.get(2).unwrap().to_string(),
        },
        "Wield" => AetObservation::Wield {
            who: arguments.get(0).unwrap().to_string(),
            what: arguments.get(1).unwrap().to_string(),
            hand: arguments.get(2).unwrap().to_string(),
        },
        "Unwield" => AetObservation::Unwield {
            who: arguments.get(0).unwrap().to_string(),
            what: arguments.get(1).unwrap().to_string(),
            hand: arguments.get(2).unwrap().to_string(),
        },
        "Balance" => AetObservation::Balance(
            arguments.get(0).unwrap().to_string(),
            arguments
                .get(1)
                .unwrap()
                .to_string()
                .parse()
                .unwrap_or_default(),
        ),
        "LimbDamage" => AetObservation::LimbDamage(
            arguments.get(0).unwrap().to_string(),
            arguments
                .get(1)
                .unwrap()
                .to_string()
                .parse()
                .unwrap_or_default(),
        ),
        "LimbHeal" => AetObservation::LimbHeal(
            arguments.get(0).unwrap().to_string(),
            arguments
                .get(1)
                .unwrap()
                .to_string()
                .parse()
                .unwrap_or_default(),
        ),
        "DiscernedCure" => AetObservation::DiscernedCure(
            arguments.get(0).unwrap().to_string(),
            parse_discern(arguments.get(1).unwrap().to_string()),
        ),
        "DiscernedAfflict" => {
            AetObservation::DiscernedAfflict(parse_discern(arguments.get(0).unwrap().to_string()))
        }
        "OtherAfflicted" => AetObservation::OtherAfflicted(
            arguments.get(0).unwrap().to_string(),
            parse_discern(arguments.get(1).unwrap().to_string()),
        ),
        "Assess" => AetObservation::Assess(
            arguments.get(0).unwrap().to_string(),
            arguments
                .get(1)
                .unwrap()
                .to_string()
                .parse()
                .unwrap_or_default(),
            arguments
                .get(2)
                .unwrap()
                .to_string()
                .parse()
                .unwrap_or_default(),
        ),
        _ => AetObservation::enum_from_args(observation_name, arguments),
    }
}
