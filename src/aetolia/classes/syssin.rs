use crate::aetolia::alpha_beta::ActionPlanner;
use crate::aetolia::classes::*;
use crate::aetolia::curatives::get_cure_depth;
use crate::aetolia::observables::*;
use crate::aetolia::timeline::*;
use crate::aetolia::topper::*;
use crate::aetolia::types::*;
use regex::Regex;
use std::collections::HashMap;

#[cfg(test)]
mod timeline_tests {
    use super::*;
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
        let savas_state = timeline.state.get_agent(&"Savas".to_string());
        assert_eq!(savas_state.balanced(BType::Balance), false);
        assert_eq!(savas_state.is(FType::Asthma), false);
        assert_eq!(savas_state.is(FType::Anorexia), false);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
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
        let savas_state = timeline.state.get_agent(&"Savas".to_string());
        assert_eq!(savas_state.balanced(BType::Balance), false);
        assert_eq!(savas_state.is(FType::Asthma), false);
        assert_eq!(savas_state.is(FType::Anorexia), false);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
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
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.is(FType::Asthma), false);
        assert_eq!(seur_state.is(FType::Anorexia), false);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
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
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.is(FType::LeftLegBroken), false);
        assert_eq!(seur_state.is(FType::LeftArmBroken), false);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
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
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.is(FType::Asthma), false);
        assert_eq!(seur_state.is(FType::Anorexia), false);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
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
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.is(FType::Asthma), false);
        assert_eq!(seur_state.is(FType::Anorexia), false);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
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
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.is(FType::Asthma), false);
        assert_eq!(seur_state.is(FType::Anorexia), false);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
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
        let bene_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(bene_state.balanced(BType::Pill), true);
        assert_eq!(bene_state.is(FType::Stupidity), true);
        assert_eq!(bene_state.is(FType::Void), true);
        assert_eq!(bene_state.is(FType::Weakvoid), false);
        timeline.push_time_slice(dstab_slice);
        let bene_state = timeline.state.get_agent(&"Seurimas".to_string());
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
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Pill), true);
        assert_eq!(bene_state.is(FType::Stupidity), true);
        assert_eq!(bene_state.is(FType::Void), true);
        assert_eq!(bene_state.is(FType::Weakvoid), false);
        timeline.push_time_slice(dstab_slice);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
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
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Pill), true);
        assert_eq!(bene_state.is(FType::Stupidity), true);
        assert_eq!(bene_state.is(FType::Void), false);
        assert_eq!(bene_state.is(FType::Weakvoid), true);
        timeline.push_time_slice(dstab_slice);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
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
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.is(FType::Asthma), false);
        assert_eq!(seur_state.is(FType::Anorexia), false);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
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
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.is(FType::Asthma), false);
        assert_eq!(seur_state.is(FType::Anorexia), false);
        let mut bene_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Balance), true);
        assert_eq!(bene_state.is(FType::Asthma), true);
        assert_eq!(bene_state.is(FType::Anorexia), true);
        timeline.push_time_slice(cure_slice);
        let mut bene_cured_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(bene_cured_state.balanced(BType::Balance), true);
        assert_eq!(bene_cured_state.is(FType::Asthma), false);
        assert_eq!(bene_cured_state.is(FType::Anorexia), false);
        timeline.push_time_slice(relapse_slice);
        let mut bene_relapsed_state = timeline.state.get_agent(&"Benedicto".to_string());
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
        let relapse_slice_1 = AetTimeSlice {
            observations: Some(vec![
                AetObservation::Relapse("Benedicto".into()),
                AetObservation::SimpleCureAction(SimpleCureAction {
                    caster: "Benedicto".into(),
                    cure_type: SimpleCure::Salve("epidermal".into(), "head".into()),
                }),
            ]),
            lines: vec![],
            prompt: AetPrompt::Blackout,
            time: 220,
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
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.is(FType::Asthma), false);
        assert_eq!(seur_state.is(FType::Anorexia), false);
        let mut bene_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Balance), true);
        assert_eq!(bene_state.is(FType::Asthma), true);
        assert_eq!(bene_state.is(FType::Anorexia), true);
        timeline.push_time_slice(cure_slice);
        let mut bene_cured_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(bene_cured_state.balanced(BType::Balance), true);
        assert_eq!(bene_cured_state.is(FType::Asthma), false);
        assert_eq!(bene_cured_state.is(FType::Anorexia), false);
        timeline.push_time_slice(relapse_slice_1);
        let mut bene_relapsed_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(bene_relapsed_state.balanced(BType::Balance), true);
        assert_eq!(bene_relapsed_state.is(FType::Asthma), false);
        assert_eq!(bene_relapsed_state.is(FType::Anorexia), false);
        timeline.push_time_slice(relapse_slice_2);
        let mut bene_relapsed_state = timeline.state.get_agent(&"Benedicto".to_string());
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
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.is(FType::Asthma), true);
        assert_eq!(seur_state.is(FType::Anorexia), true);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
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
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.is(FType::ThinBlood), false);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
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
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.is(FType::ThinBlood), false);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
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
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.is(FType::ThinBlood), false);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
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
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.is(FType::ThinBlood), false);
        assert_eq!(seur_state.get_parrying(), None);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
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
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Equil), false);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
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
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(
            bene_state.hypno_state.hypnosis_stack.get(0),
            Some(&Hypnosis::Aff(FType::Stupidity))
        );
    }
}

#[cfg(test)]
mod action_tests {
    use super::*;

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
            "qeb parry torso;;bedazzle Benedicto;;hypnotise Benedicto;;suggest Benedicto Hypochondria%%qs shadow sleight void Benedicto",
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
            "qeb parry torso;;envenom whip with curare;;flay Benedicto;;hypnotise Benedicto;;suggest Benedicto Hypochondria%%qs shadow sleight void Benedicto",
        );
    }
}

lazy_static! {
    static ref SUGGESTION: Regex = Regex::new(r"suggest (\w+) ([^;%]+)").unwrap();
}

lazy_static! {
    static ref FLAY: Regex = Regex::new(r"flay (\w+)($|;;| (\w+) ?(\w+)?$)").unwrap();
}

lazy_static! {
    static ref ACTION: Regex = Regex::new(r"action (.*)").unwrap();
}

pub fn infer_flay_target(
    name: &String,
    agent_states: &mut AetTimelineState,
) -> Option<(FType, String)> {
    if let Some(flay) = agent_states.get_player_hint(name, &"flay".into()) {
        if let Some(captures) = FLAY.captures(&flay) {
            if let Some(def_name) = captures.get(3) {
                Some((
                    FType::from_name(&def_name.as_str().to_string()).unwrap_or(FType::Rebounding),
                    captures
                        .get(4)
                        .map(|venom_match| venom_match.as_str())
                        .unwrap_or("")
                        .to_string(),
                ))
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

pub fn infer_suggestion(name: &String, agent_states: &mut AetTimelineState) -> Hypnosis {
    if let Some(suggestion) = agent_states.get_player_hint(name, &"suggestion".into()) {
        if let Some(captures) = ACTION.captures(&suggestion) {
            Hypnosis::Action(captures.get(1).unwrap().as_str().to_string())
        } else {
            if let Some(aff) = FType::from_name(&suggestion) {
                println!("Good {:?}", aff);
                Hypnosis::Aff(aff)
            } else {
                println!("Bad {}", suggestion);
                Hypnosis::Aff(FType::Impatience)
            }
        }
    } else {
        println!("Bad, no hint");
        Hypnosis::Aff(FType::Impatience)
    }
}

pub fn handle_sent(command: &String, agent_states: &mut AetTimelineState) {
    if let Some(captures) = SUGGESTION.captures(command) {
        agent_states.add_player_hint(
            captures.get(1).unwrap().as_str(),
            &"suggestion",
            captures
                .get(2)
                .unwrap()
                .as_str()
                .to_string()
                .to_ascii_lowercase(),
        );
    }
    if let Some(captures) = FLAY.captures(command) {
        agent_states.add_player_hint(
            captures.get(1).unwrap().as_str(),
            &"flay",
            captures.get(0).unwrap().as_str().to_string(),
        );
    }
}

/**
 *
 * ActiveTransitions!
 *
**/

pub struct DoublestabAction {
    pub caster: String,
    pub target: String,
    pub venoms: (String, String),
}

impl DoublestabAction {
    pub fn new(caster: String, target: String, v1: String, v2: String) -> Self {
        DoublestabAction {
            caster,
            target,
            venoms: (v1, v2),
        }
    }
}

impl ActiveTransition for DoublestabAction {
    fn simulate(&self, _timeline: &AetTimeline) -> Vec<ProbableEvent> {
        Vec::new()
    }
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(get_dstab_action(
            &timeline,
            &self.target,
            &self.venoms.0,
            &self.venoms.1,
        ))
    }
}

pub struct FlayAction {
    pub caster: String,
    pub target: String,
    pub annotation: String,
    pub venom: String,
}

impl FlayAction {
    pub fn new(caster: String, target: String, annotation: String, venom: String) -> Self {
        FlayAction {
            caster,
            target,
            annotation,
            venom,
        }
    }

    pub fn fangbarrier(caster: String, target: String) -> Self {
        FlayAction {
            caster,
            target,
            annotation: "fangbarrier".to_string(),
            venom: "".to_string(),
        }
    }
}

impl ActiveTransition for FlayAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        let mut observations = vec![CombatAction::observation(
            &self.caster,
            &self.target,
            &"Assassination",
            &"Flay",
            &self.annotation,
        )];
        if self.venom.len() > 0
            && (self.annotation.eq_ignore_ascii_case("shield")
                || self.annotation.eq_ignore_ascii_case("rebounding"))
        {
            observations.push(AetObservation::Devenoms(self.venom.clone()));
        }
        ProbableEvent::certain(observations)
    }
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(get_flay_action(
            &timeline,
            &self.target,
            self.annotation.clone(),
            self.venom.clone(),
        ))
    }
}

