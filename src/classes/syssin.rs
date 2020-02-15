use crate::actions::*;
use crate::alpha_beta::*;
use crate::classes::{add_buffers, get_venom, get_venoms, AFFLICT_VENOMS};
use crate::curatives::*;
use crate::io::*;
use crate::observables::*;
use crate::timeline::*;
use crate::types::*;
use regex::Regex;
use std::collections::HashMap;

#[cfg(test)]
mod timeline_tests {
    use super::*;

    #[test]
    fn test_dstab_3p() {
        let mut timeline = Timeline::new();
        timeline
            .state
            .add_player_hint(&"Savas", &"CALLED_VENOMS", "kalmia slike".to_string());
        let dstab_slice = TimeSlice {
            observations: vec![Observation::CombatAction(CombatAction {
                caster: "Savas".to_string(),
                category: "Assassination".to_string(),
                skill: "Doublestab".to_string(),
                target: "Benedicto".to_string(),
                annotation: "".to_string(),
            })],
            prompt: Prompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(dstab_slice);
        let savas_state = timeline.state.get_agent(&"Savas".to_string());
        assert_eq!(savas_state.balanced(BType::Balance), false);
        assert_eq!(savas_state.get_flag(FType::Asthma), false);
        assert_eq!(savas_state.get_flag(FType::Anorexia), false);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Balance), true);
        assert_eq!(bene_state.get_flag(FType::Asthma), true);
        assert_eq!(bene_state.get_flag(FType::Anorexia), true);
    }

    #[test]
    fn test_dstab_3p_dodge() {
        let mut timeline = Timeline::new();
        timeline
            .state
            .add_player_hint(&"Savas", &"CALLED_VENOMS", "kalmia slike".to_string());
        let dstab_slice = TimeSlice {
            observations: vec![
                Observation::CombatAction(CombatAction {
                    caster: "Savas".to_string(),
                    category: "Assassination".to_string(),
                    skill: "Doublestab".to_string(),
                    target: "Benedicto".to_string(),
                    annotation: "".to_string(),
                }),
                Observation::Dodges,
            ],
            prompt: Prompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(dstab_slice);
        let savas_state = timeline.state.get_agent(&"Savas".to_string());
        assert_eq!(savas_state.balanced(BType::Balance), false);
        assert_eq!(savas_state.get_flag(FType::Asthma), false);
        assert_eq!(savas_state.get_flag(FType::Anorexia), false);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Balance), true);
        assert_eq!(bene_state.get_flag(FType::Asthma), true);
        assert_eq!(bene_state.get_flag(FType::Anorexia), false);
    }

    #[test]
    fn test_dstab() {
        let mut timeline = Timeline::new();
        let dstab_slice = TimeSlice {
            observations: vec![
                Observation::CombatAction(CombatAction {
                    caster: "Seurimas".to_string(),
                    category: "Assassination".to_string(),
                    skill: "Doublestab".to_string(),
                    target: "Benedicto".to_string(),
                    annotation: "".to_string(),
                }),
                Observation::Devenoms("slike".into()),
                Observation::Devenoms("kalmia".into()),
            ],
            prompt: Prompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(dstab_slice);
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.get_flag(FType::Asthma), false);
        assert_eq!(seur_state.get_flag(FType::Anorexia), false);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Balance), true);
        assert_eq!(bene_state.get_flag(FType::Asthma), true);
        assert_eq!(bene_state.get_flag(FType::Anorexia), true);
    }

    #[test]
    fn test_void_1p() {
        let mut timeline = Timeline::new();
        timeline
            .state
            .set_flag_for_agent(&"Seurimas".to_string(), &"void".to_string(), true);
        timeline
            .state
            .set_flag_for_agent(&"Seurimas".to_string(), &"stupidity".to_string(), true);
        let dstab_slice = TimeSlice {
            observations: vec![
                Observation::SimpleCureAction(SimpleCureAction {
                    cure_type: SimpleCure::Pill("euphoriant".to_string()),
                    caster: "Seurimas".to_string(),
                }),
                Observation::Cured("void".to_string()),
                Observation::Afflicted("weakvoid".to_string()),
            ],
            prompt: Prompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        let bene_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(bene_state.balanced(BType::Pill), true);
        assert_eq!(bene_state.get_flag(FType::Stupidity), true);
        assert_eq!(bene_state.get_flag(FType::Void), true);
        assert_eq!(bene_state.get_flag(FType::Weakvoid), false);
        timeline.push_time_slice(dstab_slice);
        let bene_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(bene_state.balanced(BType::Pill), false);
        assert_eq!(bene_state.get_flag(FType::Stupidity), true);
        assert_eq!(bene_state.get_flag(FType::Void), false);
        assert_eq!(bene_state.get_flag(FType::Weakvoid), true);
    }

    #[test]
    fn test_void() {
        let mut timeline = Timeline::new();
        timeline
            .state
            .set_flag_for_agent(&"Benedicto".to_string(), &"void".to_string(), true);
        timeline
            .state
            .set_flag_for_agent(&"Benedicto".to_string(), &"stupidity".to_string(), true);
        let dstab_slice = TimeSlice {
            observations: vec![
                Observation::SimpleCureAction(SimpleCureAction {
                    cure_type: SimpleCure::Pill("euphoriant".to_string()),
                    caster: "Benedicto".to_string(),
                }),
                Observation::DiscernedCure("Benedicto".to_string(), "void".to_string()),
            ],
            prompt: Prompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Pill), true);
        assert_eq!(bene_state.get_flag(FType::Stupidity), true);
        assert_eq!(bene_state.get_flag(FType::Void), true);
        assert_eq!(bene_state.get_flag(FType::Weakvoid), false);
        timeline.push_time_slice(dstab_slice);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Pill), false);
        assert_eq!(bene_state.get_flag(FType::Stupidity), true);
        assert_eq!(bene_state.get_flag(FType::Void), false);
        assert_eq!(bene_state.get_flag(FType::Weakvoid), true);
    }

    #[test]
    fn test_weakvoid() {
        let mut timeline = Timeline::new();
        timeline
            .state
            .set_flag_for_agent(&"Benedicto".to_string(), &"weakvoid".to_string(), true);
        timeline
            .state
            .set_flag_for_agent(&"Benedicto".to_string(), &"stupidity".to_string(), true);
        let dstab_slice = TimeSlice {
            observations: vec![
                Observation::SimpleCureAction(SimpleCureAction {
                    cure_type: SimpleCure::Pill("euphoriant".to_string()),
                    caster: "Benedicto".to_string(),
                }),
                Observation::DiscernedCure("Benedicto".to_string(), "weakvoid".to_string()),
            ],
            prompt: Prompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Pill), true);
        assert_eq!(bene_state.get_flag(FType::Stupidity), true);
        assert_eq!(bene_state.get_flag(FType::Void), false);
        assert_eq!(bene_state.get_flag(FType::Weakvoid), true);
        timeline.push_time_slice(dstab_slice);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Pill), false);
        assert_eq!(bene_state.get_flag(FType::Stupidity), true);
        assert_eq!(bene_state.get_flag(FType::Void), false);
        assert_eq!(bene_state.get_flag(FType::Weakvoid), false);
    }

    #[test]
    fn test_dstab_purge() {
        let mut timeline = Timeline::new();
        let dstab_slice = TimeSlice {
            observations: vec![
                Observation::CombatAction(CombatAction {
                    caster: "Seurimas".to_string(),
                    category: "Assassination".to_string(),
                    skill: "Doublestab".to_string(),
                    target: "Benedicto".to_string(),
                    annotation: "".to_string(),
                }),
                Observation::Devenoms("slike".into()),
                Observation::Devenoms("kalmia".into()),
                Observation::PurgeVenom("Benedicto".into(), "kalmia".into()),
            ],
            prompt: Prompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(dstab_slice);
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.get_flag(FType::Asthma), false);
        assert_eq!(seur_state.get_flag(FType::Anorexia), false);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Balance), true);
        assert_eq!(bene_state.get_flag(FType::Asthma), false);
        assert_eq!(bene_state.get_flag(FType::Anorexia), true);
    }

    #[test]
    fn test_dstab_relapse() {
        let mut timeline = Timeline::new();
        let bite_slice = TimeSlice {
            observations: vec![Observation::CombatAction(CombatAction {
                caster: "Seurimas".to_string(),
                category: "Assassination".to_string(),
                skill: "Bite".to_string(),
                target: "Benedicto".to_string(),
                annotation: "scytherus".to_string(),
            })],
            prompt: Prompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        let dstab_slice = TimeSlice {
            observations: vec![
                Observation::CombatAction(CombatAction {
                    caster: "Seurimas".to_string(),
                    category: "Assassination".to_string(),
                    skill: "Doublestab".to_string(),
                    target: "Benedicto".to_string(),
                    annotation: "".to_string(),
                }),
                Observation::Devenoms("slike".into()),
                Observation::Devenoms("kalmia".into()),
            ],
            prompt: Prompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(bite_slice);
        timeline.push_time_slice(dstab_slice);
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.get_flag(FType::Asthma), false);
        assert_eq!(seur_state.get_flag(FType::Anorexia), false);
        let mut bene_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Balance), true);
        assert_eq!(bene_state.get_flag(FType::Asthma), true);
        assert_eq!(bene_state.get_flag(FType::Anorexia), true);
        assert_eq!(bene_state.relapse(), Some("slike".to_string()));
        assert_eq!(bene_state.relapse(), Some("kalmia".to_string()));
    }

    #[test]
    fn test_dstab_rebounds() {
        let mut timeline = Timeline::new();
        let dstab_slice = TimeSlice {
            observations: vec![
                Observation::CombatAction(CombatAction {
                    caster: "Seurimas".to_string(),
                    category: "Assassination".to_string(),
                    skill: "Doublestab".to_string(),
                    target: "Benedicto".to_string(),
                    annotation: "".to_string(),
                }),
                Observation::Devenoms("slike".into()),
                Observation::Rebounds,
                Observation::Devenoms("kalmia".into()),
                Observation::Rebounds,
            ],
            prompt: Prompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(dstab_slice);
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.get_flag(FType::Asthma), true);
        assert_eq!(seur_state.get_flag(FType::Anorexia), true);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Balance), true);
        assert_eq!(bene_state.get_flag(FType::Asthma), false);
        assert_eq!(bene_state.get_flag(FType::Anorexia), false);
    }

    #[test]
    fn test_bite() {
        let mut timeline = Timeline::new();
        let dstab_slice = TimeSlice {
            observations: vec![Observation::CombatAction(CombatAction {
                caster: "Seurimas".to_string(),
                category: "Assassination".to_string(),
                skill: "Bite".to_string(),
                target: "Benedicto".to_string(),
                annotation: "scytherus".to_string(),
            })],
            prompt: Prompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(dstab_slice);
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.get_flag(FType::ThinBlood), false);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Balance), true);
        assert_eq!(bene_state.get_flag(FType::ThinBlood), true);
    }

    #[test]
    fn test_suggest() {
        let mut timeline = Timeline::new();
        let suggest_slice = TimeSlice {
            observations: vec![
                Observation::Sent("suggest Benedicto stupidity".to_string()),
                Observation::CombatAction(CombatAction {
                    caster: "Seurimas".to_string(),
                    category: "Hypnosis".to_string(),
                    skill: "Suggest".to_string(),
                    target: "Benedicto".to_string(),
                    annotation: "".to_string(),
                }),
            ],
            prompt: Prompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(suggest_slice);
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Equil), false);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(
            bene_state.hypnosis_stack.get(0),
            Some(&Hypnosis::Aff(FType::Stupidity))
        );
    }

    #[test]
    fn test_suggest_qeb() {
        let mut timeline = Timeline::new();
        let suggest_slice = TimeSlice {
            observations: vec![
                Observation::Sent(
                    "qeb dstab Benedicto aconite kalmia;;suggest Benedicto stupidity".to_string(),
                ),
                Observation::CombatAction(CombatAction {
                    caster: "Seurimas".to_string(),
                    category: "Hypnosis".to_string(),
                    skill: "Suggest".to_string(),
                    target: "Benedicto".to_string(),
                    annotation: "".to_string(),
                }),
            ],
            prompt: Prompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(suggest_slice);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(
            bene_state.hypnosis_stack.get(0),
            Some(&Hypnosis::Aff(FType::Stupidity))
        );
    }
}

