use crate::unpowered::*;

pub struct Repeat<R> {
    node: Box<dyn UnpoweredFunction<World = R>>,
    runs: usize,
    runs_left: usize,
}

impl<R> Repeat<R> {
    pub fn new(node: Box<dyn UnpoweredFunction<World = R>>, runs: usize) -> Self {
        Repeat {
            node,
            runs,
            runs_left: runs,
        }
    }
}

impl<R: 'static> UnpoweredFunction for Repeat<R> {
    type World = R;
    fn resume_with(self: &mut Self, parameter: &mut Self::World) -> UnpoweredFunctionState {
        while self.runs_left > 0 {
            let result = self.node.resume_with(parameter);
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

    fn reset(self: &mut Self, _parameter: &mut Self::World) {
        self.runs_left = self.runs;
    }
}
pub struct RepeatUntilFail<R> {
    node: Box<dyn UnpoweredFunction<World = R>>,
}

impl<R> RepeatUntilFail<R> {
    pub fn new(node: Box<dyn UnpoweredFunction<World = R>>) -> Self {
        RepeatUntilFail { node }
    }
}

impl<R: 'static> UnpoweredFunction for RepeatUntilFail<R> {
    type World = R;
    fn resume_with(self: &mut Self, parameter: &mut Self::World) -> UnpoweredFunctionState {
        loop {
            let result = self.node.resume_with(parameter);
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

    fn reset(self: &mut Self, _parameter: &mut Self::World) {
        // Nothing to do.
    }
}
