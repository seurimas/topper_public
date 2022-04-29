mod timeline_tests {
    use topper_core::timeline::BaseTimeline;

    use crate::curatives::*;
    use crate::timeline::*;
    use crate::types::*;
    use topper_core::timeline::db::DummyDatabaseModule;

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
        let coag_slice = AetTimeSlice {
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
        timeline.push_time_slice(coag_slice, None as Option<&DummyDatabaseModule>);
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
        let coag_slice = AetTimeSlice {
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
        timeline.push_time_slice(coag_slice, None as Option<&DummyDatabaseModule>);
        let seur_state = timeline.state.borrow_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Salve), true);
        assert_eq!(seur_state.is(FType::LeftArmBroken), true);
        let bene_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Salve), false);
        assert_eq!(bene_state.is(FType::LeftArmBroken), false);
    }

    #[test]
    fn test_restoration_pre_restore() {
        let mut timeline = AetTimeline::new();
        timeline
            .state
            .for_agent(&"Seurimas".into(), |updated_seur| {
                updated_seur.set_limb_damage(LType::LeftLegDamage, 1500);
            });
        timeline
            .state
            .for_agent(&"Benedicto".into(), |updated_bene| {
                updated_bene.set_limb_damage(LType::LeftLegDamage, 1500);
            });
        let restore_slice = AetTimeSlice {
            observations: Some(vec![
                AetObservation::SimpleCureAction(SimpleCureAction {
                    caster: "Benedicto".into(),
                    cure_type: SimpleCure::Salve("restoration".into(), "left leg".into()),
                }),
                AetObservation::SimpleCureAction(SimpleCureAction {
                    caster: "Seurimas".into(),
                    cure_type: SimpleCure::Salve("restoration".into(), "left leg".into()),
                }),
            ]),
            lines: vec![],
            gmcp: Vec::new(),
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        let time_slice = AetTimeSlice {
            observations: Some(vec![AetObservation::LimbHeal("left leg".into(), 15.00)]),
            lines: vec![],
            gmcp: Vec::new(),
            prompt: AetPrompt::Promptless,
            time: 1000,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(restore_slice, None as Option<&DummyDatabaseModule>);
        let seur_state = timeline.state.borrow_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Salve), false);
        assert_eq!(seur_state.get_limb_state(LType::LeftLegDamage).damage, 15.0);
        let bene_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Salve), false);
        assert_eq!(bene_state.get_limb_state(LType::LeftLegDamage).damage, 15.0);
        timeline.push_time_slice(time_slice, None as Option<&DummyDatabaseModule>);
        let seur_state = timeline.state.borrow_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Salve), true);
        assert_eq!(seur_state.get_limb_state(LType::LeftLegDamage).damage, 0.0);
        let bene_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Salve), true);
        assert_eq!(bene_state.get_limb_state(LType::LeftLegDamage).damage, 0.0);
    }

    #[test]
    fn test_restoration_break() {
        let mut timeline = AetTimeline::new();
        timeline
            .state
            .for_agent(&"Seurimas".into(), |updated_seur| {
                updated_seur
                    .limb_damage
                    .set_limb_damaged(LType::LeftLegDamage, true);
            });
        timeline
            .state
            .for_agent(&"Benedicto".into(), |updated_bene| {
                updated_bene
                    .limb_damage
                    .set_limb_damaged(LType::LeftLegDamage, true);
            });
        let restore_slice = AetTimeSlice {
            observations: Some(vec![
                AetObservation::SimpleCureAction(SimpleCureAction {
                    caster: "Benedicto".into(),
                    cure_type: SimpleCure::Salve("restoration".into(), "left leg".into()),
                }),
                AetObservation::SimpleCureAction(SimpleCureAction {
                    caster: "Seurimas".into(),
                    cure_type: SimpleCure::Salve("restoration".into(), "left leg".into()),
                }),
            ]),
            lines: vec![],
            gmcp: Vec::new(),
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        let time_slice = AetTimeSlice {
            observations: Some(vec![
                AetObservation::Cured("left_leg_damaged".into()),
                AetObservation::LimbHeal("left leg".into(), 30.0),
            ]),
            lines: vec![],
            gmcp: Vec::new(),
            prompt: AetPrompt::Promptless,
            time: 1000,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(restore_slice, None as Option<&DummyDatabaseModule>);
        let seur_state = timeline.state.borrow_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Salve), false);
        assert_eq!(
            seur_state.get_limb_state(LType::LeftLegDamage).damage,
            33.33
        );
        let bene_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Salve), false);
        assert_eq!(
            bene_state.get_limb_state(LType::LeftLegDamage).damage,
            33.33
        );
        timeline.push_time_slice(time_slice, None as Option<&DummyDatabaseModule>);
        let seur_state = timeline.state.borrow_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Salve), true);
        assert_eq!(seur_state.get_limb_state(LType::LeftLegDamage).damage, 3.33);
        let bene_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Salve), true);
        assert_eq!(bene_state.get_limb_state(LType::LeftLegDamage).damage, 3.33);
    }

    #[test]
    fn test_restoration_break_restoration() {
        let mut timeline = AetTimeline::new();
        timeline
            .state
            .for_agent(&"Seurimas".into(), |updated_seur| {
                updated_seur
                    .limb_damage
                    .set_limb_damaged(LType::LeftLegDamage, true);
            });
        timeline
            .state
            .for_agent(&"Benedicto".into(), |updated_bene| {
                updated_bene
                    .limb_damage
                    .set_limb_damaged(LType::LeftLegDamage, true);
            });
        let restore_slice = AetTimeSlice {
            observations: Some(vec![
                AetObservation::SimpleCureAction(SimpleCureAction {
                    caster: "Benedicto".into(),
                    cure_type: SimpleCure::Salve("restoration".into(), "left leg".into()),
                }),
                AetObservation::SimpleCureAction(SimpleCureAction {
                    caster: "Seurimas".into(),
                    cure_type: SimpleCure::Salve("restoration".into(), "left leg".into()),
                }),
            ]),
            lines: vec![],
            gmcp: Vec::new(),
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        let regenerate_slice = AetTimeSlice {
            observations: Some(vec![
                CombatAction::observation("Benedicto", "Hunting", "Regenerate", "", ""),
                CombatAction::observation("Seurimas", "Hunting", "Regenerate", "", ""),
            ]),
            lines: vec![],
            gmcp: Vec::new(),
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        let time_slice = AetTimeSlice {
            observations: Some(vec![
                AetObservation::Cured("torso_damaged".into()),
                AetObservation::LimbHeal("Seurimas".into(), 33.33),
            ]),
            lines: vec![],
            gmcp: Vec::new(),
            prompt: AetPrompt::Promptless,
            time: 1000,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(restore_slice, None as Option<&DummyDatabaseModule>);
        timeline.push_time_slice(regenerate_slice, None as Option<&DummyDatabaseModule>);
        let seur_state = timeline.state.borrow_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Salve), false);
        assert_eq!(
            seur_state.get_limb_state(LType::LeftLegDamage).damage,
            33.33
        );
        let bene_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Salve), false);
        assert_eq!(
            bene_state.get_limb_state(LType::LeftLegDamage).damage,
            33.33
        );
        timeline.push_time_slice(time_slice, None as Option<&DummyDatabaseModule>);
        let seur_state = timeline.state.borrow_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Salve), true);
        assert_eq!(
            seur_state.get_limb_state(LType::LeftLegDamage).damage,
            33.33
        );
        let bene_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Salve), true);
        assert_eq!(bene_state.get_limb_state(LType::LeftLegDamage).damage, 0.00);
    }

    #[test]
    fn test_restoration_cure() {
        let mut timeline = AetTimeline::new();
        timeline
            .state
            .for_agent(&"Seurimas".into(), |updated_seur| {
                updated_seur
                    .limb_damage
                    .set_limb_damaged(LType::TorsoDamage, true);
                updated_seur.set_flag(FType::Heatspear, true);
            });
        timeline
            .state
            .for_agent(&"Benedicto".into(), |updated_bene| {
                updated_bene
                    .limb_damage
                    .set_limb_damaged(LType::TorsoDamage, true);
                updated_bene.set_flag(FType::Heatspear, true);
            });
        let restore_slice = AetTimeSlice {
            observations: Some(vec![
                AetObservation::SimpleCureAction(SimpleCureAction {
                    caster: "Benedicto".into(),
                    cure_type: SimpleCure::Salve("restoration".into(), "torso".into()),
                }),
                AetObservation::SimpleCureAction(SimpleCureAction {
                    caster: "Seurimas".into(),
                    cure_type: SimpleCure::Salve("restoration".into(), "torso".into()),
                }),
            ]),
            lines: vec![],
            gmcp: Vec::new(),
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        let restore_slice_two = AetTimeSlice {
            observations: Some(vec![
                AetObservation::Cured("torso_damaged".into()),
                AetObservation::LimbHeal("torso".into(), 30.0),
                AetObservation::SimpleCureAction(SimpleCureAction {
                    caster: "Benedicto".into(),
                    cure_type: SimpleCure::Salve("restoration".into(), "torso".into()),
                }),
                AetObservation::SimpleCureAction(SimpleCureAction {
                    caster: "Seurimas".into(),
                    cure_type: SimpleCure::Salve("restoration".into(), "torso".into()),
                }),
            ]),
            lines: vec![],
            gmcp: Vec::new(),
            prompt: AetPrompt::Blackout,
            time: 1000,
            me: "Seurimas".into(),
        };
        let time_slice = AetTimeSlice {
            observations: Some(vec![AetObservation::Cured("heatspear".into())]),
            lines: vec![],
            gmcp: Vec::new(),
            prompt: AetPrompt::Promptless,
            time: 2000,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(restore_slice, None as Option<&DummyDatabaseModule>);
        let seur_state = timeline.state.borrow_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Salve), false);
        assert_eq!(seur_state.get_limb_state(LType::TorsoDamage).damage, 33.33);
        assert_eq!(seur_state.is(FType::Heatspear), true);
        let bene_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Salve), false);
        assert_eq!(bene_state.get_limb_state(LType::TorsoDamage).damage, 33.33);
        assert_eq!(bene_state.is(FType::Heatspear), true);
        timeline.push_time_slice(restore_slice_two, None as Option<&DummyDatabaseModule>);
        let seur_state = timeline.state.borrow_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Salve), false);
        assert_eq!(seur_state.get_limb_state(LType::TorsoDamage).damage, 3.33);
        assert_eq!(seur_state.is(FType::Heatspear), true);
        let bene_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Salve), false);
        assert_eq!(bene_state.get_limb_state(LType::TorsoDamage).damage, 3.33);
        assert_eq!(bene_state.is(FType::Heatspear), true);
        timeline.push_time_slice(time_slice, None as Option<&DummyDatabaseModule>);
        let seur_state = timeline.state.borrow_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Salve), true);
        assert_eq!(seur_state.get_limb_state(LType::TorsoDamage).damage, 3.33);
        assert_eq!(seur_state.is(FType::Heatspear), false);
        let bene_state = timeline.state.borrow_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Salve), true);
        assert_eq!(bene_state.get_limb_state(LType::TorsoDamage).damage, 3.33);
        assert_eq!(bene_state.is(FType::Heatspear), false);
    }
}
