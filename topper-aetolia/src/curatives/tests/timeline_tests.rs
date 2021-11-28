mod timeline_tests {
    use crate::curatives::*;
    use crate::timeline::*;
    use crate::timeline::{BaseTimeline, TimeSlice};
    use crate::types::*;

    #[test]
    fn test_pill() {
        let mut timeline = AetTimeline::new();
        timeline
            .state
            .for_agent(&"Seurimas".into(), |updated_seur| {
                updated_seur.set_flag(FType::ThinBlood, true);
            });
        timeline
            .state
            .for_agent(&"Benedicto".into(), |updated_bene| {
                updated_bene.set_flag(FType::ThinBlood, true);
            });
        let coag_slice = TimeSlice {
            observations: Some(vec![AetObservation::SimpleCureAction(SimpleCureAction {
                caster: "Benedicto".into(),
                cure_type: SimpleCure::Pill("coagulation".into()),
            })]),
            lines: vec![],
            gmcp: Vec::new(),
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(coag_slice, None);
        let seur_state = timeline.state.borrow_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Pill), true);
        assert_eq!(seur_state.is(FType::ThinBlood), true);
        let bene_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Pill), false);
        assert_eq!(bene_state.is(FType::ThinBlood), false);
    }

    #[test]
    fn test_mending() {
        let mut timeline = AetTimeline::new();
        timeline
            .state
            .for_agent(&"Seurimas".into(), |updated_seur| {
                updated_seur.set_flag(FType::LeftArmBroken, true);
            });
        timeline
            .state
            .for_agent(&"Benedicto".into(), |updated_bene| {
                updated_bene.set_flag(FType::LeftArmBroken, true);
            });
        let coag_slice = TimeSlice {
            observations: Some(vec![AetObservation::SimpleCureAction(SimpleCureAction {
                caster: "Benedicto".into(),
                cure_type: SimpleCure::Salve("mending".into(), "skin".into()),
            })]),
            lines: vec![],
            gmcp: Vec::new(),
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(coag_slice, None);
        let seur_state = timeline.state.borrow_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Salve), true);
        assert_eq!(seur_state.is(FType::LeftArmBroken), true);
        let bene_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Salve), false);
        assert_eq!(bene_state.is(FType::LeftArmBroken), false);
    }
}
