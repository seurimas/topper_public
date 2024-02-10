use serde::*;
use topper_bt::unpowered::*;
use topper_core::timeline::CType;

use crate::{bt::*, classes::VENOM_AFFLICTS, timeline::apply_functions::apply_venom, types::*};

use super::actions::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum MonkPredicate {
    InStance(MonkStance),
}

impl TargetPredicate for MonkPredicate {
    fn check(
        &self,
        aet_target: &AetTarget,
        model: &BehaviorModel,
        controller: &BehaviorController,
    ) -> bool {
        if let Some(target) = aet_target.get_target(model, controller) {
            match self {
                MonkPredicate::InStance(stance) => target
                    .check_if_monk(&|monk| monk.stance == *stance)
                    .unwrap_or(false),
            }
        } else {
            false
        }
    }
}
