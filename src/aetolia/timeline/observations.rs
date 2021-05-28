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
        _ => AetObservation::enum_from_args(observation_name, arguments),
    }
}

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
            let file = File::open(path.unwrap().path())?;
            let reader = BufReader::new(file);
            let mut new_mappings: Vec<ObservationMapping> = serde_json::from_reader(reader)?;
            mappings.append(&mut new_mappings);
        }
        Ok(ObservationParser::new(mappings, aet_observation_creator))
    }
}
