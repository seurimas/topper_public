use crate::aetolia::timeline::*;
use crate::timeline::types::*;
use crate::topper::observations::*;
use regex::Regex;
use std::error::Error;
use std::fs::read_dir;
use std::fs::{DirEntry, File};
use std::io::BufReader;

#[cfg(test)]
#[path = "./tests/observations_tests.rs"]
mod observer_tests;

lazy_static! {
    static ref DAMAGE_TYPE: Regex = Regex::new("broken|damaged|mangled|dislocated").unwrap();
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

fn aet_observation_creator(observation_name: &String, arguments: Vec<String>) -> AetObservation {
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
        _ => AetObservation::enum_from_args(observation_name, arguments),
    }
}

#[derive(Debug)]
struct ObservationParserError {
    base: serde_json::Error,
    path: String,
}

impl std::fmt::Display for ObservationParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.path, self.base)
    }
}

impl Error for ObservationParserError {}

impl ObservationParser<AetObservation> {
    pub fn new_from_file(path: String) -> Result<Self, Box<Error>> {
        use std::fs::File;
        use std::io::BufReader;
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mappings = serde_json::from_reader(reader)?;
        Ok(ObservationParser::new(mappings, aet_observation_creator))
    }

    fn read_file_mappings(
        path: String,
        file: File,
        mappings: &mut Vec<ObservationMapping>,
    ) -> Result<(), Box<Error>> {
        let reader = BufReader::new(file);
        let mut new_mappings: Vec<ObservationMapping> = serde_json::from_reader(reader)
            .map_err(move |err| ObservationParserError { base: err, path })?;
        mappings.append(&mut new_mappings);
        Ok(())
    }

    fn read_mappings(
        entry: DirEntry,
        mappings: &mut Vec<ObservationMapping>,
    ) -> Result<(), Box<Error>> {
        if entry.file_type()?.is_dir() {
            for path in read_dir(entry.path()).unwrap() {
                Self::read_mappings(path.unwrap(), mappings)?;
            }
        } else {
            let path_ = entry.path();
            let file = File::open(path_.clone())?;
            Self::read_file_mappings(path_.to_str().unwrap().to_string(), file, mappings)?;
        }
        Ok(())
    }

    pub fn new_from_directory(dir: String) -> Result<Self, Box<Error>> {
        let mut mappings = Vec::new();
        for path in read_dir(dir).unwrap() {
            Self::read_mappings(path.unwrap(), &mut mappings)?;
        }
        Ok(ObservationParser::new(mappings, aet_observation_creator))
    }
}