lazy_static! {
    static ref SUGGESTION: Regex = Regex::new(r"suggest (\w+) (.*)").unwrap();
}

lazy_static! {
    static ref ACTION: Regex = Regex::new(r"action (.*)").unwrap();
}

pub fn infer_suggestion(name: &String, agent_states: &mut TimelineState) -> Hypnosis {
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

pub fn handle_sent(command: &String, agent_states: &mut TimelineState) {
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
}

pub struct DoublestabAction {
    pub caster: String,
    pub target: String,
    pub rebounded: bool,
    pub dodges: usize,
    pub venoms: Vec<String>,
}

impl ActiveTransition for DoublestabAction {
    fn read(
        now: &TimelineState,
        observation: &Observation,
        before: &Vec<Observation>,
        after: &Vec<Observation>,
        prompt: &Prompt,
    ) -> Self {
        if let Observation::CombatAction(combat_action) = observation {
            let caster = combat_action.caster.clone();
            let target = combat_action.target.clone();
            let rebounded = false;
            let dodges = 0;
            let venoms = Vec::new();
            DoublestabAction {
                caster,
                target,
                rebounded,
                dodges,
                venoms,
            }
        } else {
            panic!("Could not read DoubleStab for {:?}", observation)
        }
    }

    fn simulate(&self, now: TimelineState) -> VariableState {
        Vec::new()
    }

    fn act(&self, now: TimelineState) -> ActivateResult {
        Ok("".to_string())
    }
}

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut TimelineState,
    before: &Vec<Observation>,
    after: &Vec<Observation>,
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
            apply_or_infer_balance(&mut me, (BType::Balance, 2.8), after);
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
            apply_venom(&mut you, &combat_action.annotation)?;
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
                    you.set_flag(FType::HardenedSkin, false);
                }
                "failure-fangbarrier" => {
                    you.set_flag(FType::HardenedSkin, false);
                }
                "shield" => {
                    you.set_flag(FType::Shield, false);
                }
                "failure-shield" => {
                    you.set_flag(FType::Shield, false);
                }
                _ => {}
            }
            agent_states.set_agent(&combat_action.caster, me);
            agent_states.set_agent(&combat_action.target, you);
        }
        "Hypnotise" => {
            let mut you = agent_states.get_agent(&combat_action.target);
            you.set_flag(FType::Hypnotized, true);
            agent_states.set_agent(&combat_action.target, you);
        }
        "Seal" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            let mut you = agent_states.get_agent(&combat_action.target);
            apply_or_infer_balance(&mut me, (BType::Equil, 3.0), after);
            you.set_flag(FType::Hypnotized, false);
            agent_states.set_agent(&combat_action.caster, me);
            agent_states.set_agent(&combat_action.target, you);
        }
        "Suggest" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            let mut you = agent_states.get_agent(&combat_action.target);
            apply_or_infer_balance(&mut me, (BType::Equil, 2.25), after);
            push_suggestion(
                &mut you,
                infer_suggestion(&combat_action.target, agent_states),
            );
            agent_states.set_agent(&combat_action.caster, me);
            agent_states.set_agent(&combat_action.target, you);
        }
        "Snap" => {
            if let Some(target) =
                agent_states.get_player_hint(&combat_action.caster, &"snap".into())
            {
                let mut you = agent_states.get_agent(&target);
                start_hypnosis(&mut you);
                agent_states.set_agent(&target, you);
            }
        }
        "Bedazzle" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            let mut you = agent_states.get_agent(&combat_action.target);
            apply_or_infer_balance(&mut me, (BType::Balance, 2.25), after);
            apply_or_infer_random_afflictions(&mut you, after);
            agent_states.set_agent(&combat_action.caster, me);
            agent_states.set_agent(&combat_action.target, you);
        }
        "Fire" => {
            let mut you = agent_states.get_agent(&combat_action.target);
            apply_or_infer_random_afflictions(&mut you, after);
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
    static ref FIRE_STACK: Vec<FType> = vec![
        FType::Paresis,
        FType::Shyness,
        FType::Clumsiness,
        FType::Asthma,
        FType::Stupidity,
        FType::Allergies,
        FType::Vomiting,
    ];
}