pub struct SlitAction {
    pub caster: String,
    pub target: String,
    pub venom: String,
}

impl SlitAction {
    pub fn new(caster: String, target: String, venom: String) -> Self {
        SlitAction {
            caster,
            target,
            venom,
        }
    }
}

impl ActiveTransition for SlitAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        let mut observations = vec![CombatAction::observation(
            &self.caster,
            &self.target,
            &"Assassination",
            &"Slit",
            &"",
        )];
        observations.push(AetObservation::Devenoms(self.venom.clone()));
        ProbableEvent::certain(observations)
    }
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(get_slit_action(
            &timeline,
            &self.target,
            &self.venom.clone(),
        ))
    }
}

pub struct ShruggingAction {
    pub caster: String,
    pub shrugged: String,
}

impl ShruggingAction {
    pub fn shrug_asthma(caster: String) -> Self {
        ShruggingAction {
            caster,
            shrugged: "asthma".to_string(),
        }
    }
    pub fn shrug_anorexia(caster: String) -> Self {
        ShruggingAction {
            caster,
            shrugged: "anorexia".to_string(),
        }
    }
    pub fn shrug_slickness(caster: String) -> Self {
        ShruggingAction {
            caster,
            shrugged: "slickness".to_string(),
        }
    }
}

impl ActiveTransition for ShruggingAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        ProbableEvent::certain(vec![CombatAction::observation(
            &self.caster,
            &"",
            &"Assassination",
            &"Shrugging",
            &self.shrugged,
        )])
    }
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(format!("light pipes;;shrug {}", self.shrugged))
    }
}

pub struct BiteAction {
    pub caster: String,
    pub target: String,
    pub venom: String,
}

impl BiteAction {
    pub fn new(caster: &str, target: &str, venom: &str) -> Self {
        BiteAction {
            caster: caster.to_string(),
            target: target.to_string(),
            venom: venom.to_string(),
        }
    }
}

impl ActiveTransition for BiteAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        ProbableEvent::certain(vec![CombatAction::observation(
            &self.caster,
            &self.target,
            &"Assassination",
            &"Bite",
            &self.venom,
        )])
    }

    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(format!("bite {} {}", self.target, self.venom))
    }
}

pub struct BedazzleAction {
    pub caster: String,
    pub target: String,
}

impl BedazzleAction {
    pub fn new(caster: &str, target: &str) -> Self {
        BedazzleAction {
            caster: caster.to_string(),
            target: target.to_string(),
        }
    }
}

impl ActiveTransition for BedazzleAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        vec![]
    }

    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(format!("stand;;bedazzle {}", self.target))
    }
}

pub struct HypnotiseAction {
    pub caster: String,
    pub target: String,
}

impl HypnotiseAction {
    pub fn new(caster: &str, target: &str) -> Self {
        HypnotiseAction {
            caster: caster.to_string(),
            target: target.to_string(),
        }
    }
}

impl ActiveTransition for HypnotiseAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        ProbableEvent::certain(vec![CombatAction::observation(
            &self.caster,
            &self.target,
            &"Hypnosis",
            &"Hypnotise",
            &"",
        )])
    }
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(format!("hypnotise {}", self.target))
    }
}

pub struct SuggestAction {
    pub caster: String,
    pub target: String,
    pub suggestion: Hypnosis,
}

impl SuggestAction {
    pub fn new(caster: &str, target: &str, suggestion: Hypnosis) -> Self {
        SuggestAction {
            caster: caster.to_string(),
            target: target.to_string(),
            suggestion,
        }
    }
    pub fn get_suggestion(&self) -> String {
        let suggestion_string = match &self.suggestion {
            Hypnosis::Aff(aff) => format!("{:?}", aff),
            Hypnosis::Bulimia => format!("bulimia"),
            Hypnosis::Action(action) => format!("action {}", action),
        };
        format!("suggest {} {}", self.target, suggestion_string)
    }
}

impl ActiveTransition for SuggestAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        ProbableEvent::certain(vec![
            AetObservation::Sent(self.get_suggestion()),
            CombatAction::observation(&self.caster, &self.target, &"Hypnosis", &"Suggest", &""),
        ])
    }
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(self.get_suggestion())
    }
}

pub struct SealAction {
    pub caster: String,
    pub target: String,
    pub duration: usize,
}

impl SealAction {
    pub fn new(caster: &str, target: &str, duration: usize) -> Self {
        SealAction {
            caster: caster.to_string(),
            target: target.to_string(),
            duration,
        }
    }
}

impl ActiveTransition for SealAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        ProbableEvent::certain(vec![
            AetObservation::Sent(format!("seal {} {}", self.target, self.duration)),
            CombatAction::observation(&self.caster, &self.target, &"Hypnosis", &"Suggest", &""),
        ])
    }
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(format!("seal {} {}", self.target, self.duration))
    }
}

pub struct SnapAction {
    pub caster: String,
    pub target: String,
}

impl SnapAction {
    pub fn new(caster: &str, target: &str) -> Self {
        SnapAction {
            caster: caster.to_string(),
            target: target.to_string(),
        }
    }
}

impl ActiveTransition for SnapAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        ProbableEvent::certain(vec![
            AetObservation::Sent(format!("snap {}", self.target)),
            CombatAction::observation(&self.caster, &self.target, &"Hypnosis", &"Snap", &""),
        ])
    }
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(format!("snap {}", self.target))
    }
}

pub struct SleightAction {
    pub caster: String,
    pub target: String,
    pub sleight: String,
}

impl SleightAction {
    pub fn new(caster: &str, target: &str, sleight: &str) -> Self {
        SleightAction {
            caster: caster.to_string(),
            target: target.to_string(),
            sleight: sleight.to_string(),
        }
    }
}

impl ActiveTransition for SleightAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        ProbableEvent::certain(vec![CombatAction::observation(
            &self.caster,
            &self.target,
            &"Hypnosis",
            &"Sleight",
            &self.sleight,
        )])
    }
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(format!("shadow sleight {} {}", self.sleight, self.target))
    }
}

