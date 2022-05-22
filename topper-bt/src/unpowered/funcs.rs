#[derive(Debug, PartialEq, Copy, Clone)]
pub enum UnpoweredFunctionState {
    Waiting,
    // The powered function failed to complete all work (bad state or negative result).
    Failed,
    // The powered function completed all work.
    Complete,
}

pub trait UnpoweredFunction {
    type Model: 'static;
    type Controller: 'static;
    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState;
    fn reset(self: &mut Self, model: &Self::Model);
}