lazy_static! {
    static ref PHYS_STACK: Vec<FType> = vec![
        FType::Paresis,
        FType::Clumsiness,
        FType::Asthma,
        FType::Stupidity,
        FType::Allergies,
        FType::Dizziness,
        FType::Vomiting,
        FType::LeftLegBroken,
        FType::RightLegBroken,
    ];
}

lazy_static! {
    static ref WAYF_STACK: Vec<FType> = vec![
        FType::Stupidity,
        FType::Clumsiness,
        FType::Weariness,
        FType::Paresis,
        FType::Asthma,
        FType::Allergies,
        FType::Dizziness,
        FType::Vomiting,
        FType::LeftLegBroken,
        FType::RightLegBroken,
    ];
}

lazy_static! {
    static ref AGGRO_STACK: Vec<FType> = vec![
        FType::Stupidity,
        FType::Asthma,
        FType::Clumsiness,
        FType::Paresis,
        FType::Allergies,
        FType::Dizziness,
        FType::Vomiting,
        FType::LeftLegBroken,
        FType::RightLegBroken,
    ];
}

lazy_static! {
    static ref SALVE_STACK: Vec<FType> = vec![
        FType::Anorexia,
        FType::Stuttering,
        FType::LeftLegBroken,
        FType::RightLegBroken,
        FType::LeftArmBroken,
        FType::RightArmBroken,
        FType::Clumsiness,
    ];
}

