use crate::aetolia::timeline::*;
use crate::timeline::types::*;
use crate::topper::observations::*;
use std::error::Error;

#[cfg(test)]
mod observer_tests {
    use super::*;

    lazy_static! {
        static ref observer: ObservationParser<AetObservation> =
            ObservationParser::<AetObservation>::new_from_file("triggers.json".to_string())
                .unwrap();
    }

    #[test]
    fn test_combat_action_target() {
        let slice = AetTimeSlice {
            observations: None,
            lines: vec![(
                "You use Assassination Doublestab on Benedicto.".to_string(),
                0,
            )],
            prompt: AetPrompt::Promptless,
            time: 0,
            me: "Seurimas".into(),
        };
        let observed = observer.observe(&slice);
        let mut expected = Vec::new();
        expected.push(CombatAction::observation(
            "Seurimas",
            "Benedicto",
            "Assassination",
            "Doublestab",
            "",
        ));
        assert_eq!(observed, expected);
    }

    #[test]
    fn test_combat_action_target_annotated() {
        let slice = AetTimeSlice {
            observations: None,
            lines: vec![(
                "You use Assassination Bite (scytherus) on Benedicto.".to_string(),
                0,
            )],
            prompt: AetPrompt::Promptless,
            time: 0,
            me: "Seurimas".into(),
        };
        let observed = observer.observe(&slice);
        let mut expected = Vec::new();
        expected.push(CombatAction::observation(
            "Seurimas",
            "Benedicto",
            "Assassination",
            "Bite",
            "scytherus",
        ));
        assert_eq!(observed, expected);
    }

    #[test]
    fn test_combat_action_no_target() {
        let slice = AetTimeSlice {
            observations: None,
            lines: vec![("You use Assassination Warding.".to_string(), 0)],
            prompt: AetPrompt::Promptless,
            time: 0,
            me: "Seurimas".into(),
        };
        let observed = observer.observe(&slice);
        let mut expected = Vec::new();
        expected.push(CombatAction::observation(
            "Seurimas",
            "",
            "Assassination",
            "Warding",
            "",
        ));
        assert_eq!(observed, expected);
    }

    #[test]
    fn test_touch_tree() {
        let slice = AetTimeSlice {
            observations: None,
            lines: vec![
                ("You touch the tree of life tattoo.".to_string(), 0),
                ("Benedicto touches a tree of life tattoo.".to_string(), 1),
            ],
            prompt: AetPrompt::Promptless,
            time: 0,
            me: "Seurimas".into(),
        };
        let observed = observer.observe(&slice);
        let mut expected = Vec::new();
        expected.push(CombatAction::observation(
            "Seurimas", "", "Tattoos", "Tree", "",
        ));
        expected.push(CombatAction::observation(
            "Benedicto",
            "",
            "Tattoos",
            "Tree",
            "",
        ));
        assert_eq!(observed, expected);
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
