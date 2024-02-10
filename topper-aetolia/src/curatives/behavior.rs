use std::ops::DerefMut;

use serde::*;
use topper_bt::unpowered::*;
use topper_core::timeline::db::DummyDatabaseModule;

use crate::{
    bt::*,
    classes::{FitnessAction, ParryAction},
    db::AetDatabaseModule,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum CurativeBehavior {
    Parry,
    Repipe,
    Fitness,
    Dodge,
}

impl UnpoweredFunction for CurativeBehavior {
    type Model = BehaviorModel;
    type Controller = BehaviorController;

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState {
        UnpoweredFunctionState::Failed
    }

    fn reset(self: &mut Self, model: &Self::Model) {
        // Nothing to do
    }
}