lazy_static! {
    static ref SLIT_STACK: Vec<FType> = vec![
        FType::Haemophilia,
        FType::LeftLegBroken,
        FType::RightLegBroken,
        FType::LeftArmBroken,
        FType::RightArmBroken,
        FType::Allergies,
        FType::Vomiting,
    ];
}

lazy_static! {
    static ref SOFT_STACK: Vec<FType> = vec![FType::Asthma, FType::Anorexia, FType::Slickness,];
}

lazy_static! {
    static ref SOFT_BUFFER: Vec<FType> = vec![FType::Clumsiness, FType::Stupidity];
}

lazy_static! {
    static ref THIN_BUFFER_STACK: Vec<FType> = vec![FType::Allergies, FType::Vomiting];
}

lazy_static! {
    static ref LOCK_BUFFER_STACK: Vec<FType> =
        vec![FType::Clumsiness, FType::Stupidity, FType::Paresis];
}

lazy_static! {
    static ref STACKING_STRATEGIES: HashMap<String, Vec<FType>> = {
        let mut val = HashMap::new();
        val.insert("coag".into(), COAG_STACK.to_vec());
        val.insert("dec".into(), DEC_STACK.to_vec());
        val.insert("phys".into(), PHYS_STACK.to_vec());
        val.insert("fire".into(), FIRE_STACK.to_vec());
        val.insert("aggro".into(), AGGRO_STACK.to_vec());
        val.insert("salve".into(), SALVE_STACK.to_vec());
        val.insert("wayf".into(), WAYF_STACK.to_vec());
        val
    };
}

