use serde::Deserialize;
use serde::Serialize;
use topper_bt::unpowered::*;

use crate::classes::bard::BardBehavior;
use crate::classes::VenomPlan;
use crate::observables::PlainAction;
use crate::timeline::*;
use crate::types::*;

use super::{BehaviorController, BehaviorModel};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum AetBehavior {
    UnstackAffs(Vec<FType>),
    PushAff(FType),
    PlainQebBehavior(String),
    BardBehavior(BardBehavior),
}

impl UnpoweredFunction for AetBehavior {
    type Model = BehaviorModel;
    type Controller = BehaviorController;

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState {
        match self {
            AetBehavior::UnstackAffs(unstacked) => {
                if let Some(priorities) = &mut controller.aff_priorities {
                    priorities.retain(|aff| !unstacked.contains(&aff.affliction()));
                    return UnpoweredFunctionState::Complete;
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetBehavior::PushAff(aff) => {
                if let Some(priorities) = &mut controller.aff_priorities {
                    priorities.insert(0, VenomPlan::Stick(*aff));
                    return UnpoweredFunctionState::Complete;
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetBehavior::PlainQebBehavior(action) => {
                controller
                    .plan
                    .add_to_qeb(Box::new(PlainAction::new(action.clone())));
                UnpoweredFunctionState::Complete
            }
            AetBehavior::BardBehavior(bard_behavior) => {
                bard_behavior.resume_with(model, controller)
            }
        }
    }

    fn reset(self: &mut Self, model: &Self::Model) {
        match self {
            _ => {}
        }
    }
}
