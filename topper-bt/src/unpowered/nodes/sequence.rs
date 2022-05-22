use crate::unpowered::*;

pub struct Sequence<M, C> {
    nodes: Vec<Box<dyn UnpoweredFunction<Model = M, Controller = C> + Send + Sync>>,
    index: Option<usize>,
}

impl<M, C> Sequence<M, C> {
    pub fn new(
        nodes: Vec<Box<dyn UnpoweredFunction<Model = M, Controller = C> + Send + Sync>>,
    ) -> Self {
        Sequence { nodes, index: None }
    }
}

impl<M: 'static, C: 'static> UnpoweredFunction for Sequence<M, C> {
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
                    UnpoweredFunctionState::Complete => {
                        // Move on to the next node.
                        running_index += 1;
                    }
                    UnpoweredFunctionState::Failed => {
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
                return UnpoweredFunctionState::Complete;
            }
        }
    }

    fn reset(self: &mut Self, model: &Self::Model) {
        self.index = None;
    }
}
