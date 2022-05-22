use crate::unpowered::*;

pub struct Failer<M, C> {
    node: Box<dyn UnpoweredFunction<Model = M, Controller = C> + Send + Sync>,
}

impl<M, C> Failer<M, C> {
    pub fn new(node: Box<dyn UnpoweredFunction<Model = M, Controller = C> + Send + Sync>) -> Self {
        Failer { node }
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
        match self.node.resume_with(model, controller) {
            UnpoweredFunctionState::Failed | UnpoweredFunctionState::Complete => {
                return UnpoweredFunctionState::Failed;
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
