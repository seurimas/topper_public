use crate::io::Topper;
use crate::timeline::{Observation, Prompt};

// A list of states and their relative weights.
pub type ActiveEvent = Vec<Observation>;
pub struct ProbableEvent(ActiveEvent, u32);
pub type Activator = String;
pub type ActivatorFailure = String;
pub type ActivateResult = Result<Activator, ActivatorFailure>;

impl ProbableEvent {
    pub fn new(observations: ActiveEvent, weight: u32) -> Self {
        ProbableEvent(observations, weight)
    }
}

pub trait ActiveTransition {
    fn act(&self, topper: &Topper) -> ActivateResult;
    fn simulate(&self, topper: &Topper) -> Vec<ProbableEvent>;
}

pub struct Inactivity;

impl ActiveTransition for Inactivity {
    fn act(&self, topper: &Topper) -> ActivateResult {
        Ok(format!(""))
    }
    fn simulate(&self, topper: &Topper) -> Vec<ProbableEvent> {
        vec![]
    }
}
