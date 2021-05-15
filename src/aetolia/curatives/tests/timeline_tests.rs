mod timeline_tests {
    use crate::timeline::{TimeSlice, BaseTimeline};
    use crate::aetolia::curatives::*;
    use crate::aetolia::types::*;
    use crate::aetolia::timeline::*;

    #[test]
    fn test_pill() {
        let mut timeline = AetTimeline::new();
        {
            let mut updated_seur = timeline.state.get_agent(&"Seurimas".to_string());
            updated_seur.set_flag(FType::ThinBlood, true);
            timeline.state.set_agent(&"Seurimas".into(), updated_seur);
        }
        {
            let mut updated_bene = timeline.state.get_agent(&"Benedicto".to_string());
            updated_bene.set_flag(FType::ThinBlood, true);
            timeline.state.set_agent(&"Benedicto".into(), updated_bene);
        }
        let coag_slice = TimeSlice {
            observations: Some(vec![AetObservation::SimpleCureAction(SimpleCureAction {
                caster: "Benedicto".into(),
                cure_type: SimpleCure::Pill("coagulation".into()),
            })]),
            lines: vec![],
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(coag_slice);
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Pill), true);
        assert_eq!(seur_state.is(FType::ThinBlood), true);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Pill), false);
        assert_eq!(bene_state.is(FType::ThinBlood), false);
    }

    #[test]
    fn test_mending() {
        let mut timeline = AetTimeline::new();
        {
            let mut updated_seur = timeline.state.get_agent(&"Seurimas".to_string());
            updated_seur.set_flag(FType::LeftArmBroken, true);
            timeline.state.set_agent(&"Seurimas".into(), updated_seur);
        }
        {
            let mut updated_bene = timeline.state.get_agent(&"Benedicto".to_string());
            updated_bene.set_flag(FType::LeftLegBroken, true);
            timeline.state.set_agent(&"Benedicto".into(), updated_bene);
        }
        let coag_slice = TimeSlice {
            observations: Some(vec![AetObservation::SimpleCureAction(SimpleCureAction {
                caster: "Benedicto".into(),
                cure_type: SimpleCure::Salve("mending".into(), "skin".into()),
            })]),
            lines: vec![],
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(coag_slice);
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Salve), true);
        assert_eq!(seur_state.is(FType::LeftArmBroken), true);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Salve), false);
        assert_eq!(bene_state.is(FType::LeftArmBroken), false);
    }
}