/**
 *
 * MOD ENTRY POINTS
 *
**/

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut AetTimelineState,
    _before: &Vec<AetObservation>,
    after: &Vec<AetObservation>,
) -> Result<(), String> {
    match combat_action.skill.as_ref() {
        "Doublestab" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            let mut you = agent_states.get_agent(&combat_action.target);
            apply_weapon_hits(
                &mut me,
                &mut you,
                after,
                combat_action.caster.eq(&agent_states.me),
                agent_states.get_player_hint(&combat_action.caster, &"CALLED_VENOMS".to_string()),
            )?;
            apply_or_infer_balance(&mut me, (BType::Balance, 2.65), after);
            agent_states.set_agent(&combat_action.caster, me);
            agent_states.set_agent(&combat_action.target, you);
        }
        "Slit" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            let mut you = agent_states.get_agent(&combat_action.target);
            apply_weapon_hits(
                &mut me,
                &mut you,
                after,
                combat_action.caster.eq(&agent_states.me),
                agent_states.get_player_hint(&combat_action.caster, &"CALLED_VENOMS".to_string()),
            )?;
            apply_or_infer_balance(&mut me, (BType::Balance, 1.88), after);
            agent_states.set_agent(&combat_action.caster, me);
            agent_states.set_agent(&combat_action.target, you);
        }
        "Shrugging" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            apply_or_infer_balance(&mut me, (BType::ClassCure1, 20.0), after);
            agent_states.set_agent(&combat_action.caster, me);
        }
        "Bite" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            let mut you = agent_states.get_agent(&combat_action.target);
            me.set_balance(BType::Balance, 1.9);
            if let Some(AetObservation::Parry(who, _what)) = after.get(1) {
                if !who.eq(&combat_action.target) {
                    apply_venom(&mut you, &combat_action.annotation, false)?;
                }
            } else if let Some(AetObservation::Absorbed(who, _what)) = after.get(1) {
                if !who.eq(&combat_action.target) {
                    apply_venom(&mut you, &combat_action.annotation, false)?;
                }
            } else if let Some(AetObservation::PurgeVenom(who, _what)) = after.get(1) {
                if !who.eq(&combat_action.target) {
                    apply_venom(&mut you, &combat_action.annotation, false)?;
                }
            } else {
                apply_venom(&mut you, &combat_action.annotation, false)?;
            }
            apply_or_infer_balance(&mut me, (BType::Balance, 1.9), after);
            agent_states.set_agent(&combat_action.caster, me);
            agent_states.set_agent(&combat_action.target, you);
        }
        "Sleight" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            let mut you = agent_states.get_agent(&combat_action.target);
            match combat_action.annotation.as_ref() {
                "Void" => {
                    apply_or_infer_balance(&mut me, (BType::Secondary, 6.0), after);
                    you.set_flag(FType::Void, true);
                }
                _ => {}
            }
            agent_states.set_agent(&combat_action.caster, me);
            agent_states.set_agent(&combat_action.target, you);
        }
        "Marks" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            let mut you = agent_states.get_agent(&combat_action.target);
            match combat_action.annotation.as_ref() {
                "Numbness" => {
                    apply_or_infer_balance(&mut me, (BType::Balance, 3.0), after);
                    apply_or_infer_balance(&mut me, (BType::Secondary, 3.0), after);
                    you.set_flag(FType::NumbedSkin, true);
                }
                "Fatigue" => {
                    apply_or_infer_balance(&mut me, (BType::Balance, 3.0), after);
                    apply_or_infer_balance(&mut me, (BType::Secondary, 3.0), after);
                    you.set_flag(FType::MentalFatigue, true);
                }
                _ => {}
            }
            agent_states.set_agent(&combat_action.caster, me);
            agent_states.set_agent(&combat_action.target, you);
        }
        "Flay" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            let mut you = agent_states.get_agent(&combat_action.target);
            me.set_balance(BType::Balance, 1.9);
            apply_or_infer_balance(&mut me, (BType::Balance, 1.9), after);
            if combat_action.annotation.eq(&"rebounding") || combat_action.annotation.eq(&"shield")
            {
                apply_weapon_hits(
                    &mut me,
                    &mut you,
                    after,
                    combat_action.caster.eq(&agent_states.me),
                    agent_states
                        .get_player_hint(&combat_action.caster, &"CALLED_VENOMS".to_string()),
                )?;
            }
            match combat_action.annotation.as_ref() {
                "rebounding" => {
                    you.set_flag(FType::Rebounding, false);
                }
                "failure-rebounding" => {
                    you.set_flag(FType::Rebounding, false);
                }
                "fangbarrier" => {
                    you.set_flag(FType::Fangbarrier, false);
                }
                "failure-fangbarrier" => {
                    you.set_flag(FType::Fangbarrier, false);
                }
                "shield" => {
                    you.set_flag(FType::Shielded, false);
                }
                "failure-shield" => {
                    you.set_flag(FType::Shielded, false);
                }
                "speed" => {
                    you.set_flag(FType::Speed, false);
                }
                "cloak" => {
                    you.set_flag(FType::Cloak, false);
                }
                _ => {}
            }
            if infer_flay_target(&combat_action.target, agent_states).is_none() {
                remove_through(
                    &mut you,
                    match combat_action.annotation.as_ref() {
                        "rebounding" => FType::Rebounding,
                        "fangbarrier" => FType::Fangbarrier,
                        "shield" => FType::Shielded,
                        "speed" => FType::Speed,
                        "cloak" => FType::Cloak,
                        _ => FType::Cloak,
                    },
                    &FLAY_ORDER.to_vec(),
                )
            }
            agent_states.set_agent(&combat_action.caster, me);
            agent_states.set_agent(&combat_action.target, you);
        }
        "Hypnotise" => {
            let mut you = agent_states.get_agent(&combat_action.target);
            you.hypno_state.hypnotize();
            agent_states.set_agent(&combat_action.target, you);
        }
        "Desway" => {
            let mut you = agent_states.get_agent(&combat_action.target);
            you.hypno_state.desway();
            agent_states.set_agent(&combat_action.target, you);
        }
        "Seal" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            let mut you = agent_states.get_agent(&combat_action.target);
            apply_or_infer_balance(&mut me, (BType::Equil, 3.0), after);
            you.hypno_state.seal(3.0);
            agent_states.set_agent(&combat_action.caster, me);
            agent_states.set_agent(&combat_action.target, you);
        }
        "Suggest" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            let mut you = agent_states.get_agent(&combat_action.target);
            apply_or_infer_balance(&mut me, (BType::Equil, 2.25), after);
            you.hypno_state
                .push_suggestion(infer_suggestion(&combat_action.target, agent_states));
            agent_states.set_agent(&combat_action.caster, me);
            agent_states.set_agent(&combat_action.target, you);
        }
        "Fizzle" => {
            let mut me = agent_states.get_agent(&combat_action.target);
            me.hypno_state.pop_suggestion(false);
            agent_states.set_agent(&combat_action.target, me);
        }
        "Snap" => {
            if let Some(target) =
                agent_states.get_player_hint(&combat_action.caster, &"snap".into())
            {
                let mut you = agent_states.get_agent(&target);
                if you.hypno_state.sealed.is_some() {
                    you.hypno_state.activate();
                }
                agent_states.set_agent(&target, you);
            } else if !combat_action.target.eq(&"".to_string()) {
                let mut you = agent_states.get_agent(&combat_action.target);
                if you.hypno_state.sealed.is_some() {
                    you.hypno_state.activate();
                }
                agent_states.set_agent(&combat_action.target, you);
            }
        }
        "Bedazzle" => {
            let mut you = agent_states.get_agent(&combat_action.target);
            apply_or_infer_random_afflictions(&mut you, after)?;
            agent_states.set_agent(&combat_action.target, you);
            let mut me = agent_states.get_agent(&combat_action.caster);
            apply_or_infer_balance(&mut me, (BType::Balance, 2.75), after);
            agent_states.set_agent(&combat_action.caster, me);
        }
        "Fire" => {
            let mut you = agent_states.get_agent(&combat_action.target);
            apply_or_infer_suggestion(&mut you, after)?;
            agent_states.set_agent(&combat_action.target, you)
        }
        _ => {}
    }
    Ok(())
}

lazy_static! {
    static ref COAG_STACK: Vec<FType> = vec![
        FType::Allergies,
        FType::Vomiting,
        FType::Clumsiness,
        FType::Asthma,
        FType::Shyness,
        FType::Stupidity,
        FType::Paresis,
        FType::Sensitivity,
        FType::LeftLegBroken,
    ];
}

