use crate::classes::*;
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
            lines: vec![],
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
            lines: vec![],
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
            lines: vec![],
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
    fn test_dstab_absorbed() {
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
                Observation::Absorbed("Benedicto".into(), "Remnant".into()),
                Observation::Devenoms("kalmia".into()),
            ],
            lines: vec![],
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
        assert_eq!(bene_state.get_flag(FType::Anorexia), false);
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
            lines: vec![],
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
            lines: vec![],
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
            lines: vec![],
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
            lines: vec![],
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
            lines: vec![],
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
            lines: vec![],
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
                Observation::Rebounds,
                Observation::Devenoms("slike".into()),
                Observation::Rebounds,
                Observation::Devenoms("kalmia".into()),
            ],
            lines: vec![],
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
            lines: vec![],
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
    fn test_bite_absorbed() {
        let mut timeline = Timeline::new();
        let dstab_slice = TimeSlice {
            observations: vec![
                Observation::CombatAction(CombatAction {
                    caster: "Seurimas".to_string(),
                    category: "Assassination".to_string(),
                    skill: "Bite".to_string(),
                    target: "Benedicto".to_string(),
                    annotation: "scytherus".to_string(),
                }),
                Observation::Absorbed("Benedicto".into(), "Remnant".into()),
            ],
            lines: vec![],
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
        assert_eq!(bene_state.get_flag(FType::ThinBlood), false);
    }

    #[test]
    fn test_bite_parry() {
        let mut timeline = Timeline::new();
        let dstab_slice = TimeSlice {
            observations: vec![
                Observation::CombatAction(CombatAction {
                    caster: "Seurimas".to_string(),
                    category: "Assassination".to_string(),
                    skill: "Bite".to_string(),
                    target: "Benedicto".to_string(),
                    annotation: "scytherus".to_string(),
                }),
                Observation::Parry("Benedicto".to_string(), "head".to_string()),
            ],
            lines: vec![],
            prompt: Prompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(dstab_slice);
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.get_flag(FType::ThinBlood), false);
        assert_eq!(seur_state.get_parrying(), None);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Balance), true);
        assert_eq!(bene_state.get_flag(FType::ThinBlood), false);
        assert_eq!(bene_state.get_parrying(), Some(LType::HeadDamage));
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
            lines: vec![],
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
            lines: vec![],
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
    static ref SUGGESTION: Regex = Regex::new(r"suggest (\w+) ([^;%]+)").unwrap();
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
    fn simulate(&self, topper: &Topper) -> Vec<ProbableEvent> {
        Vec::new()
    }
    fn act(&self, topper: &Topper) -> ActivateResult {
        Ok(get_dstab_action(
            &topper,
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
    fn simulate(&self, topper: &Topper) -> Vec<ProbableEvent> {
        let mut observations = vec![Observation::CombatAction(CombatAction {
            caster: self.caster.clone(),
            target: self.target.clone(),
            annotation: self.annotation.clone(),
            category: "Assassination".to_string(),
            skill: "Flay".to_string(),
        })];
        if self.venom.len() > 0
            && (self.annotation.eq_ignore_ascii_case("shield")
                || self.annotation.eq_ignore_ascii_case("rebounding"))
        {
            observations.push(Observation::Devenoms(self.venom.clone()));
        }
        vec![ProbableEvent::new(observations, 1)]
    }
    fn act(&self, topper: &Topper) -> ActivateResult {
        Ok(get_flay_action(
            &topper,
            &self.target,
            self.annotation.clone(),
            self.venom.clone(),
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
    fn simulate(&self, topper: &Topper) -> Vec<ProbableEvent> {
        vec![ProbableEvent::new(
            vec![Observation::CombatAction(CombatAction {
                caster: self.caster.clone(),
                category: "Assassination".to_string(),
                skill: "Shrugging".to_string(),
                annotation: self.shrugged.clone(),
                target: "".to_string(),
            })],
            1,
        )]
    }
    fn act(&self, topper: &Topper) -> ActivateResult {
        Ok(format!("shrug {}", self.shrugged))
    }
}

pub struct BiteAction {
    pub caster: String,
    pub target: String,
    pub venom: String,
}

impl BiteAction {
    pub fn new(caster: String, target: String, venom: String) -> Self {
        BiteAction {
            caster,
            target,
            venom,
        }
    }
}

impl ActiveTransition for BiteAction {
    fn simulate(&self, topper: &Topper) -> Vec<ProbableEvent> {
        vec![ProbableEvent::new(
            vec![Observation::CombatAction(CombatAction {
                caster: self.caster.clone(),
                target: self.target.clone(),
                annotation: self.venom.clone(),
                category: "Assassination".to_string(),
                skill: "Bite".to_string(),
            })],
            1,
        )]
    }

    fn act(&self, topper: &Topper) -> ActivateResult {
        Ok(format!("bite {} {}", self.target, self.venom))
    }
}

pub struct BedazzleAction {
    pub caster: String,
    pub target: String,
}

impl BedazzleAction {
    pub fn new(caster: String, target: String) -> Self {
        BedazzleAction { caster, target }
    }
}

impl ActiveTransition for BedazzleAction {
    fn simulate(&self, topper: &Topper) -> Vec<ProbableEvent> {
        vec![]
    }

    fn act(&self, topper: &Topper) -> ActivateResult {
        Ok(format!("bedazzle {}", self.target))
    }
}

/**
 *
 * MOD ENTRY POINTS
 *
**/

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut TimelineState,
    _before: &Vec<Observation>,
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
            if let Some(Observation::Parry(who, _what)) = after.get(1) {
                if !who.eq(&combat_action.target) {
                    apply_venom(&mut you, &combat_action.annotation)?;
                }
            } else if let Some(Observation::Absorbed(who, _what)) = after.get(1) {
                if !who.eq(&combat_action.target) {
                    apply_venom(&mut you, &combat_action.annotation)?;
                }
            } else {
                apply_venom(&mut you, &combat_action.annotation)?;
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
        "Desway" => {
            let mut you = agent_states.get_agent(&combat_action.target);
            you.set_flag(FType::Hypnotized, false);
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
            you.push_suggestion(infer_suggestion(&combat_action.target, agent_states));
            agent_states.set_agent(&combat_action.caster, me);
            agent_states.set_agent(&combat_action.target, you);
        }
        "Fizzle" => {
            let mut me = agent_states.get_agent(&combat_action.target);
            me.pop_suggestion();
            agent_states.set_agent(&combat_action.target, me);
        }
        "Snap" => {
            if let Some(target) =
                agent_states.get_player_hint(&combat_action.caster, &"snap".into())
            {
                let mut you = agent_states.get_agent(&target);
                start_hypnosis(&mut you);
                agent_states.set_agent(&target, you);
            } else if !combat_action.target.eq(&"".to_string()) {
                let mut you = agent_states.get_agent(&combat_action.target);
                start_hypnosis(&mut you);
                agent_states.set_agent(&combat_action.target, you);
            }
        }
        "Bedazzle" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            let mut you = agent_states.get_agent(&combat_action.target);
            apply_or_infer_balance(&mut me, (BType::Balance, 2.25), after);
            apply_or_infer_random_afflictions(&mut you, after)?;
            agent_states.set_agent(&combat_action.caster, me);
            agent_states.set_agent(&combat_action.target, you);
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
    static ref PEACE_STACK: Vec<FType> = vec![
        FType::Stupidity,
        FType::Peace,
        FType::Paresis,
        FType::Clumsiness,
        FType::Asthma,
        FType::Allergies,
        FType::LeftLegBroken,
        FType::LeftArmBroken,
        FType::Dizziness,
        FType::Dizziness,
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
    static ref AGGRO_STACK: Vec<FType> = vec![
        FType::Paresis,
        FType::Asthma,
        FType::Clumsiness,
        FType::Stupidity,
        FType::Allergies,
        FType::Dizziness,
        FType::Vomiting,
        FType::LeftLegBroken,
        FType::RightLegBroken,
    ];
}

lazy_static! {
    static ref SALVE_STACK: Vec<FType> = vec![
        FType::LeftArmBroken,
        FType::RightArmBroken,
        FType::LeftLegBroken,
        FType::RightLegBroken,
        FType::Anorexia,
        FType::Slickness,
        FType::Paresis,
        FType::Stupidity,
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
    pub static ref SOFT_STACK: Vec<FType> = vec![FType::Asthma, FType::Anorexia, FType::Slickness,];
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
    static ref STACKING_STRATEGIES: HashMap<String, Vec<FType>> = {
        let mut val = HashMap::new();
        val.insert("coag".into(), COAG_STACK.to_vec());
        val.insert("dec".into(), DEC_STACK.to_vec());
        val.insert("phys".into(), PHYS_STACK.to_vec());
        val.insert("fire".into(), FIRE_STACK.to_vec());
        val.insert("aggro".into(), AGGRO_STACK.to_vec());
        val.insert("salve".into(), SALVE_STACK.to_vec());
        val.insert("peace".into(), PEACE_STACK.to_vec());
        val.insert("yedan".into(), YEDAN_STACK.to_vec());
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

fn should_bedazzle(topper: &Topper) -> bool {
    let me = topper
        .timeline
        .state
        .borrow_agent(&topper.timeline.who_am_i());
    me.is(FType::LeftArmBroken) && !me.is(FType::RightArmBroken)
}

fn should_regenerate(topper: &Topper) -> bool {
    let me = topper
        .timeline
        .state
        .borrow_agent(&topper.timeline.who_am_i());
    if let Some((_limb, damage, regenerating)) = me.get_restoring() {
        !regenerating && damage > 4000
    } else {
        false
    }
}

fn needs_restore(topper: &Topper) -> bool {
    let me = topper
        .timeline
        .state
        .borrow_agent(&topper.timeline.who_am_i());
    me.restore_count() > 0
        && me.restore_count() < 3
        && me.is(FType::Prone)
        && me.get_balance(BType::Salve) > 2.5
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

fn go_for_thin_blood(_topper: &Topper, you: &AgentState, _strategy: &String) -> bool {
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
}

pub fn should_lock(you: &AgentState, lockers: &Vec<&str>) -> bool {
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

pub fn get_flay_action(topper: &Topper, target: &String, def: String, v1: String) -> String {
    let action = if use_one_rag(topper) && !v1.eq_ignore_ascii_case("") {
        format!("hw {};;flay {} {}", v1, target, def)
    } else {
        format!("flay {} {} {}", target, def, v1)
    };
    let action = if should_call_venoms(topper) {
        format!("{};;{}", call_venom(target, &v1), action)
    } else {
        action
    };

    action
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

/*
pub fn get_slit_action(topper: &Topper, target: &String, v1: String) -> String {
    if use_one_rag(topper) {
        format!("hr {};;slit {}", v1, target)
    } else {
        format!("slit {} {}", target, v1)
    }
}
*/

pub fn get_balance_attack(
    topper: &Topper,
    target: &String,
    strategy: &String,
) -> Box<dyn ActiveTransition> {
    if let Some(stack) = STACKING_STRATEGIES.get(strategy) {
        let you = topper.timeline.state.borrow_agent(target);
        if needs_shrugging(&topper) {
            return Box::new(ShruggingAction::shrug_asthma(topper.me()));
        } else if needs_restore(&topper) {
            return Box::new(RestoreAction::new(topper.me()));
        } else if get_equil_attack(topper, target, strategy).starts_with("seal") {
            return Box::new(Inactivity);
        } else if you.is(FType::Shielded) || you.is(FType::Rebounding) {
            let defense = if you.is(FType::Shielded) {
                "shield"
            } else {
                "rebounding"
            };
            if let Some(venom) = get_venoms(stack.to_vec(), 1, &you).pop() {
                return Box::new(FlayAction::new(
                    topper.me(),
                    target.to_string(),
                    defense.to_string(),
                    venom.to_string(),
                ));
            } else {
                return Box::new(FlayAction::new(
                    topper.me(),
                    target.to_string(),
                    defense.to_string(),
                    "".to_string(),
                ));
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
                    if you.is(FType::Fangbarrier) {
                        return Box::new(FlayAction::fangbarrier(topper.me(), target.to_string()));
                    } else {
                        return Box::new(BiteAction::new(
                            topper.me(),
                            target.to_string(),
                            "scytherus".to_string(),
                        ));
                    }
                }
                let mut buffer = get_venoms(THIN_BUFFER_STACK.to_vec(), 2, &you);
                if you.lock_duration().map_or(false, |dur| dur > 10.0) && !you.is(FType::Voyria) {
                    buffer.insert(buffer.len(), "voyria");
                }
                if you.is(FType::ThinBlood) && buffer.len() > 0 {
                    println!("Buffering! {:?} {:?}", venoms, buffer);
                    add_buffers(&mut venoms, &buffer);
                } else {
                    let mut hypno_buffers = vec![];
                    if you.is(FType::Impatience)
                        || you.get_next_hypno_aff() == Some(FType::Impatience)
                    {
                        hypno_buffers.push(FType::Shyness);
                    }
                    if you.is(FType::Loneliness)
                        || you.get_next_hypno_aff() == Some(FType::Loneliness)
                    {
                        hypno_buffers.push(FType::Recklessness);
                    }
                    if you.is(FType::Generosity)
                        || you.get_next_hypno_aff() == Some(FType::Generosity)
                    {
                        hypno_buffers.push(FType::Peace);
                        hypno_buffers.push(FType::Stupidity);
                    }
                    let hypno_buffers = get_venoms(hypno_buffers, 1, &you);
                    add_buffers(&mut venoms, &hypno_buffers);
                }
            }
            let v1 = venoms.pop();
            let v2 = venoms.pop();
            if should_bedazzle(&topper) {
                println!("Bedazzling!");
                return Box::new(BedazzleAction::new(topper.me(), target.to_string()));
            } else if you.is(FType::Hypersomnia) && !you.is(FType::Asleep) {
                return Box::new(DoublestabAction::new(
                    topper.me(),
                    target.to_string(),
                    "delphinium".to_string(),
                    "delphinium".to_string(),
                ));
            } else if let (Some(v1), Some(v2)) = (v1, v2) {
                return Box::new(DoublestabAction::new(
                    topper.me(),
                    target.to_string(),
                    v1.to_string(),
                    v2.to_string(),
                ));
            } else if you.is(FType::Fangbarrier) {
                return Box::new(FlayAction::fangbarrier(topper.me(), target.to_string()));
            } else {
                return Box::new(BiteAction::new(
                    topper.me(),
                    target.to_string(),
                    "camus".to_string(),
                ));
            }
        }
    } else if strategy == "damage" {
        let you = topper.timeline.state.borrow_agent(target);
        if you.is(FType::Fangbarrier) {
            return Box::new(FlayAction::fangbarrier(topper.me(), target.to_string()));
        } else {
            return Box::new(BiteAction::new(
                topper.me(),
                target.to_string(),
                "camus".to_string(),
            ));
        }
    } else {
        return Box::new(Inactivity);
    }
}

pub fn get_equil_attack(topper: &Topper, target: &String, strategy: &String) -> String {
    if strategy.eq("damage") {
        return "".to_string();
    }
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
                if you.lock_duration().is_some() {
                    format!(";;shadow sleight blank {}", target)
                } else {
                    format!(";;shadow sleight dissipate {}", target)
                }
            } else {
                format!("%%qs shadow sleight void {}", target)
            }
        } else {
            format!(";;shadow sleight abrasion {}", target)
        }
    }
}

pub fn get_snap(topper: &Topper, target: &String, _strategy: &String) -> bool {
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
    let mut balance = get_balance_attack(topper, target, strategy);
    if should_regenerate(&topper) {
        balance = Box::new(RegenerateAction::new(topper.me()));
    }
    let equil = get_equil_attack(topper, target, strategy);
    let shadow = get_shadow_attack(topper, target, strategy);
    let should_snap = get_snap(topper, target, strategy);
    let mut attack: String = if should_snap {
        format!("snap {}", target)
    } else {
        "".to_string()
    };
    if let Ok(activation) = balance.act(&topper) {
        attack = format!("qeb {}", activation);
    }
    if equil != "" {
        attack = format!("{};;{}", attack, equil);
    }
    if shadow != "" {
        attack = format!("{}{}", attack, shadow);
    }
    attack
}
