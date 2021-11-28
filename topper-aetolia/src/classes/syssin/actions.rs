use super::*;
use crate::alpha_beta::ActionPlanner;
use crate::classes::*;
use crate::curatives::get_cure_depth;
use crate::observables::*;
use crate::timeline::*;
use crate::types::*;
use regex::Regex;
use std::collections::HashMap;

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

    pub fn fangbarrier(caster: String, target: String, venom: String) -> Self {
        FlayAction {
            caster,
            target,
            annotation: "fangbarrier".to_string(),
            venom,
        }
    }
}

impl ActiveTransition for FlayAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        let mut observations = vec![CombatAction::observation(
            &self.caster,
            &"Assassination",
            &"Flay",
            &self.annotation,
            &self.target,
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
            &"Assassination",
            &"Slit",
            &"",
            &self.target,
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
            &"Assassination",
            &"Shrugging",
            &self.shrugged,
            &"",
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
    pub limb: Option<String>,
}

impl BiteAction {
    pub fn new(caster: &str, target: &str, venom: &str) -> Self {
        BiteAction {
            caster: caster.to_string(),
            target: target.to_string(),
            venom: venom.to_string(),
            limb: None,
        }
    }

    pub fn camus(caster: &str, target: &str, limb: &str) -> Self {
        BiteAction {
            caster: caster.to_string(),
            target: target.to_string(),
            venom: "camus".to_string(),
            limb: Some(limb.to_string()),
        }
    }
}

impl ActiveTransition for BiteAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        ProbableEvent::certain(vec![CombatAction::observation(
            &self.caster,
            &"Assassination",
            &"Bite",
            &self.venom,
            &self.target,
        )])
    }

    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        if let Some(limb) = &self.limb {
            Ok(format!(
                "target {};;bite {} {};;target nothing",
                limb, self.target, self.venom
            ))
        } else {
            Ok(format!("bite {} {}", self.target, self.venom))
        }
    }
}

pub struct GarroteAction {
    pub caster: String,
    pub target: String,
}

impl GarroteAction {
    pub fn new(caster: &str, target: &str) -> Self {
        GarroteAction {
            caster: caster.to_string(),
            target: target.to_string(),
        }
    }
}

impl ActiveTransition for GarroteAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        ProbableEvent::certain(vec![CombatAction::observation(
            &self.caster,
            &"Assassination",
            &"Garrote",
            &"",
            &self.target,
        )])
    }

    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(format!("garrote {}", self.target))
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
            &"Hypnosis",
            &"Hypnotise",
            &"",
            &self.target,
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
            Hypnosis::Eradicate => format!("eradicate"),
            Hypnosis::Trigger(word) => format!("trigger {}", word),
        };
        format!("suggest {} {}", self.target, suggestion_string)
    }
}

impl ActiveTransition for SuggestAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        ProbableEvent::certain(vec![
            AetObservation::Sent(self.get_suggestion()),
            CombatAction::observation(&self.caster, &"Hypnosis", &"Suggest", &"", &self.target),
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
            CombatAction::observation(&self.caster, &"Hypnosis", &"Suggest", &"", &self.target),
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
            CombatAction::observation(&self.caster, &"Hypnosis", &"Snap", &"", &self.target),
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
            &"Hypnosis",
            &"Sleight",
            &self.sleight,
            &self.target,
        )])
    }
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(format!("shadow sleight {} {}", self.sleight, self.target))
    }
}