lazy_static! {
    static ref DEC_STACK: Vec<FType> = vec![
        FType::Clumsiness,
        FType::Weariness,
        FType::Asthma,
        FType::Stupidity,
        FType::Paresis,
        FType::Allergies,
        FType::Vomiting,
        FType::LeftLegBroken,
        FType::LeftArmBroken,
        FType::Shyness,
    ];
}

lazy_static! {
    static ref KILL_STACK: Vec<FType> = vec![
        FType::Allergies,
        FType::Vomiting,
        FType::Sensitivity,
        FType::Recklessness,
        FType::Asthma,
        FType::Paresis,
        FType::Slickness,
        FType::Anorexia,
        FType::LeftLegBroken,
        FType::LeftArmBroken,
        FType::RightLegBroken,
        FType::RightArmBroken,
    ];
}

lazy_static! {
    static ref FIRE_STACK: Vec<FType> = vec![
        FType::Paresis,
        FType::Clumsiness,
        FType::Asthma,
        FType::Shyness,
        FType::Stupidity,
        FType::Allergies,
        FType::Vomiting,
        FType::LeftLegBroken,
        FType::LeftArmBroken,
        FType::RightLegBroken,
        FType::RightArmBroken,
        FType::Voyria,
        FType::Stuttering,
    ];
}

lazy_static! {
    static ref PHYS_STACK: Vec<FType> = vec![
        FType::Paresis,
        FType::Clumsiness,
        FType::Allergies,
        FType::Vomiting,
        FType::Asthma,
        FType::Dizziness,
        FType::Weariness,
        FType::Slickness,
        FType::LeftArmBroken,
        FType::RightArmBroken,
        FType::LeftLegBroken,
        FType::RightLegBroken,
    ];
}

lazy_static! {
    static ref GANK_STACK: Vec<FType> = vec![
        FType::Paresis,
        FType::Asthma,
        FType::Clumsiness,
        FType::Squelched,
        FType::Disfigurement,
        FType::Slickness,
        FType::Stupidity,
        FType::Anorexia,
        FType::Dizziness,
        FType::LeftLegBroken,
        FType::Stuttering,
        FType::RightLegBroken,
        FType::LeftArmBroken,
        FType::RightArmBroken,
    ];
}

lazy_static! {
    static ref MONK_STACK: Vec<FType> = vec![
        FType::Allergies,
        FType::Weariness,
        FType::Paresis,
        FType::Stupidity,
        FType::Dizziness,
        FType::Clumsiness,
        FType::Vomiting,
        FType::Asthma,
        FType::LeftArmBroken,
        FType::RightArmBroken,
        FType::LeftLegBroken,
        FType::RightLegBroken,
    ];
}

lazy_static! {
    static ref PEACE_STACK: Vec<FType> = vec![
        FType::Paresis,
        FType::Asthma,
        FType::Clumsiness,
        FType::Allergies,
        FType::Stupidity,
        FType::Peace,
        FType::Vomiting,
        FType::Slickness,
        FType::LeftArmBroken,
        FType::RightArmBroken,
        FType::Dizziness,
        FType::LeftLegBroken,
        FType::RightLegBroken,
    ];
}

lazy_static! {
    static ref YEDAN_STACK: Vec<FType> = vec![
        FType::Slickness,
        FType::Paresis,
        FType::Anorexia,
        FType::Stupidity,
        FType::Clumsiness,
        FType::Weariness,
        FType::Asthma,
        FType::Allergies,
        FType::Dizziness,
        FType::Vomiting,
        FType::LeftLegBroken,
        FType::RightLegBroken,
    ];
}

lazy_static! {
    static ref BEDAZZLE_STACK: Vec<FType> = vec![
        FType::Allergies,
        FType::Clumsiness,
        FType::Asthma,
        FType::Paresis,
        FType::Slickness,
        FType::LeftArmBroken,
        FType::RightArmBroken,
        FType::Vomiting,
        FType::Stupidity,
        FType::LeftLegBroken,
        FType::RightLegBroken,
    ];
}

lazy_static! {
    static ref AGGRO_STACK: Vec<FType> = vec![
        FType::Paresis,
        FType::Asthma,
        FType::Clumsiness,
        FType::Allergies,
        FType::Stupidity,
        FType::Vomiting,
        FType::Slickness,
        FType::LeftArmBroken,
        FType::RightArmBroken,
        FType::Dizziness,
        FType::LeftLegBroken,
        FType::RightLegBroken,
    ];
}

lazy_static! {
    static ref SALVE_STACK: Vec<FType> = vec![
        FType::Stuttering,
        FType::LeftLegBroken,
        FType::RightLegBroken,
        FType::LeftArmBroken,
        FType::RightArmBroken,
        FType::Anorexia,
        FType::Asthma,
        FType::Slickness,
        FType::Paresis,
        FType::Stupidity,
    ];
}

lazy_static! {
    static ref SLIT_STACK: Vec<VenomPlan> = vec![
        VenomPlan::OnTree(FType::Paresis),
        VenomPlan::IfNotDo(
            FType::Hypersomnia,
            Box::new(VenomPlan::OneOf(FType::Vomiting, FType::Allergies))
        ),
        VenomPlan::Stick(FType::Haemophilia),
        VenomPlan::OneOf(FType::Stupidity, FType::Dizziness),
        VenomPlan::OneOf(FType::Asthma, FType::Weariness),
        VenomPlan::OneOf(FType::Recklessness, FType::Clumsiness),
        VenomPlan::OneOf(FType::Slickness, FType::Anorexia),
        VenomPlan::OneOf(FType::LeftLegBroken, FType::LeftArmBroken),
        VenomPlan::OneOf(FType::RightLegBroken, FType::RightArmBroken),
        VenomPlan::Stick(FType::Anorexia),
    ];
}

lazy_static! {
    static ref THIN_STACK: Vec<VenomPlan> = vec![
        VenomPlan::OnTree(FType::Paresis),
        VenomPlan::IfDo(
            FType::ThinBlood,
            Box::new(VenomPlan::OneOf(FType::Vomiting, FType::Allergies))
        ),
        VenomPlan::IfNotDo(
            FType::ThinBlood,
            Box::new(VenomPlan::Stick(FType::Allergies)),
        ),
        VenomPlan::IfNotDo(
            FType::ThinBlood,
            Box::new(VenomPlan::Stick(FType::Vomiting)),
        ),
        VenomPlan::Stick(FType::Asthma),
        VenomPlan::OneOf(FType::Clumsiness, FType::Weariness),
        VenomPlan::IfDo(
            FType::Loneliness,
            Box::new(VenomPlan::OneOf(FType::Recklessness, FType::Sensitivity))
        ),
        VenomPlan::Stick(FType::Slickness),
        VenomPlan::Stick(FType::Paresis),
        VenomPlan::OneOf(FType::LeftLegBroken, FType::LeftArmBroken),
        VenomPlan::OneOf(FType::RightLegBroken, FType::RightArmBroken),
    ];
}

lazy_static! {
    static ref CARNIFEX_STACK: Vec<VenomPlan> = vec![
        VenomPlan::OnTree(FType::Paresis),
        VenomPlan::Stick(FType::Clumsiness),
        VenomPlan::Stick(FType::Vomiting),
        VenomPlan::Stick(FType::Allergies),
        VenomPlan::OneOf(FType::Sensitivity, FType::Dizziness),
        VenomPlan::OneOf(FType::Stupidity, FType::Weariness),
        VenomPlan::OneOf(FType::Asthma, FType::Slickness),
        VenomPlan::OneOf(FType::LeftLegBroken, FType::LeftArmBroken),
        VenomPlan::OneOf(FType::RightLegBroken, FType::RightLegBroken),
        VenomPlan::OneOf(FType::Sensitivity, FType::Dizziness),
    ];
}

