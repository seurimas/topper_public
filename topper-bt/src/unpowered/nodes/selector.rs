use crate::unpowered::*;

pub struct Selector<M, C> {
    nodes: Vec<Box<dyn UnpoweredFunction<Model = M, Controller = C> + Send + Sync>>,
    index: Option<usize>,
}

impl<M, C> Selector<M, C> {
    pub fn new(
        nodes: Vec<Box<dyn UnpoweredFunction<Model = M, Controller = C> + Send + Sync>>,
    ) -> Self {
        Selector { nodes, index: None }
    }
}

impl<M: 'static, C: 'static> UnpoweredFunction for Selector<M, C> {
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
                        self.index = None;
                        return result;
                    }
                    _ => {
                        // Waiting, NeedsGas
                        self.index = Some(running_index);
                        return result;
                    }
                }
            } else {
                self.index = None;
                return UnpoweredFunctionState::Failed;
            }
        }
    }

    fn reset(self: &mut Self, _parameter: &Self::Model) {
        self.index = None;
    }
}
