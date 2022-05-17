use crate::unpowered::*;

pub struct Selector<R> {
    nodes: Vec<Box<dyn UnpoweredFunction<World = R>>>,
    index: Option<usize>,
}

impl<R> Selector<R> {
    pub fn new(nodes: Vec<Box<dyn UnpoweredFunction<World = R>>>) -> Self {
        Selector { nodes, index: None }
    }
}

impl<R: 'static> UnpoweredFunction for Selector<R> {
    type World = R;
    fn resume_with(self: &mut Self, parameter: &mut Self::World) -> UnpoweredFunctionState {
        let mut running_index = self.index.unwrap_or(0);
        loop {
            if let Some(node) = self.nodes.get_mut(running_index) {
                let result = node.resume_with(parameter);
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

    fn reset(self: &mut Self, _parameter: &mut Self::World) {
        self.index = None;
    }
}