lazy_static! {
    static ref WAYFARER_STACK: Vec<VenomPlan> = vec![
        VenomPlan::OnTree(FType::Paresis),
        VenomPlan::Stick(FType::Clumsiness),
        VenomPlan::Stick(FType::Asthma),
        VenomPlan::OneOf(FType::Allergies, FType::Vomiting),
        VenomPlan::OneOf(FType::Weariness, FType::Stupidity),
        VenomPlan::OneOf(FType::Slickness, FType::Anorexia),
        VenomPlan::OneOf(FType::LeftLegBroken, FType::LeftArmBroken),
        VenomPlan::OneOf(FType::RightLegBroken, FType::RightLegBroken),
        VenomPlan::OneOf(FType::Sensitivity, FType::Dizziness),
    ];
}

lazy_static! {
    static ref ZEALOT_STACK: Vec<VenomPlan> = vec![
        VenomPlan::Stick(FType::Clumsiness),
        VenomPlan::Stick(FType::Asthma),
        VenomPlan::OneOf(FType::Allergies, FType::Vomiting),
        VenomPlan::OneOf(FType::Weariness, FType::Stupidity),
        VenomPlan::OneOf(FType::Slickness, FType::Anorexia),
        VenomPlan::OneOf(FType::LeftLegBroken, FType::LeftArmBroken),
        VenomPlan::OneOf(FType::RightLegBroken, FType::RightLegBroken),
        VenomPlan::OneOf(FType::Sensitivity, FType::Dizziness),
    ];
}

lazy_static! {
    static ref SYSSIN_STACK: Vec<VenomPlan> = vec![
        VenomPlan::Stick(FType::Paresis),
        VenomPlan::Stick(FType::Clumsiness),
        VenomPlan::Stick(FType::Asthma),
        VenomPlan::OneOf(FType::Allergies, FType::Vomiting),
        VenomPlan::OneOf(FType::Weariness, FType::Stupidity),
        VenomPlan::OneOf(FType::Slickness, FType::Anorexia),
        VenomPlan::OneOf(FType::LeftLegBroken, FType::LeftArmBroken),
        VenomPlan::OneOf(FType::RightLegBroken, FType::RightLegBroken),
        VenomPlan::OneOf(FType::Sensitivity, FType::Dizziness),
    ];
}

lazy_static! {
    static ref PRAENOMEN_STACK: Vec<VenomPlan> = vec![
        VenomPlan::Stick(FType::Paresis),
        VenomPlan::Stick(FType::Asthma),
        VenomPlan::OneOf(FType::Weariness, FType::Stupidity),
        VenomPlan::OneOf(FType::Allergies, FType::Vomiting),
        VenomPlan::OneOf(FType::Haemophilia, FType::Dizziness),
        VenomPlan::OneOf(FType::Slickness, FType::Anorexia),
        VenomPlan::OneOf(FType::LeftLegBroken, FType::LeftArmBroken),
        VenomPlan::OneOf(FType::RightLegBroken, FType::RightLegBroken),
        VenomPlan::OneOf(FType::Sensitivity, FType::Dizziness),
    ];
}

lazy_static! {
    static ref INDORANI_STACK: Vec<VenomPlan> = vec![
        VenomPlan::OnTree(FType::Paresis),
        VenomPlan::Stick(FType::Asthma),
        VenomPlan::IfNotDo(
            FType::Hypochondria,
            Box::new(VenomPlan::Stick(FType::Clumsiness))
        ),
        VenomPlan::OneOf(FType::Paresis, FType::Allergies),
        VenomPlan::OneOf(FType::Disfigurement, FType::Weariness),
        VenomPlan::OneOf(FType::Slickness, FType::Anorexia),
        VenomPlan::OneOf(FType::LeftLegBroken, FType::LeftArmBroken),
        VenomPlan::OneOf(FType::RightLegBroken, FType::RightLegBroken),
        VenomPlan::OneOf(FType::Sensitivity, FType::Dizziness),
    ];
}

lazy_static! {
    static ref LUMINARY_STACK: Vec<VenomPlan> = vec![
        VenomPlan::OnTree(FType::Paresis),
        VenomPlan::Stick(FType::Asthma),
        VenomPlan::IfNotDo(
            FType::Hypochondria,
            Box::new(VenomPlan::Stick(FType::Clumsiness))
        ),
        VenomPlan::OneOf(FType::Paresis, FType::Allergies),
        VenomPlan::OneOf(FType::Peace, FType::Vomiting),
        VenomPlan::OneOf(FType::Slickness, FType::Anorexia),
        VenomPlan::OneOf(FType::LeftLegBroken, FType::LeftArmBroken),
        VenomPlan::OneOf(FType::RightLegBroken, FType::RightArmBroken),
        VenomPlan::OneOf(FType::Sensitivity, FType::Dizziness),
    ];
}

lazy_static! {
    static ref SHAMAN_STACK: Vec<VenomPlan> = vec![
        VenomPlan::Stick(FType::Allergies),
        VenomPlan::Stick(FType::Paresis),
        VenomPlan::OneOf(FType::Asthma, FType::Clumsiness),
        VenomPlan::OneOf(FType::Vomiting, FType::Stupidity),
        VenomPlan::OneOf(FType::Peace, FType::Weariness),
        VenomPlan::OneOf(FType::Slickness, FType::Anorexia),
        VenomPlan::OneOf(FType::LeftLegBroken, FType::LeftArmBroken),
        VenomPlan::OneOf(FType::RightLegBroken, FType::RightArmBroken),
        VenomPlan::OneOf(FType::Dizziness, FType::Squelched),
    ];
}

lazy_static! {
    pub static ref SOFT_STACK: Vec<FType> = vec![FType::Slickness, FType::Asthma, FType::Anorexia];
}

lazy_static! {
    static ref SOFT_BUFFER: Vec<FType> = vec![FType::Clumsiness, FType::Stupidity];
}

lazy_static! {
    static ref THIN_BUFFER_STACK: Vec<FType> = vec![FType::Allergies, FType::Vomiting];
}

lazy_static! {
    static ref LOCK_BUFFER_STACK: Vec<FType> =
        vec![FType::Paresis, FType::Stupidity, FType::Clumsiness];
}

lazy_static! {
    static ref BEDAZZLE_AFFS: Vec<FType> = vec![
        FType::Vomiting,
        FType::Stuttering,
        FType::BlurryVision,
        FType::Dizziness,
        FType::Weariness,
        FType::Laxity,
    ];
}

lazy_static! {
    static ref FLAY_ORDER: Vec<FType> = vec![
        FType::Shielded,
        FType::Rebounding,
        FType::Fangbarrier,
        FType::Speed,
        FType::Cloak,
    ];
}

lazy_static! {
    static ref STACKING_STRATEGIES: HashMap<String, Vec<VenomPlan>> = {
        let mut val = HashMap::new();
        val.insert("coag".into(), get_simple_plan(COAG_STACK.to_vec()));
        val.insert("dec".into(), get_simple_plan(DEC_STACK.to_vec()));
        val.insert("phys".into(), get_simple_plan(PHYS_STACK.to_vec()));
        val.insert("gank".into(), get_simple_plan(GANK_STACK.to_vec()));
        val.insert("fire".into(), get_simple_plan(FIRE_STACK.to_vec()));
        val.insert("kill".into(), get_simple_plan(KILL_STACK.to_vec()));
        val.insert("aggro".into(), get_simple_plan(AGGRO_STACK.to_vec()));
        val.insert("salve".into(), get_simple_plan(SALVE_STACK.to_vec()));
        val.insert("peace".into(), get_simple_plan(PEACE_STACK.to_vec()));
        val.insert("slit".into(), SLIT_STACK.to_vec());
        val.insert("Monk".into(), get_simple_plan(MONK_STACK.to_vec()));
        val.insert("Luminary".into(), LUMINARY_STACK.to_vec());
        val.insert("Carnifex".into(), CARNIFEX_STACK.to_vec());
        val.insert("Wayfarer".into(), WAYFARER_STACK.to_vec());
        val.insert("Praenomen".into(), PRAENOMEN_STACK.to_vec());
        val.insert("Syssin".into(), SYSSIN_STACK.to_vec());
        val.insert("Shaman".into(), SHAMAN_STACK.to_vec());
        val.insert("Templar".into(), get_simple_plan(PHYS_STACK.to_vec()));
        val.insert("Indorani".into(), INDORANI_STACK.to_vec());
        val.insert("Zealot".into(), ZEALOT_STACK.to_vec());
        val.insert("yedan".into(), get_simple_plan(YEDAN_STACK.to_vec()));
        val.insert("bedazzle".into(), get_simple_plan(BEDAZZLE_STACK.to_vec()));
        val.insert("thin".into(), THIN_STACK.to_vec());
        val
    };
}

