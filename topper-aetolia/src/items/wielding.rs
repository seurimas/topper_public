use crate::{observables::*, timeline::*, types::*};

pub struct WieldAction {
    caster: String,
    left: Option<String>,
    right: Option<String>,
    two_handed: Option<String>,
}

impl WieldAction {
    pub fn quick_wield(caster: String, wielded: String, left_hand: bool) -> Self {
        WieldAction {
            caster,
            left: if left_hand {
                Some(wielded.clone())
            } else {
                None
            },
            right: if !left_hand {
                Some(wielded.clone())
            } else {
                None
            },
            two_handed: None,
        }
    }
}

impl ActiveTransition for WieldAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        match (&self.left, &self.right, &self.two_handed) {
            (None, None, Some(wielded)) => {
                ProbableEvent::certain(vec![AetObservation::TwoHandedWield(
                    self.caster.clone(),
                    wielded.clone(),
                )])
            }
            (Some(left), None, None) => ProbableEvent::certain(vec![AetObservation::Wield {
                hand: "left".to_string(),
                what: left.clone(),
                who: self.caster.clone(),
            }]),
            (None, Some(right), None) => ProbableEvent::certain(vec![AetObservation::Wield {
                hand: "right".to_string(),
                what: right.clone(),
                who: self.caster.clone(),
            }]),
            (Some(left), Some(right), None) => {
                ProbableEvent::certain(vec![AetObservation::DualWield {
                    left: left.clone(),
                    right: right.clone(),
                    who: self.caster.clone(),
                }])
            }
            _ => ProbableEvent::certain(vec![]),
        }
    }

    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        match (&self.left, &self.right, &self.two_handed) {
            (None, None, Some(wielded)) => Ok(format!("quickwield both {}", wielded)),
            (Some(left), None, None) => Ok(format!("quickwield left {}", left)),
            (None, Some(right), None) => Ok(format!("quickwield right {}", right)),
            (Some(left), Some(right), None) => Ok(format!("quickwield both {} {}", left, right)),
            _ => Ok("echo BAD WIELD".to_string()),
        }
    }
}

pub struct UnwieldAction {
    caster: String,
    left_hand: bool,
}

impl UnwieldAction {
    pub fn unwield(caster: String, left_hand: bool) -> Self {
        UnwieldAction { caster, left_hand }
    }
}

impl ActiveTransition for UnwieldAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        if self.left_hand {
            if let Some(what) = timeline
                .state
                .borrow_agent(&self.caster)
                .wield_state
                .get_left()
            {
                ProbableEvent::certain(vec![AetObservation::Unwield {
                    who: self.caster.clone(),
                    what,
                    hand: "left".to_string(),
                }])
            } else {
                vec![]
            }
        } else {
            if let Some(what) = timeline
                .state
                .borrow_agent(&self.caster)
                .wield_state
                .get_right()
            {
                ProbableEvent::certain(vec![AetObservation::Unwield {
                    who: self.caster.clone(),
                    what,
                    hand: "right".to_string(),
                }])
            } else {
                vec![]
            }
        }
    }

    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        if self.left_hand {
            Ok("secure left;;unwield left".to_string())
        } else {
            Ok("secure right;;unwield right".to_string())
        }
    }
}
