use crate::timeline::{Observation, Prompt, TimelineState};
use crate::types::*;

// A list of states and their relative weights.
pub type VariableState = Vec<(TimelineState, u32)>;
pub type Activator = String;
pub type ActivatorFailure = String;
pub type ActivateResult = Result<Activator, ActivatorFailure>;

pub trait ActiveTransition {
    fn read(
        now: &TimelineState,
        observation: &Observation,
        before: &Vec<Observation>,
        after: &Vec<Observation>,
        prompt: &Prompt,
    ) -> Self;
    fn simulate(&self, now: TimelineState) -> VariableState;
    fn act(&self, now: TimelineState) -> ActivateResult;
}
