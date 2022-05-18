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
    type Model: 'static;
    type Controller: 'static;
    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState;
    fn reset(self: &mut Self, model: &Self::Model);
}

pub struct Succeeder<M, C>(pub PhantomData<M>, pub PhantomData<C>);

impl<M, C> Succeeder<M, C> {
    pub fn new() -> Self {
        Succeeder(PhantomData, PhantomData)
    }
}

impl<M: 'static, C: 'static> UnpoweredFunction for Succeeder<M, C> {
    type Model = M;
    type Controller = C;

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState {
        UnpoweredFunctionState::Complete
    }

    fn reset(self: &mut Self, model: &Self::Model) {
        // No state.
    }
}

pub struct Failer<M, C>(pub PhantomData<M>, pub PhantomData<C>);

impl<M, C> Failer<M, C> {
    pub fn new() -> Self {
        Failer(PhantomData, PhantomData)
    }
}

impl<M: 'static, C: 'static> UnpoweredFunction for Failer<M, C> {
    type Model = M;
    type Controller = C;

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState {
        UnpoweredFunctionState::Failed
    }

    fn reset(self: &mut Self, model: &Self::Model) {
        // No state.
    }
}
