mod sentinel_timeline_tests {
    use crate::timeline::*;
    use crate::types::*;
    use topper_core::timeline::BaseTimeline;

    #[test]
    fn test_salve_attacks() {
        let mut timeline = AetTimeline::new();
        let breath_flourish_slice = AetTimeSlice {
            observations: Some(vec![
                AetObservation::CombatAction(CombatAction {
                    annotation: "".to_string(),
                    caster: "Rinata".to_string(),
                    category: "Woodlore".to_string(),
                    skill: "Icebreath".to_string(),
                    target: "Illidan".to_string(),
                }),
                AetObservation::CombatAction(CombatAction {
                    annotation: "".to_string(),
                    caster: "Rinata".to_string(),
                    category: "Dhuriv".to_string(),
                    skill: "Flourish".to_string(),
                    target: "Illidan".to_string(),
                }),
                AetObservation::Devenoms("epseth".to_string()),
            ]),
            lines: vec![],
            gmcp: Vec::new(),
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Rinata".into(),
        };
        timeline.push_time_slice(breath_flourish_slice, None);
        let me_state = timeline.state.borrow_agent(&"Rinata".to_string());
        assert_eq!(me_state.balanced(BType::Balance), false);
        assert_eq!(me_state.balanced(BType::Equil), false);
        assert_eq!(me_state.is(FType::Insulation), true);
        assert_eq!(me_state.is(FType::LeftLegBroken), false);
        let you_state = timeline.state.borrow_agent(&"Illidan".to_string());
        assert_eq!(you_state.balanced(BType::Balance), true);
        assert_eq!(you_state.balanced(BType::Equil), true);
        assert_eq!(you_state.is(FType::Insulation), false);
        assert_eq!(you_state.is(FType::LeftLegBroken), true);
    }
}