lazy_static! {
    static ref HARD_HYPNO: Vec<Hypnosis> = vec![
        Hypnosis::Aff(FType::Hypochondria),
        Hypnosis::Aff(FType::Impatience),
        Hypnosis::Aff(FType::Loneliness),
        Hypnosis::Aff(FType::Hypochondria),
        Hypnosis::Aff(FType::Impatience),
        Hypnosis::Aff(FType::Vertigo),
        Hypnosis::Aff(FType::Impatience),
        Hypnosis::Aff(FType::Loneliness),
    ];
}

pub fn get_hypno_str(target: &String, hypno: &Hypnosis) -> String {
    match hypno {
        Hypnosis::Aff(affliction) => format!("suggest {} {:?}", target, affliction),
        Hypnosis::Action(act) => format!("suggest {} action {}", target, act),
        Hypnosis::Bulimia => format!("suggest {} bulimia", target),
    }
}

pub fn get_top_hypno<'s>(
    me: &String,
    target_name: &String,
    target: &AgentState,
    hypnos: &Vec<Hypnosis>,
) -> Option<Box<ActiveTransition>> {
    let mut hypno_idx = 0;
    for i in 0..target.hypno_state.hypnosis_stack.len() {
        if target.hypno_state.hypnosis_stack.get(i) == hypnos.get(hypno_idx) {
            hypno_idx += 1;
        }
    }
    if hypno_idx < hypnos.len() {
        if let Some(next_hypno) = hypnos.get(hypno_idx) {
            if !target.hypno_state.hypnotized {
                Some(Box::new(SeparatorAction::pair(
                    Box::new(HypnotiseAction::new(&me, &target_name)),
                    Box::new(SuggestAction::new(&me, &target_name, next_hypno.clone())),
                )))
            } else {
                Some(Box::new(SuggestAction::new(
                    &me,
                    &target_name,
                    next_hypno.clone(),
                )))
            }
        } else {
            panic!(
                "get_top_hypno: Len checked {} vs {}",
                hypno_idx,
                hypnos.len()
            )
        }
    } else if target.hypno_state.hypnotized {
        Some(Box::new(SealAction::new(&me, &target_name, 3)))
    } else {
        None
    }
}

fn check_config_str(timeline: &AetTimeline, value: &String) -> String {
    timeline.state.get_my_hint(value).unwrap_or("n".to_string())
}

fn check_config(timeline: &AetTimeline, value: &String) -> bool {
    timeline
        .state
        .get_my_hint(value)
        .unwrap_or("false".to_string())
        .eq(&"true")
}

fn check_config_int(timeline: &AetTimeline, value: &String) -> i32 {
    timeline
        .state
        .get_my_hint(value)
        .unwrap_or("0".to_string())
        .parse::<i32>()
        .unwrap()
}

fn use_one_rag(timeline: &AetTimeline) -> bool {
    check_config(timeline, &"ONE_RAG".to_string())
}

fn should_call_venoms(timeline: &AetTimeline) -> bool {
    check_config(timeline, &"VENOM_CALLING".to_string())
}

fn should_void(timeline: &AetTimeline) -> bool {
    !check_config(timeline, &"NO_VOID".to_string())
}

fn should_slit(me: &AgentState, target: &AgentState, strategy: &String) -> bool {
    if !target.is_prone() {
        false
    } else if target.is(FType::Asleep) {
        true
    } else if target.is(FType::Haemophilia)
        && target.affs_count(&vec![FType::Lethargy, FType::Allergies, FType::Vomiting]) >= 1
    {
        true
    } else {
        false
    }
}

fn should_bedazzle(
    me: &AgentState,
    target: &AgentState,
    strategy: &String,
    before_flay: bool,
) -> bool {
    if !before_flay && me.is(FType::LeftArmBroken) && !me.is(FType::RightArmBroken) {
        true
    } else if before_flay && me.is(FType::RightArmBroken) && !me.is(FType::LeftArmBroken) {
        true
    } else if target.affs_count(&BEDAZZLE_AFFS.to_vec()) >= 5 {
        false
    } else if strategy.eq_ignore_ascii_case("bedazzle")
        && target.affs_count(&vec![FType::Vomiting, FType::Laxity, FType::Weariness]) < 2
        && !target.is(FType::ThinBlood)
        && !target.lock_duration().is_some()
    {
        true
    } else if strategy.eq_ignore_ascii_case("bedazzle")
        && (me.is(FType::Clumsiness) || target.is(FType::Rebounding))
        && target.affs_count(&vec![
            FType::Vomiting,
            FType::Laxity,
            FType::Weariness,
            FType::Dizziness,
        ]) < 3
        && !target.is(FType::ThinBlood)
        && !target.lock_duration().is_some()
    {
        true
    } else {
        false
    }
}

fn should_regenerate(timeline: &AetTimeline, me: &String) -> bool {
    let me = timeline.state.borrow_agent(me);
    if me.balanced(BType::Regenerate) {
        false
    } else if let Some((_limb, damage, regenerating)) = me.get_restoring() {
        !regenerating && damage > 4000
    } else {
        false
    }
}

fn needs_restore(timeline: &AetTimeline, me: &String) -> bool {
    let me = timeline.state.borrow_agent(me);
    me.restore_count() > 0
        && me.restore_count() < 3
        && me.is(FType::Fallen)
        && me.get_balance(BType::Salve) > 2.5
}

fn needs_shrugging(timeline: &AetTimeline, me: &String) -> bool {
    let me = timeline.state.borrow_agent(me);
    me.balanced(BType::ClassCure1)
        && me.is(FType::Asthma)
        && me.is(FType::Anorexia)
        && me.is(FType::Slickness)
        && (!me.balanced(BType::Tree) || me.is(FType::Paresis) || me.is(FType::Paralysis))
        && (!me.balanced(BType::Focus) || me.is(FType::Impatience) || me.is(FType::Stupidity))
}

fn go_for_thin_blood(_timeline: &AetTimeline, you: &AgentState, _strategy: &String) -> bool {
    let mut buffer_count = 0;
    if you.is(FType::Lethargy) {
        buffer_count = buffer_count + 1;
    }
    if you.is(FType::Vomiting) {
        buffer_count = buffer_count + 1;
    }
    if you.is(FType::Allergies) {
        buffer_count = buffer_count + 1;
    }
    (buffer_count >= 2 || (buffer_count >= 1 && !you.is(FType::Fangbarrier)))
        && !you.is(FType::ThinBlood)
        && (!you.is(FType::Fangbarrier) || you.get_balance(BType::Tree) > 3.0)
        && (!you.is(FType::Fangbarrier) || you.get_balance(BType::Renew) > 8.0)
}

pub fn should_lock(me: Option<&AgentState>, you: &AgentState, lockers: &Vec<&str>) -> bool {
    if let Some(me) = me {
        if lockers.len() == 2
            && ((you.dodge_state.can_dodge_at(me.get_qeb_balance())
                && you.affs_count(&vec![
                    FType::Hypochondria,
                    FType::Clumsiness,
                    FType::Weariness,
                ]) < 1)
                || you.is(FType::Hypersomnia))
        {
            return false;
        }
    }
    (!you.can_focus(true) || you.is(FType::Stupidity) || you.get_balance(BType::Focus) > 2.5)
        && (!you.can_tree(true) || you.get_balance(BType::Tree) > 2.5)
        && lockers.len() < 3
        && lockers.len() > 0
        && (you.aff_count() >= 4 || you.get_balance(BType::Renew) > 4.0)
}

pub fn call_venom(target: &String, v1: &String) -> String {
    format!("wt Afflicting {}: {}", target, v1)
}

