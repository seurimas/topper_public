mod zealot_timeline_tests {
    use crate::timeline::observations::aet_observation_creator;
    use crate::timeline::*;
    use crate::types::*;
    use topper_core::observations::*;
    use topper_core::timeline::db::DummyDatabaseModule;
    use topper_core::timeline::BaseTimeline;

    lazy_static! {
        static ref observer: ObservationParser<AetObservation> =
            ObservationParser::<AetObservation>::new_from_directory(
                "../triggers".to_string(),
                aet_observation_creator
            )
            .unwrap();
    }

    #[test]
    fn test_no_break() {
        let mut timeline = AetTimeline::new();
        timeline
            .state
            .for_agent(&"Tina".to_string(), |me: &mut AgentState| {
                me.set_limb_damage(LType::TorsoDamage, 1700);
            });
        let mut no_break = AetTimeSlice {
            observations: None,
            lines: vec![
                ("You use Zeal Clawtwist on Tina.".to_string(), 0),
                ("You grip what you can and twist viciously.".to_string(), 0),
                ("You use Zeal Clawtwist on Tina.".to_string(), 0),
                ("You grip what you can and twist viciously.".to_string(), 0),
                ("Balance Used: 3.37 seconds".to_string(), 0),
            ],
            gmcp: Vec::new(),
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Rinata".into(),
        };
        no_break.observations = Some(observer.observe(&no_break));
        println!("{:?}", no_break.observations);
        timeline.push_time_slice(no_break, None as Option<&DummyDatabaseModule>);
        let me_state = timeline.state.borrow_agent(&"Rinata".to_string());
        assert_eq!(me_state.balanced(BType::Balance), false);
        let you_state = timeline.state.borrow_agent(&"Tina".to_string());
        assert_eq!(you_state.get_limb_state(LType::TorsoDamage).damage, 33.32);
        assert_eq!(you_state.get_limb_state(LType::TorsoDamage).damaged, false);
    }
}
