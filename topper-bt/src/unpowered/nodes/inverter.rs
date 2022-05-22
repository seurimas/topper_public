use crate::unpowered::*;

pub struct Inverter<M, C> {
    node: Box<dyn UnpoweredFunction<Model = M, Controller = C> + Send + Sync>,
}

impl<M, C> Inverter<M, C> {
    pub fn new(node: Box<dyn UnpoweredFunction<Model = M, Controller = C> + Send + Sync>) -> Self {
        Inverter { node }
    }
}

impl<M: 'static, C: 'static> UnpoweredFunction for Inverter<M, C> {
    type Model = M;
    type Controller = C;
    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState {
        match self.node.resume_with(model, controller) {
            UnpoweredFunctionState::Complete => {
                return UnpoweredFunctionState::Failed;
            }
            UnpoweredFunctionState::Failed => {
                return UnpoweredFunctionState::Complete;
            }
            result => {
                // Waiting, NeedsGas
                return result;
            }
        }
    }

    fn reset(self: &mut Self, model: &Self::Model) {
        self.node.reset(model);
    }
}