pub fn call_venoms(target: &String, v1: &String, v2: &String) -> String {
    format!("wt Afflicting {}: {}, {}", target, v1, v2)
}

pub fn get_flay_action(timeline: &AetTimeline, target: &String, def: String, v1: String) -> String {
    let action = if use_one_rag(timeline) && !v1.eq_ignore_ascii_case("") {
        format!("stand;;hw {};;flay {}", v1, target)
    } else if def.eq_ignore_ascii_case("rebounding") || def.eq_ignore_ascii_case("shield") {
        format!("stand;;envenom whip with {};;flay {}", v1, target)
    } else {
        format!("stand;;flay {} {} {}", target, def, v1)
    };
    let action = if should_call_venoms(timeline) && !v1.eq_ignore_ascii_case("") {
        format!("{};;{}", call_venom(target, &v1), action)
    } else {
        action
    };

    action
}

pub fn get_dstab_action(
    timeline: &AetTimeline,
    target: &String,
    v1: &String,
    v2: &String,
) -> String {
    let action = if use_one_rag(timeline) {
        format!("hr {};;hr {};;stand;;dstab {};;dash d", v2, v1, target)
    } else {
        format!("stand;;dstab {} {} {};;dash d", target, v1, v2)
    };
    if should_call_venoms(timeline) {
        format!("{};;{}", call_venoms(target, v1, v2), action)
    } else {
        action
    }
}

pub fn get_slit_action(timeline: &AetTimeline, target: &String, v1: &String) -> String {
    let action = if use_one_rag(timeline) {
        format!("stand;;hr {};;dstab {};;dash d", v1, target)
    } else {
        format!("stand;;slit {} {};;dash d", target, v1)
    };
    if should_call_venoms(timeline) {
        format!("{};;{}", call_venom(target, v1), action)
    } else {
        action
    }
}

pub fn add_delphs(
    timeline: &AetTimeline,
    me: &AgentState,
    you: &AgentState,
    strategy: &String,
    venoms: &mut Vec<&'static str>,
) {
    if you.is(FType::Allergies) || you.is(FType::Vomiting) {
        return;
    }
    if you.is(FType::Hypersomnia) {
        if you.is(FType::Insomnia) {
            venoms.push("delphinium");
        }
        if !you.is(FType::Asleep) {
            venoms.push("delphinium");
        }
        if you.is(FType::Instawake) {
            venoms.push("delphinium");
        }
        if venoms.len() >= 2 && Some(&"darkshade") == venoms.get(venoms.len() - 2) {
            venoms.remove(venoms.len() - 2);
        }
        if venoms.len() >= 2 && Some(&"euphorbia") == venoms.get(venoms.len() - 2) {
            venoms.remove(venoms.len() - 2);
        }
    } else if !you.is(FType::Insomnia) {
        venoms.push("delphinium");
        if you.is(FType::Instawake) {
            venoms.push("delphinium");
        }
    }
}

pub fn get_stack<'s>(
    timeline: &AetTimeline,
    target: &String,
    strategy: &String,
    db: Option<&DatabaseModule>,
) -> Option<Vec<VenomPlan>> {
    if strategy.eq("class") {
        if let Some(class) = db.and_then(|db| db.get_class(target)) {
            let class_name = format!("{:?}", class);
            if STACKING_STRATEGIES.contains_key(&class_name) {
                return STACKING_STRATEGIES.get(&class_name).cloned();
            } else if is_affected_by(&class, FType::Clumsiness) {
                return STACKING_STRATEGIES.get("phys").cloned();
            } else if is_affected_by(&class, FType::Peace) {
                return STACKING_STRATEGIES.get("peace").cloned();
            } else {
                return STACKING_STRATEGIES.get("aggro").cloned();
            }
        } else {
            return STACKING_STRATEGIES.get("aggro").cloned();
        }
    }
    db.and_then(|db| db.get_venom_plan(&format!("syssin_{}", strategy)))
        .or(STACKING_STRATEGIES.get(strategy).cloned())
}

pub fn get_balance_attack<'s>(
    timeline: &AetTimeline,
    who_am_i: &String,
    target: &String,
    strategy: &String,
    db: Option<&DatabaseModule>,
) -> Box<dyn ActiveTransition> {
    if let Some(stack) = get_stack(timeline, target, strategy, db) {
        let me = timeline.state.borrow_agent(who_am_i);
        let you = timeline.state.borrow_agent(target);
        if needs_shrugging(&timeline, who_am_i) {
            return Box::new(ShruggingAction::shrug_asthma(who_am_i.to_string()));
        } else if needs_restore(&timeline, who_am_i) {
            return Box::new(RestoreAction::new(who_am_i.to_string()));
        } else if let Ok(true) = get_equil_attack(timeline, who_am_i, target, strategy, db)
            .act(&timeline)
            .map(|act| act.starts_with("seal"))
        {
            return Box::new(Inactivity);
        } else if you.is(FType::Shielded)
            || you.is(FType::Rebounding)
            || you.will_be_rebounding(me.get_qeb_balance())
        {
            if !you.is(FType::Shielded) && should_bedazzle(&me, &you, &strategy, true) {
                return Box::new(BedazzleAction::new(who_am_i, &target));
            }
            let defense = if you.is(FType::Shielded) {
                "shield"
            } else {
                "rebounding"
            };
            if let Some(venom) = get_venoms_from_plan(&stack.to_vec(), 1, &you).pop() {
                return Box::new(FlayAction::new(
                    who_am_i.to_string(),
                    target.to_string(),
                    defense.to_string(),
                    venom.to_string(),
                ));
            } else {
                return Box::new(FlayAction::new(
                    who_am_i.to_string(),
                    target.to_string(),
                    defense.to_string(),
                    "".to_string(),
                ));
            }
        } else {
            let mut venoms = get_venoms_from_plan(&stack.to_vec(), 2, &you);
            let lockers = get_venoms(SOFT_STACK.to_vec(), 3, &you);
            let mut priority_buffer = false;
            if !strategy.eq("slit") && should_lock(Some(&me), &you, &lockers) {
                add_buffers(&mut venoms, &lockers);
                priority_buffer = true;
            } else if !strategy.eq("slit") && lockers.len() == 0 {
                let buffer = get_venoms(LOCK_BUFFER_STACK.to_vec(), 2, &you);
                add_buffers(&mut venoms, &buffer);
                priority_buffer = buffer.len() > 0;
            }
            if !priority_buffer {
                if go_for_thin_blood(timeline, &you, strategy) {
                    if you.is(FType::Fangbarrier) {
                        return Box::new(FlayAction::fangbarrier(
                            who_am_i.to_string(),
                            target.to_string(),
                        ));
                    } else {
                        return Box::new(BiteAction::new(who_am_i, &target, &"scytherus"));
                    }
                }
                let mut buffer = get_venoms(THIN_BUFFER_STACK.to_vec(), 2, &you);
                if strategy.eq("thin") {
                    buffer.clear();
                }
                if you.lock_duration().map_or(false, |dur| dur > 10.0) && !you.is(FType::Voyria) {
                    buffer.insert(buffer.len(), "voyria");
                }
                if you.is(FType::ThinBlood) && buffer.len() > 0 {
                    add_buffers(&mut venoms, &buffer);
                } else if !you.can_tree(false) {
                    let mut hypno_buffers = vec![];
                    let mut buffer_count = 1;
                    if you.is(FType::Impatience)
                        || you.hypno_state.get_next_hypno_aff() == Some(FType::Impatience)
                    {
                        if you.is(FType::Impatience) {
                            hypno_buffers.push(FType::Stupidity);
                            match check_config_int(timeline, &"SYSSIN_IMPATIENCE_DEPTH".to_string())
                            {
                                3 => {
                                    hypno_buffers.push(FType::Shyness);
                                    hypno_buffers.push(FType::Dizziness);
                                    buffer_count = 2;
                                }
                                2 => {
                                    hypno_buffers.push(FType::Dizziness);
                                    buffer_count = 2;
                                }
                                _ => {}
                            }
                        } else {
                            hypno_buffers.push(FType::Shyness);
                        }
                    }
                    if you.is(FType::Impatience)
                        && (you.is(FType::Loneliness)
                            || you.hypno_state.get_next_hypno_aff() == Some(FType::Loneliness))
                    {
                        hypno_buffers.push(FType::Recklessness);
                    } else if you.is(FType::Impatience)
                        && (you.is(FType::Vertigo)
                            || you.hypno_state.get_next_hypno_aff() == Some(FType::Vertigo))
                    {
                        hypno_buffers.push(FType::Recklessness);
                    }
                    if you.is(FType::Generosity)
                        || you.hypno_state.get_next_hypno_aff() == Some(FType::Generosity)
                    {
                        hypno_buffers.push(FType::Peace);
                        if !you.is(FType::Impatience) {
                            hypno_buffers.push(FType::Stupidity);
                        }
                    }
                    let hypno_buffers = get_venoms(hypno_buffers, buffer_count, &you);
                    add_buffers(&mut venoms, &hypno_buffers);
                }
            }
            if !priority_buffer
                || (you.is(FType::Hypersomnia)
                    && get_cure_depth(&you, FType::Hypersomnia).cures > 1)
                || (you.is(FType::Hypersomnia)
                    && (!you.is(FType::Instawake) || !you.is(FType::Insomnia)))
            {
                add_delphs(&timeline, &me, &you, &strategy, &mut venoms);
            }
            let v2 = venoms.pop();
            let v1 = venoms.pop();
            if should_bedazzle(&me, &you, &strategy, false) {
                return Box::new(BedazzleAction::new(who_am_i, &target));
            } else if should_slit(&me, &you, &strategy) && v1.is_some() {
                return Box::new(SlitAction::new(
                    who_am_i.to_string(),
                    target.to_string(),
                    v2.or(v1).unwrap().to_string(),
                ));
            } else if let (Some(v1), Some(v2)) = (v1, v2) {
                return Box::new(DoublestabAction::new(
                    who_am_i.to_string(),
                    target.to_string(),
                    v1.to_string(),
                    v2.to_string(),
                ));
            } else if you.is(FType::Fangbarrier) {
                return Box::new(FlayAction::fangbarrier(
                    who_am_i.to_string(),
                    target.to_string(),
                ));
            } else {
                return Box::new(BiteAction::new(who_am_i, &target, &"camus"));
            }
        }
    } else if strategy == "damage" {
        let you = timeline.state.borrow_agent(target);
        if you.is(FType::Fangbarrier) {
            return Box::new(FlayAction::fangbarrier(
                who_am_i.to_string(),
                target.to_string(),
            ));
        } else {
            return Box::new(BiteAction::new(who_am_i, &target, &"camus"));
        }
    } else if strategy == "shield" {
        let me = timeline.state.borrow_me();
        if me.can_touch() && !me.is(FType::Shielded) {
            return Box::new(ShieldAction::new(who_am_i));
        } else if needs_shrugging(timeline, who_am_i) {
            return Box::new(ShruggingAction::shrug_asthma(who_am_i.to_string()));
        } else {
            return Box::new(Action::new(
                "firstaid elevate paresis;;firstaid elevate frozen;;firstaid elevate paralysis"
                    .to_string(),
            ));
        }
    } else {
        return Box::new(Inactivity);
    }
}

