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
    pub fn certain(observations: ActiveEvent) -> Vec<Self> {
        vec![Self::new(observations, 1)]
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

pub struct SeparatorAction<A: ActiveTransition, B: ActiveTransition>(A, B);

impl<A: ActiveTransition, B: ActiveTransition> SeparatorAction<A, B> {
    pub fn pair(first: A, second: B) -> Self {
        SeparatorAction(first, second)
    }
}

impl<A: ActiveTransition, B: ActiveTransition> ActiveTransition for SeparatorAction<A, B> {
    fn act(&self, topper: &Topper) -> ActivateResult {
        Ok(format!(
            "{};;{}",
            self.0.act(&topper)?,
            self.1.act(&topper)?
        ))
    }
    fn simulate(&self, topper: &Topper) -> Vec<ProbableEvent> {
        let mut results = vec![];
        for ProbableEvent(simulate_first, weight_first) in self.0.simulate(&topper).iter() {
            for ProbableEvent(simulate_second, weight_second) in self.1.simulate(&topper).iter() {
                let mut observations = vec![];
                observations.append(&mut simulate_first.clone());
                observations.append(&mut simulate_second.clone());
                results.push(ProbableEvent(observations, weight_first * weight_second));
            }
        }
        results
    }
}
