mod observer_tests {
    use super::super::*;

    lazy_static! {
        static ref observer: ObservationParser<AetObservation> =
            ObservationParser::<AetObservation>::new_from_directory("triggers".to_string())
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
            gmcp: Vec::new(),
            prompt: AetPrompt::Promptless,
            time: 0,
            me: "Seurimas".into(),
        };
        let observed = observer.observe(&slice);
        let mut expected = Vec::new();
        expected.push(CombatAction::observation(
            "Seurimas",
            "Assassination",
            "Doublestab",
            "",
            "Benedicto",
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
            gmcp: Vec::new(),
            prompt: AetPrompt::Promptless,
            time: 0,
            me: "Seurimas".into(),
        };
        let observed = observer.observe(&slice);
        let mut expected = Vec::new();
        expected.push(CombatAction::observation(
            "Seurimas",
            "Assassination",
            "Bite",
            "scytherus",
            "Benedicto",
        ));
        assert_eq!(observed, expected);
    }

    #[test]
    fn test_combat_action_no_target() {
        let slice = AetTimeSlice {
            observations: None,
            lines: vec![("You use Assassination Warding.".to_string(), 0)],
            gmcp: Vec::new(),
            prompt: AetPrompt::Promptless,
            time: 0,
            me: "Seurimas".into(),
        };
        let observed = observer.observe(&slice);
        let mut expected = Vec::new();
        expected.push(CombatAction::observation(
            "Seurimas",
            "Assassination",
            "Warding",
            "",
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
            gmcp: Vec::new(),
            prompt: AetPrompt::Promptless,
            time: 0,
            me: "Seurimas".into(),
        };
        let observed = observer.observe(&slice);
        let mut expected = Vec::new();
        expected.push(CombatAction::observation(
            "Seurimas", "Tattoos", "Tree", "", "",
        ));
        expected.push(CombatAction::observation(
            "Benedicto",
            "Tattoos",
            "Tree",
            "",
            "",
        ));
        assert_eq!(observed, expected);
    }
}