pub fn get_hypno_stack_name(timeline: &AetTimeline, target: &String, strategy: &String) -> String {
    timeline
        .state
        .get_my_hint(&"HYPNO_STACK".to_string())
        .unwrap_or(strategy.to_string())
}

pub fn get_hypno_stack<'s>(
    timeline: &AetTimeline,
    target: &String,
    strategy: &String,
    db: Option<&DatabaseModule>,
) -> Vec<Hypnosis> {
    db.and_then(|db| {
        let stack = get_hypno_stack_name(timeline, target, strategy);
        if stack == "normal" {
            None // Default to HARD_HYPNO
        } else if stack == "class" {
            if let Some(class) = db.get_class(target) {
                db.get_hypno_plan(&class.to_string())
            } else {
                db.get_hypno_plan(&format!("hypno_{}", stack))
            }
        } else {
            db.get_hypno_plan(&format!("hypno_{}", stack))
        }
    })
    .unwrap_or(HARD_HYPNO.to_vec())
}

pub fn get_equil_attack<'s>(
    timeline: &AetTimeline,
    me: &String,
    target: &String,
    strategy: &String,
    db: Option<&DatabaseModule>,
) -> Box<dyn ActiveTransition> {
    if strategy.eq("damage") || strategy.eq("shield") || strategy.eq("runaway") {
        return Box::new(Inactivity);
    }
    let you = timeline.state.borrow_agent(target);
    let stack = get_hypno_stack(timeline, target, strategy, db);
    let hypno_action = get_top_hypno(me, target, &you, &stack);
    hypno_action.unwrap_or(Box::new(Inactivity))
}

pub fn get_shadow_attack<'s>(
    timeline: &AetTimeline,
    me: &String,
    target: &String,
    strategy: &String,
) -> Box<dyn ActiveTransition> {
    if strategy == "pre" || strategy == "shield" || strategy == "runaway" {
        Box::new(Inactivity)
    } else {
        let you = timeline.state.borrow_agent(target);
        if !should_void(timeline)
            || you.is(FType::Void)
            || you.is(FType::Weakvoid)
            || you.hypno_state.active
        {
            if you.lock_duration().is_some() {
                Box::new(SleightAction::new(me, &target, &"blank"))
            } else if strategy == "salve" {
                Box::new(SleightAction::new(me, &target, &"abrasion"))
            } else {
                Box::new(SleightAction::new(me, &target, &"dissipate"))
            }
        } else {
            Box::new(SleightAction::new(me, &target, &"void"))
        }
    }
}

pub fn get_snap(timeline: &AetTimeline, me: &String, target: &String, _strategy: &String) -> bool {
    let you = timeline.state.borrow_agent(target);
    if get_top_hypno(me, target, &you, &HARD_HYPNO.to_vec()).is_none()
        && you.hypno_state.sealed.is_some()
    {
        return true;
    } else {
        return false;
    }
}

pub fn get_action_plan(
    timeline: &AetTimeline,
    me: &String,
    target: &String,
    strategy: &String,
    db: Option<&DatabaseModule>,
) -> ActionPlan {
    let mut action_plan = ActionPlan::new(me);
    let mut balance = get_balance_attack(timeline, me, target, strategy, db);
    if should_regenerate(&timeline, me) {
        balance = Box::new(RegenerateAction::new(me.to_string()));
    }
    if let Some(parry) = get_needed_parry(timeline, me, target, strategy, db) {
        balance = Box::new(SeparatorAction::pair(
            Box::new(ParryAction::new(me.to_string(), parry)),
            balance,
        ));
    }
    let equil = get_equil_attack(timeline, me, target, strategy, db);
    let shadow = get_shadow_attack(timeline, me, target, strategy);
    if let Ok(_activation) = balance.act(&timeline) {
        action_plan.add_to_qeb(balance);
    }
    if let Ok(_activation) = equil.act(&timeline) {
        action_plan.add_to_qeb(equil);
    }
    if let Ok(activation) = shadow.act(&timeline) {
        if activation.starts_with("shadow sleight void") {
            action_plan.queue_for(BType::Secondary, shadow);
        } else {
            action_plan.add_to_qeb(shadow);
        }
    }
    action_plan
}

struct SyssinActionPlanner;
const STRATEGIES: [&'static str; 3] = ["phys", "bedazzle", "aggro"];

impl ActionPlanner for SyssinActionPlanner {
    fn get_strategies(&self) -> &'static [&'static str] {
        &STRATEGIES
    }
    fn get_plan(
        &self,
        timeline: &AetTimeline,
        actor: &String,
        target: &String,
        strategy: &str,
        db: Option<&DatabaseModule>,
    ) -> ActionPlan {
        get_action_plan(timeline, actor, target, &strategy.to_string(), db)
    }
}

pub fn get_attack(
    timeline: &AetTimeline,
    target: &String,
    strategy: &String,
    db: Option<&DatabaseModule>,
) -> String {
    let action_plan = get_action_plan(&timeline, &timeline.who_am_i(), &target, &strategy, db);
    action_plan.get_inputs(&timeline)
}
