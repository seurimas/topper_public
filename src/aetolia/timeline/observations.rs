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
}
