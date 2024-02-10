use serde::*;
use topper_bt::unpowered::*;
use topper_core::timeline::CType;

use crate::{bt::*, classes::VENOM_AFFLICTS, timeline::apply_functions::apply_venom, types::*};

use super::actions::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum InfiltratorPredicate {
    Sealed,
    HypnoFiring,
    TopHypnoAffIs(FType),
    HasFinesse,
}

impl TargetPredicate for InfiltratorPredicate {
    fn check(
        &self,
        aet_target: &AetTarget,
        model: &BehaviorModel,
        controller: &BehaviorController,
    ) -> bool {
        if let Some(target) = aet_target.get_target(model, controller) {
            match self {
                InfiltratorPredicate::Sealed => target.hypno_state.is_sealed(),
                InfiltratorPredicate::HypnoFiring => target.hypno_state.is_firing(),
                InfiltratorPredicate::TopHypnoAffIs(aff) => {
                    Some(*aff) == target.hypno_state.get_next_hypno_aff()
                }
                InfiltratorPredicate::HasFinesse => target
                    .check_if_infiltrator(&|infiltrator| infiltrator.finesse > 0)
                    .unwrap_or(false),
            }
        } else {
            false
        }
    }
}
