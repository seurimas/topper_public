use crate::unpowered::*;

pub struct Executor<M, C> {
    nodes: Vec<Box<dyn UnpoweredFunction<Model = M, Controller = C> + Send + Sync>>,
    index: Option<usize>,
    success: bool,
}

impl<M, C> Executor<M, C> {
    pub fn new(
        nodes: Vec<Box<dyn UnpoweredFunction<Model = M, Controller = C> + Send + Sync>>,
    ) -> Self {
        Executor {
            nodes,
            index: None,
            success: false,
        }
    }
}

impl<M: 'static, C: 'static> UnpoweredFunction for Executor<M, C> {
    type Model = M;
    type Controller = C;
    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState {
        let mut running_index = self.index.unwrap_or(0);
        loop {
            if let Some(node) = self.nodes.get_mut(running_index) {
                let result = node.resume_with(model, controller);
                match result {
                    UnpoweredFunctionState::Failed => {
                        // Move on to the next node.
                        running_index += 1;
                    }
                    UnpoweredFunctionState::Complete => {
                        running_index += 1;
                        self.success = true;
                    }
                    _ => {
                        // Waiting, NeedsGas
                        self.index = Some(running_index);
                        return result;
                    }
                }
            } else {
                self.index = None;
                if self.success {
                    return UnpoweredFunctionState::Complete;
                }
                return UnpoweredFunctionState::Failed;
            }
        }
    }

    fn reset(self: &mut Self, _parameter: &Self::Model) {
        self.index = None;
        self.success = false;
    }
}
