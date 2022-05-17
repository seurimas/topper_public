use std::marker::PhantomData;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum UnpoweredFunctionState {
    Waiting,
    // The powered function failed to complete all work (bad state or negative result).
    Failed,
    // The powered function completed all work.
    Complete,
}

pub trait UnpoweredFunction {
    type World: 'static;
    fn resume_with(self: &mut Self, parameter: &mut Self::World) -> UnpoweredFunctionState;
    fn reset(self: &mut Self, parameter: &mut Self::World);
}

pub struct Succeeder<R>(pub PhantomData<R>);

impl<R> Succeeder<R> {
    pub fn new() -> Self {
        Succeeder(PhantomData)
    }
}

impl<R: 'static> UnpoweredFunction for Succeeder<R> {
    type World = R;
    fn resume_with(self: &mut Self, _param: &mut Self::World) -> UnpoweredFunctionState {
        UnpoweredFunctionState::Complete
    }

    fn reset(self: &mut Self, _parameter: &mut Self::World) {
        // No state.
    }
}

pub struct Failer<R>(pub PhantomData<R>);

impl<R> Failer<R> {
    pub fn new() -> Self {
        Failer(PhantomData)
    }
}

impl<R: 'static> UnpoweredFunction for Failer<R> {
    type World = R;
    fn resume_with(self: &mut Self, _param: &mut Self::World) -> UnpoweredFunctionState {
        UnpoweredFunctionState::Failed
    }

    fn reset(self: &mut Self, _parameter: &mut Self::World) {
        // No state.
    }
}