lazy_static! {
    static ref HARD_HYPNO: Vec<Hypnosis> = vec![
        Hypnosis::Aff(FType::Lethargy),
        Hypnosis::Aff(FType::Lethargy),
        Hypnosis::Aff(FType::Impatience),
        Hypnosis::Aff(FType::Impatience),
        Hypnosis::Aff(FType::Loneliness),
        Hypnosis::Aff(FType::Loneliness),
        Hypnosis::Aff(FType::Impatience),
        Hypnosis::Aff(FType::Loneliness),
    ];
}

pub fn get_hypno_str(target: &String, hypno: &Hypnosis) -> String {
    match hypno {
        Hypnosis::Aff(affliction) => format!("suggest {} {:?}", target, affliction),
        Hypnosis::Action(act) => format!("suggest {} action {}", target, act),
    }
}

pub fn start_hypnosis(who: &mut AgentState) {
    who.set_flag(FType::Snapped, true);
}

pub fn get_top_hypno(name: &String, target: &AgentState, hypnos: &Vec<Hypnosis>) -> Option<String> {
    let mut hypno_idx = 0;
    let mut hypno = None;
    for i in 0..target.hypnosis_stack.len() {
        if target.hypnosis_stack.get(i) == hypnos.get(hypno_idx) {
            hypno_idx += 1;
        }
    }
    if hypno_idx < hypnos.len() {
        if let Some(next_hypno) = hypnos.get(hypno_idx) {
            hypno = Some(get_hypno_str(name, next_hypno));
        }
    }
    if let Some(suggestion) = hypno {
        if !target.get_flag(FType::Hypnotized) {
            Some(format!("hypnotise {};;{}", name, suggestion))
        } else {
            Some(suggestion)
        }
    } else if target.get_flag(FType::Hypnotized) {
        Some(format!("seal {} 3", name))
    } else {
        None
    }
}

fn check_config(topper: &Topper, value: &String) -> bool {
    topper
        .timeline
        .state
        .get_my_hint(value)
        .unwrap_or("false".to_string())
        .eq(&"true")
}

fn use_one_rag(topper: &Topper) -> bool {
    check_config(topper, &"ONE_RAG".to_string())
}

fn should_call_venoms(topper: &Topper) -> bool {
    check_config(topper, &"VENOM_CALLING".to_string())
}

fn should_void(topper: &Topper) -> bool {
    !check_config(topper, &"NO_VOID".to_string())
}

fn needs_shrugging(topper: &Topper) -> bool {
    let me = topper
        .timeline
        .state
        .borrow_agent(&topper.timeline.who_am_i());
    me.balanced(BType::ClassCure1)
        && me.is(FType::Asthma)
        && me.is(FType::Anorexia)
        && me.is(FType::Slickness)
        && (!me.balanced(BType::Tree) || me.is(FType::Paresis) || me.is(FType::Paralysis))
        && (!me.balanced(BType::Focus) || me.is(FType::Impatience) || me.is(FType::Stupidity))
}

fn go_for_thin_blood(topper: &Topper, you: &AgentState, strategy: &String) -> bool {
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
    (buffer_count >= 2 || (buffer_count >= 1 && !you.is(FType::HardenedSkin)))
        && !you.is(FType::ThinBlood)
}

fn should_lock(you: &AgentState, lockers: &Vec<&str>) -> bool {
    (!you.can_focus(true) || you.is(FType::Stupidity) || you.get_balance(BType::Focus) > 2.5)
        && (!you.can_tree(true) || you.get_balance(BType::Tree) > 2.5)
        && lockers.len() < 3
        && lockers.len() > 0
}

pub fn call_venom(target: &String, v1: &String) -> String {
    format!("wt Afflicting {}: {}", target, v1)
}

pub fn call_venoms(target: &String, v1: &String, v2: &String) -> String {
    format!("wt Afflicting {}: {}, {}", target, v1, v2)
}

