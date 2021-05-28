use crate::aetolia::timeline::*;
use crate::timeline::types::*;
use crate::topper::observations::*;
use std::error::Error;

#[cfg(test)]
#[path = "./tests/observations_tests.rs"]
mod observer_tests;

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
        "Wound" => AetObservation::Wound(
            arguments.get(0).unwrap().to_string(),
            arguments
                .get(1)
                .unwrap()
                .to_string()
                .parse()
                .unwrap_or_default(),
        ),
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

    pub fn new_from_directory(dir: String) -> Result<Self, Box<Error>> {
        use std::fs::read_dir;
        use std::fs::File;
        use std::io::BufReader;
        let mut mappings = Vec::new();
        for path in read_dir(dir).unwrap() {
            let path_ = path.unwrap().path();
            let file = File::open(path_.clone())?;
            let reader = BufReader::new(file);
            let mut new_mappings: Vec<ObservationMapping> = serde_json::from_reader(reader)
                .map_err(move |err| ObservationParserError {
                    base: err,
                    path: path_.to_str().unwrap().to_string(),
                })?;
            mappings.append(&mut new_mappings);
        }
        Ok(ObservationParser::new(mappings, aet_observation_creator))
    }
}
