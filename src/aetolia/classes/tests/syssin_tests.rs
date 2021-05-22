mod syssin_timeline_tests {
    use crate::aetolia::timeline::*;
    use crate::aetolia::types::*;
    use crate::timeline::BaseTimeline;

    #[test]
    fn test_dstab_3p() {
        let mut timeline = AetTimeline::new();
        timeline
            .state
            .add_player_hint(&"Savas", &"CALLED_VENOMS", "kalmia slike".to_string());
        let dstab_slice = AetTimeSlice {
            observations: Some(vec![AetObservation::CombatAction(CombatAction {
                caster: "Savas".to_string(),
                category: "Assassination".to_string(),
                skill: "Doublestab".to_string(),
                target: "Benedicto".to_string(),
                annotation: "".to_string(),
            })]),
            lines: vec![],
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(dstab_slice);
        let savas_state = timeline.state.borrow_agent(&"Savas".to_string());
        assert_eq!(savas_state.balanced(BType::Balance), false);
        assert_eq!(savas_state.is(FType::Asthma), false);
        assert_eq!(savas_state.is(FType::Anorexia), false);
        let bene_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Balance), true);
        assert_eq!(bene_state.is(FType::Asthma), true);
        assert_eq!(bene_state.is(FType::Anorexia), true);
    }

    #[test]
    fn test_dstab_3p_dodge() {
        let mut timeline = AetTimeline::new();
        timeline
            .state
            .add_player_hint(&"Savas", &"CALLED_VENOMS", "kalmia slike".to_string());
        let dstab_slice = AetTimeSlice {
            observations: Some(vec![
                AetObservation::CombatAction(CombatAction {
                    caster: "Savas".to_string(),
                    category: "Assassination".to_string(),
                    skill: "Doublestab".to_string(),
                    target: "Benedicto".to_string(),
                    annotation: "".to_string(),
                }),
                AetObservation::Dodges("Benedicto".to_string()),
            ]),
            lines: vec![],
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(dstab_slice);
        let savas_state = timeline.state.borrow_agent(&"Savas".to_string());
        assert_eq!(savas_state.balanced(BType::Balance), false);
        assert_eq!(savas_state.is(FType::Asthma), false);
        assert_eq!(savas_state.is(FType::Anorexia), false);
        let bene_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Balance), true);
        assert_eq!(bene_state.is(FType::Asthma), true);
        assert_eq!(bene_state.is(FType::Anorexia), false);
    }

    #[test]
    fn test_dstab() {
        let mut timeline = AetTimeline::new();
        let dstab_slice = AetTimeSlice {
            observations: Some(vec![
                AetObservation::CombatAction(CombatAction {
                    caster: "Seurimas".to_string(),
                    category: "Assassination".to_string(),
                    skill: "Doublestab".to_string(),
                    target: "Benedicto".to_string(),
                    annotation: "".to_string(),
                }),
                AetObservation::Devenoms("slike".into()),
                AetObservation::Devenoms("kalmia".into()),
            ]),
            lines: vec![],
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(dstab_slice);
        let seur_state = timeline.state.borrow_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.is(FType::Asthma), false);
        assert_eq!(seur_state.is(FType::Anorexia), false);
        let bene_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Balance), true);
        assert_eq!(bene_state.is(FType::Asthma), true);
        assert_eq!(bene_state.is(FType::Anorexia), true);
    }

    #[test]
    fn test_dstab_salve() {
        let mut timeline = AetTimeline::new();
        let dstab_slice = AetTimeSlice {
            observations: Some(vec![
                AetObservation::CombatAction(CombatAction {
                    caster: "Seurimas".to_string(),
                    category: "Assassination".to_string(),
                    skill: "Doublestab".to_string(),
                    target: "Benedicto".to_string(),
                    annotation: "".to_string(),
                }),
                AetObservation::Devenoms("epseth".into()),
                AetObservation::Devenoms("epteth".into()),
            ]),
            lines: vec![],
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(dstab_slice);
        let seur_state = timeline.state.borrow_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.is(FType::LeftLegBroken), false);
        assert_eq!(seur_state.is(FType::LeftArmBroken), false);
        let bene_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Balance), true);
        assert_eq!(bene_state.is(FType::LeftLegBroken), true);
        assert_eq!(bene_state.is(FType::LeftArmBroken), true);
    }

    #[test]
    fn test_dstab_absorbed() {
        let mut timeline = AetTimeline::new();
        let dstab_slice = AetTimeSlice {
            observations: Some(vec![
                AetObservation::CombatAction(CombatAction {
                    caster: "Seurimas".to_string(),
                    category: "Assassination".to_string(),
                    skill: "Doublestab".to_string(),
                    target: "Benedicto".to_string(),
                    annotation: "".to_string(),
                }),
                AetObservation::Absorbed("Benedicto".into(), "Remnant".into()),
                AetObservation::Devenoms("kalmia".into()),
            ]),
            lines: vec![],
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(dstab_slice);
        let seur_state = timeline.state.borrow_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.is(FType::Asthma), false);
        assert_eq!(seur_state.is(FType::Anorexia), false);
        let bene_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Balance), true);
        assert_eq!(bene_state.is(FType::Asthma), true);
        assert_eq!(bene_state.is(FType::Anorexia), false);
    }

    #[test]
    fn test_flay_general() {
        let mut timeline = AetTimeline::new();
        let dstab_slice = AetTimeSlice {
            observations: Some(vec![
                AetObservation::Gained("Benedicto".to_string(), "rebounding".to_string()),
                AetObservation::Sent("flay benedicto".into()),
                AetObservation::CombatAction(CombatAction {
                    caster: "Seurimas".to_string(),
                    category: "Assassination".to_string(),
                    skill: "Flay".to_string(),
                    target: "Benedicto".to_string(),
                    annotation: "speed".to_string(),
                }),
            ]),
            lines: vec![],
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(dstab_slice);
        let seur_state = timeline.state.borrow_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.is(FType::Asthma), false);
        assert_eq!(seur_state.is(FType::Anorexia), false);
        let bene_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Balance), true);
        assert_eq!(bene_state.is(FType::Rebounding), false);
        assert_eq!(bene_state.is(FType::Speed), false);
    }

    #[test]
    fn test_flay_specific() {
        let mut timeline = AetTimeline::new();
        let dstab_slice = AetTimeSlice {
            observations: Some(vec![
                AetObservation::Gained("Benedicto".to_string(), "rebounding".to_string()),
                AetObservation::Sent("flay Benedicto speed".into()),
                AetObservation::CombatAction(CombatAction {
                    caster: "Seurimas".to_string(),
                    category: "Assassination".to_string(),
                    skill: "Flay".to_string(),
                    target: "Benedicto".to_string(),
                    annotation: "speed".to_string(),
                }),
            ]),
            lines: vec![],
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(dstab_slice);
        let seur_state = timeline.state.borrow_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.is(FType::Asthma), false);
        assert_eq!(seur_state.is(FType::Anorexia), false);
        let bene_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Balance), true);
        assert_eq!(bene_state.is(FType::Rebounding), true);
        assert_eq!(bene_state.is(FType::Speed), false);
    }

    #[test]
    fn test_void_1p() {
        let mut timeline = AetTimeline::new();
        timeline
            .state
            .set_flag_for_agent(&"Seurimas".to_string(), &"void".to_string(), true);
        timeline
            .state
            .set_flag_for_agent(&"Seurimas".to_string(), &"stupidity".to_string(), true);
        let dstab_slice = AetTimeSlice {
            observations: Some(vec![
                AetObservation::SimpleCureAction(SimpleCureAction {
                    cure_type: SimpleCure::Pill("euphoriant".to_string()),
                    caster: "Seurimas".to_string(),
                }),
                AetObservation::Cured("void".to_string()),
                AetObservation::Afflicted("weakvoid".to_string()),
            ]),
            lines: vec![],
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        let bene_state = timeline.state.borrow_agent(&"Seurimas".to_string());
        assert_eq!(bene_state.balanced(BType::Pill), true);
        assert_eq!(bene_state.is(FType::Stupidity), true);
        assert_eq!(bene_state.is(FType::Void), true);
        assert_eq!(bene_state.is(FType::Weakvoid), false);
        timeline.push_time_slice(dstab_slice);
        let bene_state = timeline.state.borrow_agent(&"Seurimas".to_string());
        assert_eq!(bene_state.balanced(BType::Pill), false);
        assert_eq!(bene_state.is(FType::Stupidity), true);
        assert_eq!(bene_state.is(FType::Void), false);
        assert_eq!(bene_state.is(FType::Weakvoid), true);
    }

    #[test]
    fn test_void() {
        let mut timeline = AetTimeline::new();
        timeline
            .state
            .set_flag_for_agent(&"Benedicto".to_string(), &"void".to_string(), true);
        timeline
            .state
            .set_flag_for_agent(&"Benedicto".to_string(), &"stupidity".to_string(), true);
        let dstab_slice = AetTimeSlice {
            observations: Some(vec![
                AetObservation::SimpleCureAction(SimpleCureAction {
                    cure_type: SimpleCure::Pill("euphoriant".to_string()),
                    caster: "Benedicto".to_string(),
                }),
                AetObservation::DiscernedCure("Benedicto".to_string(), "void".to_string()),
            ]),
            lines: vec![],
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        let bene_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Pill), true);
        assert_eq!(bene_state.is(FType::Stupidity), true);
        assert_eq!(bene_state.is(FType::Void), true);
        assert_eq!(bene_state.is(FType::Weakvoid), false);
        timeline.push_time_slice(dstab_slice);
        let bene_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Pill), false);
        assert_eq!(bene_state.is(FType::Stupidity), true);
        assert_eq!(bene_state.is(FType::Void), false);
        assert_eq!(bene_state.is(FType::Weakvoid), true);
    }

    #[test]
    fn test_weakvoid() {
        let mut timeline = AetTimeline::new();
        timeline
            .state
            .set_flag_for_agent(&"Benedicto".to_string(), &"weakvoid".to_string(), true);
        timeline
            .state
            .set_flag_for_agent(&"Benedicto".to_string(), &"stupidity".to_string(), true);
        let dstab_slice = AetTimeSlice {
            observations: Some(vec![
                AetObservation::SimpleCureAction(SimpleCureAction {
                    cure_type: SimpleCure::Pill("euphoriant".to_string()),
                    caster: "Benedicto".to_string(),
                }),
                AetObservation::DiscernedCure("Benedicto".to_string(), "weakvoid".to_string()),
            ]),
            lines: vec![],
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        let bene_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Pill), true);
        assert_eq!(bene_state.is(FType::Stupidity), true);
        assert_eq!(bene_state.is(FType::Void), false);
        assert_eq!(bene_state.is(FType::Weakvoid), true);
        timeline.push_time_slice(dstab_slice);
        let bene_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Pill), false);
        assert_eq!(bene_state.is(FType::Stupidity), true);
        assert_eq!(bene_state.is(FType::Void), false);
        assert_eq!(bene_state.is(FType::Weakvoid), false);
    }

    #[test]
    fn test_dstab_purge() {
        let mut timeline = AetTimeline::new();
        let dstab_slice = AetTimeSlice {
            observations: Some(vec![
                AetObservation::CombatAction(CombatAction {
                    caster: "Seurimas".to_string(),
                    category: "Assassination".to_string(),
                    skill: "Doublestab".to_string(),
                    target: "Benedicto".to_string(),
                    annotation: "".to_string(),
                }),
                AetObservation::Devenoms("slike".into()),
                AetObservation::Devenoms("kalmia".into()),
                AetObservation::PurgeVenom("Benedicto".into(), "kalmia".into()),
            ]),
            lines: vec![],
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(dstab_slice);
        let seur_state = timeline.state.borrow_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.is(FType::Asthma), false);
        assert_eq!(seur_state.is(FType::Anorexia), false);
        let bene_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Balance), true);
        assert_eq!(bene_state.is(FType::Asthma), false);
        assert_eq!(bene_state.is(FType::Anorexia), true);
    }

    #[test]
    fn test_dstab_relapse() {
        let mut timeline = AetTimeline::new();
        let bite_slice = AetTimeSlice {
            observations: Some(vec![AetObservation::CombatAction(CombatAction {
                caster: "Seurimas".to_string(),
                category: "Assassination".to_string(),
                skill: "Bite".to_string(),
                target: "Benedicto".to_string(),
                annotation: "scytherus".to_string(),
            })]),
            lines: vec![],
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        let dstab_slice = AetTimeSlice {
            observations: Some(vec![
                AetObservation::CombatAction(CombatAction {
                    caster: "Seurimas".to_string(),
                    category: "Assassination".to_string(),
                    skill: "Doublestab".to_string(),
                    target: "Benedicto".to_string(),
                    annotation: "".to_string(),
                }),
                AetObservation::Devenoms("slike".into()),
                AetObservation::Devenoms("kalmia".into()),
            ]),
            lines: vec![],
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        let cure_slice = AetTimeSlice {
            observations: Some(vec![
                AetObservation::SimpleCureAction(SimpleCureAction {
                    caster: "Benedicto".into(),
                    cure_type: SimpleCure::Pill("decongestant".into()),
                }),
                AetObservation::SimpleCureAction(SimpleCureAction {
                    caster: "Benedicto".into(),
                    cure_type: SimpleCure::Salve("epidermal".into(), "head".into()),
                }),
            ]),
            lines: vec![],
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        let relapse_slice = AetTimeSlice {
            observations: Some(vec![
                AetObservation::Relapse("Benedicto".into()),
                AetObservation::Relapse("Benedicto".into()),
            ]),
            lines: vec![],
            prompt: AetPrompt::Blackout,
            time: 220,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(bite_slice);
        timeline.push_time_slice(dstab_slice);
        let seur_state = timeline.state.borrow_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.is(FType::Asthma), false);
        assert_eq!(seur_state.is(FType::Anorexia), false);
        let mut bene_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Balance), true);
        assert_eq!(bene_state.is(FType::Asthma), true);
        assert_eq!(bene_state.is(FType::Anorexia), true);
        timeline.push_time_slice(cure_slice);
        let mut bene_cured_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_cured_state.balanced(BType::Balance), true);
        assert_eq!(bene_cured_state.is(FType::Asthma), false);
        assert_eq!(bene_cured_state.is(FType::Anorexia), false);
        timeline.push_time_slice(relapse_slice);
        let mut bene_relapsed_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_relapsed_state.balanced(BType::Balance), true);
        assert_eq!(bene_relapsed_state.is(FType::Asthma), true);
        assert_eq!(bene_relapsed_state.is(FType::Anorexia), true);
    }

    #[test]
    fn test_dstab_relapse_clever() {
        let mut timeline = AetTimeline::new();
        let bite_slice = AetTimeSlice {
            observations: Some(vec![AetObservation::CombatAction(CombatAction {
                caster: "Seurimas".to_string(),
                category: "Assassination".to_string(),
                skill: "Bite".to_string(),
                target: "Benedicto".to_string(),
                annotation: "scytherus".to_string(),
            })]),
            lines: vec![],
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(bite_slice);
        let dstab_slice = AetTimeSlice {
            observations: Some(vec![
                AetObservation::CombatAction(CombatAction {
                    caster: "Seurimas".to_string(),
                    category: "Assassination".to_string(),
                    skill: "Doublestab".to_string(),
                    target: "Benedicto".to_string(),
                    annotation: "".to_string(),
                }),
                AetObservation::Devenoms("slike".into()),
                AetObservation::Devenoms("kalmia".into()),
            ]),
            lines: vec![],
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        let cure_slice_1 = AetTimeSlice {
            observations: Some(vec![
                AetObservation::SimpleCureAction(SimpleCureAction {
                    caster: "Benedicto".into(),
                    cure_type: SimpleCure::Pill("decongestant".into()),
                }),
                AetObservation::SimpleCureAction(SimpleCureAction {
                    caster: "Benedicto".into(),
                    cure_type: SimpleCure::Salve("epidermal".into(), "torso".into()),
                }),
            ]),
            lines: vec![],
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        let relapse_slice_1 = AetTimeSlice {
            observations: Some(vec![AetObservation::Relapse("Benedicto".into())]),
            lines: vec![],
            prompt: AetPrompt::Blackout,
            time: 220,
            me: "Seurimas".into(),
        };
        let cure_slice_2 = AetTimeSlice {
            observations: Some(vec![AetObservation::SimpleCureAction(SimpleCureAction {
                caster: "Benedicto".into(),
                cure_type: SimpleCure::Salve("epidermal".into(), "torso".into()),
            })]),
            lines: vec![],
            prompt: AetPrompt::Blackout,
            time: 225,
            me: "Seurimas".into(),
        };
        let relapse_slice_2 = AetTimeSlice {
            observations: Some(vec![AetObservation::Relapse("Benedicto".into())]),
            lines: vec![],
            prompt: AetPrompt::Blackout,
            time: 270,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(dstab_slice);
        let seur_state = timeline.state.borrow_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.is(FType::Asthma), false);
        assert_eq!(seur_state.is(FType::Anorexia), false);
        let mut bene_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Balance), true);
        assert_eq!(bene_state.is(FType::Asthma), true);
        assert_eq!(bene_state.is(FType::Anorexia), true);
        timeline.push_time_slice(cure_slice_1);
        let mut bene_cured_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_cured_state.balanced(BType::Balance), true);
        assert_eq!(bene_cured_state.is(FType::Asthma), false);
        assert_eq!(bene_cured_state.is(FType::Anorexia), false);
        timeline.push_time_slice(relapse_slice_1);
        timeline.push_time_slice(cure_slice_2);
        let mut bene_relapsed_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_relapsed_state.balanced(BType::Balance), true);
        assert_eq!(bene_relapsed_state.is(FType::Asthma), false);
        assert_eq!(bene_relapsed_state.is(FType::Anorexia), false);
        timeline.push_time_slice(relapse_slice_2);
        let mut bene_relapsed_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_relapsed_state.balanced(BType::Balance), true);
        assert_eq!(bene_relapsed_state.is(FType::Asthma), true);
        assert_eq!(bene_relapsed_state.is(FType::Anorexia), false);
    }

    #[test]
    fn test_dstab_rebounds() {
        let mut timeline = AetTimeline::new();
        let dstab_slice = AetTimeSlice {
            observations: Some(vec![
                AetObservation::CombatAction(CombatAction {
                    caster: "Seurimas".to_string(),
                    category: "Assassination".to_string(),
                    skill: "Doublestab".to_string(),
                    target: "Benedicto".to_string(),
                    annotation: "".to_string(),
                }),
                AetObservation::Rebounds,
                AetObservation::Devenoms("slike".into()),
                AetObservation::Rebounds,
                AetObservation::Devenoms("kalmia".into()),
            ]),
            lines: vec![],
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(dstab_slice);
        let seur_state = timeline.state.borrow_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.is(FType::Asthma), true);
        assert_eq!(seur_state.is(FType::Anorexia), true);
        let bene_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Balance), true);
        assert_eq!(bene_state.is(FType::Asthma), false);
        assert_eq!(bene_state.is(FType::Anorexia), false);
    }

    #[test]
    fn test_bite() {
        let mut timeline = AetTimeline::new();
        let dstab_slice = AetTimeSlice {
            observations: Some(vec![AetObservation::CombatAction(CombatAction {
                caster: "Seurimas".to_string(),
                category: "Assassination".to_string(),
                skill: "Bite".to_string(),
                target: "Benedicto".to_string(),
                annotation: "scytherus".to_string(),
            })]),
            lines: vec![],
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(dstab_slice);
        let seur_state = timeline.state.borrow_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.is(FType::ThinBlood), false);
        let bene_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Balance), true);
        assert_eq!(bene_state.is(FType::ThinBlood), true);
    }

    #[test]
    fn test_bite_absorbed() {
        let mut timeline = AetTimeline::new();
        let dstab_slice = AetTimeSlice {
            observations: Some(vec![
                AetObservation::CombatAction(CombatAction {
                    caster: "Seurimas".to_string(),
                    category: "Assassination".to_string(),
                    skill: "Bite".to_string(),
                    target: "Benedicto".to_string(),
                    annotation: "scytherus".to_string(),
                }),
                AetObservation::Absorbed("Benedicto".into(), "Remnant".into()),
            ]),
            lines: vec![],
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(dstab_slice);
        let seur_state = timeline.state.borrow_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.is(FType::ThinBlood), false);
        let bene_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Balance), true);
        assert_eq!(bene_state.is(FType::ThinBlood), false);
    }

    #[test]
    fn test_bite_countercurrent() {
        let mut timeline = AetTimeline::new();
        let dstab_slice = AetTimeSlice {
            observations: Some(vec![
                AetObservation::CombatAction(CombatAction {
                    caster: "Seurimas".to_string(),
                    category: "Assassination".to_string(),
                    skill: "Bite".to_string(),
                    target: "Benedicto".to_string(),
                    annotation: "scytherus".to_string(),
                }),
                AetObservation::PurgeVenom("Benedicto".into(), "scytherus".into()),
            ]),
            lines: vec![],
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(dstab_slice);
        let seur_state = timeline.state.borrow_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.is(FType::ThinBlood), false);
        let bene_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Balance), true);
        assert_eq!(bene_state.is(FType::ThinBlood), false);
    }

    #[test]
    fn test_bite_parry() {
        let mut timeline = AetTimeline::new();
        let dstab_slice = AetTimeSlice {
            observations: Some(vec![
                AetObservation::CombatAction(CombatAction {
                    caster: "Seurimas".to_string(),
                    category: "Assassination".to_string(),
                    skill: "Bite".to_string(),
                    target: "Benedicto".to_string(),
                    annotation: "scytherus".to_string(),
                }),
                AetObservation::Parry("Benedicto".to_string(), "head".to_string()),
            ]),
            lines: vec![],
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(dstab_slice);
        let seur_state = timeline.state.borrow_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.is(FType::ThinBlood), false);
        assert_eq!(seur_state.get_parrying(), None);
        let bene_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Balance), true);
        assert_eq!(bene_state.is(FType::ThinBlood), false);
        assert_eq!(bene_state.get_parrying(), Some(LType::HeadDamage));
    }

    #[test]
    fn test_suggest() {
        let mut timeline = AetTimeline::new();
        let suggest_slice = AetTimeSlice {
            observations: Some(vec![
                AetObservation::Sent("suggest Benedicto stupidity".to_string()),
                AetObservation::CombatAction(CombatAction {
                    caster: "Seurimas".to_string(),
                    category: "Hypnosis".to_string(),
                    skill: "Suggest".to_string(),
                    target: "Benedicto".to_string(),
                    annotation: "".to_string(),
                }),
            ]),
            lines: vec![],
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(suggest_slice);
        let seur_state = timeline.state.borrow_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Equil), false);
        let bene_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(
            bene_state.hypno_state.hypnosis_stack.get(0),
            Some(&Hypnosis::Aff(FType::Stupidity))
        );
    }

    #[test]
    fn test_suggest_qeb() {
        let mut timeline = AetTimeline::new();
        let suggest_slice = AetTimeSlice {
            observations: Some(vec![
                AetObservation::Sent(
                    "qeb dstab Benedicto aconite kalmia;;suggest Benedicto stupidity".to_string(),
                ),
                AetObservation::CombatAction(CombatAction {
                    caster: "Seurimas".to_string(),
                    category: "Hypnosis".to_string(),
                    skill: "Suggest".to_string(),
                    target: "Benedicto".to_string(),
                    annotation: "".to_string(),
                }),
            ]),
            lines: vec![],
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(suggest_slice);
        let bene_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(
            bene_state.hypno_state.hypnosis_stack.get(0),
            Some(&Hypnosis::Aff(FType::Stupidity))
        );
    }

    use crate::aetolia::classes::syssin::get_attack;

    #[test]
    fn test_bedazzling() {
        let mut timeline = AetTimeline::new();
        let qeb = get_attack(
            &timeline,
            &"Benedicto".to_string(),
            &"bedazzle".to_string(),
            None,
        );
        assert_eq!(
            qeb,
            "qeb parry head;;stand;;bedazzle Benedicto;;hypnotise Benedicto;;suggest Benedicto Hypochondria%%qs shadow sleight void Benedicto",
        );
    }

    #[test]
    fn test_aggro() {
        let mut timeline = AetTimeline::new();
        let qeb = get_attack(
            &timeline,
            &"Benedicto".to_string(),
            &"aggro".to_string(),
            None,
        );
        assert_eq!(
            qeb,
            "qeb parry head;;stand;;envenom whip with curare;;flay Benedicto;;hypnotise Benedicto;;suggest Benedicto Hypochondria%%qs shadow sleight void Benedicto",
        );
    }
}
