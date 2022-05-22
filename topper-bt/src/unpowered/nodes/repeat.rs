use crate::unpowered::*;

pub struct Repeat<M, C> {
    node: Box<dyn UnpoweredFunction<Model = M, Controller = C> + Send + Sync>,
    runs: usize,
    runs_left: usize,
}

impl<M, C> Repeat<M, C> {
    pub fn new(
        node: Box<dyn UnpoweredFunction<Model = M, Controller = C> + Send + Sync>,
        runs: usize,
    ) -> Self {
        Repeat {
            node,
            runs,
            runs_left: runs,
        }
    }
}

impl<M: 'static, C: 'static> UnpoweredFunction for Repeat<M, C> {
    type Model = M;
    type Controller = C;
    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState {
        while self.runs_left > 0 {
            let result = self.node.resume_with(model, controller);
            match result {
                UnpoweredFunctionState::Failed => {
                    self.runs_left = self.runs;
                    return result;
                }
                UnpoweredFunctionState::Complete => {
                    self.runs_left -= 1;
                }
                _ => {
                    // Waiting
                    return result;
                }
            }
        }
        self.runs_left = self.runs;
        return UnpoweredFunctionState::Complete;
    }

    fn reset(self: &mut Self, model: &Self::Model) {
        self.runs_left = self.runs;
    }
}
pub struct RepeatUntilFail<M, C> {
    node: Box<dyn UnpoweredFunction<Model = M, Controller = C> + Send + Sync>,
}

impl<M, C> RepeatUntilFail<M, C> {
    pub fn new(node: Box<dyn UnpoweredFunction<Model = M, Controller = C> + Send + Sync>) -> Self {
        RepeatUntilFail { node }
    }
}

impl<M: 'static, C: 'static> UnpoweredFunction for RepeatUntilFail<M, C> {
    type Model = M;
    type Controller = C;
    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState {
        loop {
            let result = self.node.resume_with(model, controller);
            match result {
                UnpoweredFunctionState::Failed => {
                    return UnpoweredFunctionState::Complete;
                }
                UnpoweredFunctionState::Complete => {
                    // We'll be stepping the current node again.
                    continue;
                }
                _ => {
                    // Waiting, NeedsGas
                    return result;
                }
            }
        }
    }

    fn reset(self: &mut Self, model: &Self::Model) {
        // Nothing to do.
    }
}

pub struct RepeatUntilSuccess<M, C> {
    node: Box<dyn UnpoweredFunction<Model = M, Controller = C> + Send + Sync>,
}

impl<M, C> RepeatUntilSuccess<M, C> {
    pub fn new(node: Box<dyn UnpoweredFunction<Model = M, Controller = C> + Send + Sync>) -> Self {
        RepeatUntilSuccess { node }
    }
}

impl<M: 'static, C: 'static> UnpoweredFunction for RepeatUntilSuccess<M, C> {
    type Model = M;
    type Controller = C;
    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState {
        loop {
            let result = self.node.resume_with(model, controller);
            match result {
                UnpoweredFunctionState::Complete => {
                    return UnpoweredFunctionState::Complete;
                }
                UnpoweredFunctionState::Failed => {
                    // We'll be stepping the current node again.
                    continue;
                }
                _ => {
                    // Waiting, NeedsGas
                    return result;
                }
            }
        }
    }

    fn reset(self: &mut Self, model: &Self::Model) {
        // Nothing to do.
    }
}
