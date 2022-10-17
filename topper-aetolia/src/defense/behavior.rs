use serde::*;
use topper_bt::unpowered::*;
use topper_core::timeline::db::DummyDatabaseModule;

use crate::{
    bt::*,
    classes::{FitnessAction, ParryAction},
};

use super::{get_needed_parry, get_needed_refills};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum DefenseBehavior {
    Parry,
    Repipe,
    Fitness,
}

impl UnpoweredFunction for DefenseBehavior {
    type Model = BehaviorModel;
    type Controller = BehaviorController;

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState {
        match self {
            DefenseBehavior::Parry => {
                if let Some(limb) = get_needed_parry(
                    model,
                    &model.who_am_i(),
                    &controller.target.clone().unwrap_or_default(),
                    &"".to_string(),
                    None as Option<&DummyDatabaseModule>,
                ) {
                    controller
                        .plan
                        .add_to_qeb(Box::new(ParryAction::new(model.who_am_i(), limb)));
                }
            }
            DefenseBehavior::Repipe => {
                let refill_actions = get_needed_refills(&model.state.borrow_me());
                for action in refill_actions {
                    controller.plan.add_to_qeb(Box::new(action));
                }
            }
            DefenseBehavior::Fitness => {
                let me = model.state.borrow_me();
                if me.lock_duration().is_some() {
                    controller
                        .plan
                        .add_to_qeb(Box::new(FitnessAction::new(model.who_am_i())));
                }
            }
        }
        UnpoweredFunctionState::Complete
    }

    fn reset(self: &mut Self, model: &Self::Model) {
        // Do nothing...
    }
}