pub fn get_flay_action(topper: &Topper, target: &String, def: String, v1: String) -> String {
    let action = if use_one_rag(topper) && !v1.eq_ignore_ascii_case("") {
        format!("hw {};;flay {} {}", v1, target, def)
    } else {
        format!("flay {} {} {}", target, def, v1)
    };
    if should_call_venoms(topper) {
        format!("{};;{}", call_venom(target, &v1), action)
    } else {
        action
    }
}

pub fn get_dstab_action(topper: &Topper, target: &String, v1: &String, v2: &String) -> String {
    let action = if use_one_rag(topper) {
        format!("hr {};;hr {};;dstab {}", v2, v1, target)
    } else {
        format!("dstab {} {} {}", target, v1, v2)
    };
    if should_call_venoms(topper) {
        format!("{};;{}", call_venoms(target, v1, v2), action)
    } else {
        action
    }
}

pub fn get_slit_action(topper: &Topper, target: &String, v1: String) -> String {
    if use_one_rag(topper) {
        format!("hr {};;slit {}", v1, target)
    } else {
        format!("slit {} {}", target, v1)
    }
}

pub fn get_balance_attack(topper: &Topper, target: &String, strategy: &String) -> String {
    if let Some(stack) = STACKING_STRATEGIES.get(strategy) {
        let you = topper.timeline.state.borrow_agent(target);
        if needs_shrugging(&topper) {
            return "shrug asthma".to_string();
        } else if get_equil_attack(topper, target, strategy).starts_with("seal") {
            "".into()
        } else if you.is(FType::Shield) || you.is(FType::Rebounding) {
            let defense = if you.is(FType::Shield) {
                "shield"
            } else {
                "rebounding"
            };
            if let Some(venom) = get_venoms(stack.to_vec(), 1, &you).pop() {
                return get_flay_action(topper, target, defense.to_string(), venom.to_string());
            } else {
                return format!("flay {} {}", target, defense);
            }
        } else {
            println!("{}", you.flags);
            let mut venoms = get_venoms(stack.to_vec(), 2, &you);
            let lockers = get_venoms(SOFT_STACK.to_vec(), 3, &you);
            let mut priority_buffer = false;
            if should_lock(&you, &lockers) {
                println!("Locking!");
                add_buffers(&mut venoms, &lockers);
                priority_buffer = true;
            } else if lockers.len() == 0 {
                let buffer = get_venoms(LOCK_BUFFER_STACK.to_vec(), 2, &you);
                println!("Lock Buffering! {:?} {:?}", venoms, buffer);
                add_buffers(&mut venoms, &buffer);
                priority_buffer = buffer.len() > 0;
            }
            if !priority_buffer {
                if go_for_thin_blood(topper, &you, strategy) {
                    println!("Thinning!");
                    if you.is(FType::HardenedSkin) {
                        return format!("flay {} fangbarrier", target);
                    } else {
                        return format!("bite {} scytherus", target);
                    }
                }
                let mut buffer = get_venoms(THIN_BUFFER_STACK.to_vec(), 2, &you);
                if you.lock_duration().map_or(false, |dur| dur > 10.0) && !you.is(FType::Voyria) {
                    buffer.insert(buffer.len(), "voyria");
                }
                if you.is(FType::ThinBlood) && buffer.len() > 0 {
                    println!("Buffering! {:?} {:?}", venoms, buffer);
                    add_buffers(&mut venoms, &buffer);
                }
            }
            let v1 = venoms.pop();
            let v2 = venoms.pop();
            if you.is(FType::Hypersomnia) && !you.is(FType::Asleep) {
                return get_dstab_action(
                    topper,
                    target,
                    &"delphinium".to_string(),
                    &"delphinium".to_string(),
                );
            } else if let (Some(v1), Some(v2)) = (v1, v2) {
                return get_dstab_action(topper, target, &v1.to_string(), &v2.to_string());
            } else if you.is(FType::HardenedSkin) {
                return format!("flay {} fangbarrier", target);
            } else {
                return format!("bite {} camus", target);
            }
        }
    } else if strategy == "damage" {
        let you = topper.timeline.state.borrow_agent(target);
        if you.is(FType::HardenedSkin) {
            return format!("flay {} fangbarrier", target);
        } else {
            return format!("bite {} camus", target);
        }
    } else {
        "".into()
    }
}

pub fn get_equil_attack(topper: &Topper, target: &String, strategy: &String) -> String {
    let you = topper.timeline.state.borrow_agent(target);
    let hypno_action = get_top_hypno(target, &you, &HARD_HYPNO.to_vec());
    hypno_action.unwrap_or("".into())
}

pub fn get_shadow_attack(topper: &Topper, target: &String, strategy: &String) -> String {
    if strategy == "pre" {
        "".into()
    } else {
        if !strategy.eq("salve") {
            let you = topper.timeline.state.borrow_agent(target);
            if !should_void(topper)
                || you.get_flag(FType::Void)
                || you.get_flag(FType::Weakvoid)
                || you.get_flag(FType::Snapped)
            {
                format!(";;shadow sleight dissipate {}", target)
            } else {
                format!("%%qs shadow sleight void {}", target)
            }
        } else {
            format!(";;shadow sleight abrasion {}", target)
        }
    }
}

pub fn get_snap(topper: &Topper, target: &String, strategy: &String) -> bool {
    let you = topper.timeline.state.borrow_agent(target);
    if get_top_hypno(target, &you, &HARD_HYPNO.to_vec()) == None
        && !you.get_flag(FType::Snapped)
        && !you.get_flag(FType::Hypnotized)
        && !you.balanced(BType::Tree)
    {
        return true;
    } else {
        return false;
    }
}

pub fn get_attack(topper: &Topper, target: &String, strategy: &String) -> String {
    let balance = get_balance_attack(topper, target, strategy);
    let equil = get_equil_attack(topper, target, strategy);
    let shadow = get_shadow_attack(topper, target, strategy);
    let should_snap = get_snap(topper, target, strategy);
    let mut attack: String = if should_snap {
        format!("snap {}", target)
    } else {
        "".to_string()
    };
    if balance != "" {
        attack = format!("qeb {}", balance);
    }
    if equil != "" {
        attack = format!("{};;{}", attack, equil);
    }
    if shadow != "" {
        attack = format!("{}{}", attack, shadow);
    }
    attack
}

pub fn get_offensive_actions() -> Vec<StateAction> {
    let mut actions = vec![];
    // Aggro Stack
    actions.push(dstab_stack(vec![
        FType::Paresis,
        FType::Asthma,
        FType::ThinBlood,
        FType::Stupidity,
        FType::Vomiting,
        FType::Allergies,
        FType::Anorexia,
        FType::Slickness,
    ]));
    // Coag Stack
    actions.push(dstab_stack(COAG_STACK.to_vec()));
    // Salve Stack
    actions.push(dstab_stack(vec![
        FType::LeftLegBroken,
        FType::RightLegBroken,
        FType::LeftArmBroken,
        FType::RightArmBroken,
        FType::Anorexia,
        FType::Slickness,
        FType::Asthma,
    ]));
    actions
}

#[cfg(test)]
mod simulation_tests {
    use super::*;

    #[test]
    fn test_dstab_stack() {
        let salve_stack = dstab_stack(vec![
            FType::LeftLegBroken,
            FType::RightLegBroken,
            FType::LeftArmBroken,
            FType::RightArmBroken,
            FType::Anorexia,
        ]);
        let mut simulation = SimulationState::new(&vec![BASE_STATE.clone(), BASE_STATE.clone()]);
        simulation.apply_action(&salve_stack, 0, 1);
        assert_eq!(simulation.states[1].is(FType::LeftLegBroken), true);
        assert_eq!(simulation.states[1].is(FType::RightLegBroken), true);
        assert_eq!(simulation.states[1].is(FType::LeftArmBroken), false);
        assert_eq!(simulation.states[1].is(FType::RightArmBroken), false);
        assert_eq!(simulation.states[1].is(FType::Anorexia), false);
        simulation.apply_action(&salve_stack, 0, 1);
        assert_eq!(simulation.states[1].is(FType::LeftLegBroken), true);
        assert_eq!(simulation.states[1].is(FType::RightLegBroken), true);
        assert_eq!(simulation.states[1].is(FType::LeftArmBroken), true);
        assert_eq!(simulation.states[1].is(FType::RightArmBroken), true);
        assert_eq!(simulation.states[1].is(FType::Anorexia), false);
        simulation.apply_action(&salve_stack, 0, 1);
        assert_eq!(simulation.states[1].is(FType::LeftLegBroken), true);
        assert_eq!(simulation.states[1].is(FType::RightLegBroken), true);
        assert_eq!(simulation.states[1].is(FType::LeftArmBroken), true);
        assert_eq!(simulation.states[1].is(FType::RightArmBroken), true);
        assert_eq!(simulation.states[1].is(FType::Anorexia), true);
    }

    #[test]
    fn test_flay_stack() {
        let salve_stack = flay_stack(vec![
            FType::LeftLegBroken,
            FType::RightLegBroken,
            FType::LeftArmBroken,
            FType::RightArmBroken,
            FType::Anorexia,
        ]);
        let mut simulation = SimulationState::new(&vec![BASE_STATE.clone(), BASE_STATE.clone()]);
        simulation.states[1].set_flag(FType::Shield, true);
        simulation.states[1].set_flag(FType::Rebounding, true);
        simulation.apply_action(&salve_stack, 0, 1);
        assert_eq!(simulation.states[1].is(FType::LeftLegBroken), true);
        assert_eq!(simulation.states[1].is(FType::RightLegBroken), false);
        assert_eq!(simulation.states[1].is(FType::LeftArmBroken), false);
        assert_eq!(simulation.states[1].is(FType::RightArmBroken), false);
        assert_eq!(simulation.states[1].is(FType::Anorexia), false);
        assert_eq!(simulation.states[1].is(FType::Shield), false);
        assert_eq!(simulation.states[1].is(FType::Rebounding), true);
        simulation.apply_action(&salve_stack, 0, 1);
        assert_eq!(simulation.states[1].is(FType::LeftLegBroken), true);
        assert_eq!(simulation.states[1].is(FType::RightLegBroken), true);
        assert_eq!(simulation.states[1].is(FType::LeftArmBroken), false);
        assert_eq!(simulation.states[1].is(FType::RightArmBroken), false);
        assert_eq!(simulation.states[1].is(FType::Anorexia), false);
        assert_eq!(simulation.states[1].is(FType::Shield), false);
        assert_eq!(simulation.states[1].is(FType::Rebounding), false);
    }
}

pub fn dstab_stack(afflictions: Vec<FType>) -> StateAction {
    StateAction {
        name: format!("dstab {:?}", afflictions),
        changes: vec![
            balance_change(BType::Balance, 2.8),
            afflict_in_order(afflictions.clone()),
            afflict_in_order(afflictions.clone()),
        ],
        initial: vec![
            alive(),
            target(alive()),
            target(lacks(FType::Rebounding)),
            target(lacks(FType::Shield)),
            has(BType::Balance),
            has(BType::Equil),
            target(lacks_some(afflictions)),
        ],
    }
}

pub fn flay_stack(afflictions: Vec<FType>) -> StateAction {
    let flayable = vec![FType::Shield, FType::Rebounding];
    StateAction {
        name: "flay".into(),
        changes: vec![
            balance_change(BType::Balance, 2.5),
            flag_me(FType::Shield, false),
            strip_in_order(flayable.clone()),
            afflict_in_order(afflictions.clone()),
        ],
        initial: vec![
            alive(),
            target(alive()),
            has(BType::Balance),
            has(BType::Equil),
            target(some(flayable)),
        ],
    }
}

pub fn dstab_action(
    (venom1, affliction1): (String, FType),
    (venom2, affliction2): (String, FType),
) -> StateAction {
    StateAction {
        name: format!("dstab {} {}", venom1, venom2),
        changes: vec![
            balance_change(BType::Balance, 2.8),
            flag_me(FType::Shield, false),
            afflict(affliction1),
            afflict(affliction2),
        ],
        initial: vec![
            alive(),
            target(alive()),
            target(lacks(FType::Rebounding)),
            target(lacks(FType::Shield)),
            has(BType::Balance),
            has(BType::Equil),
            target(lacks(affliction1)),
            target(lacks(affliction2)),
        ],
    }
}

pub fn bite_one(affliction: FType) -> StateAction {
    StateAction {
        name: "bite".into(),
        changes: vec![
            balance_change(BType::Balance, 1.9),
            flag_me(FType::Shield, false),
            afflict(affliction),
        ],
        initial: vec![
            alive(),
            target(alive()),
            target(lacks(FType::HardenedSkin)),
            target(lacks(FType::Shield)),
            has(BType::Balance),
            has(BType::Equil),
            target(lacks(affliction)),
        ],
    }
}

pub fn flay_one(defense: FType) -> StateAction {
    StateAction {
        name: "flay".into(),
        changes: vec![
            balance_change(BType::Balance, 2.5),
            flag_me(FType::Shield, false),
            strip_in_order(vec![defense]),
        ],
        initial: vec![
            alive(),
            target(alive()),
            has(BType::Balance),
            has(BType::Equil),
            target(some(vec![defense])),
        ],
    }
}

pub fn flay_action() -> StateAction {
    let flayable = vec![FType::Shield, FType::Rebounding, FType::HardenedSkin];
    StateAction {
        name: "flay".into(),
        changes: vec![
            balance_change(BType::Balance, 2.5),
            flag_me(FType::Shield, false),
            strip_in_order(flayable.clone()),
        ],
        initial: vec![
            alive(),
            target(alive()),
            has(BType::Balance),
            has(BType::Equil),
            target(some(flayable)),
        ],
    }
}

pub fn snipe_action(affliction: FType) -> StateAction {
    StateAction {
        name: "snipe".into(),
        changes: vec![
            attack_change(900),
            balance_change(BType::Balance, 3.25),
            afflict(affliction),
        ],
        initial: vec![
            alive(),
            target(alive()),
            has(BType::Balance),
            has(BType::Equil),
            target(lacks(affliction)),
        ],
    }
}
