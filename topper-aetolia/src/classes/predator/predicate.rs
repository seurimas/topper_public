use serde::*;
use topper_bt::unpowered::*;
use topper_core::timeline::CType;

use crate::{bt::*, classes::VENOM_AFFLICTS, timeline::apply_functions::apply_venom, types::*};

use super::actions::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum PredatorPredicate {
    InStance(KnifeStance),
    CanFeint,
    Fleshbaned,
    FleshbanedOver(u32),
    Bloodscourged,
    TidalslashReady,
    Veinripped,
    Intoxicating(AetTarget),
    Intoxicated,
    Negated,
    ApexAtLeast(u32),
    HasOrgyuk,
    HasSpider,
    HasOrel,
}

impl TargetPredicate for PredatorPredicate {
    fn check(
        &self,
        aet_target: &AetTarget,
        model: &BehaviorModel,
        controller: &BehaviorController,
    ) -> bool {
        if let Some(target) = aet_target.get_target(model, controller) {
            match self {
                PredatorPredicate::InStance(stance) => target
                    .check_if_predator(&|predator| predator.stance == *stance)
                    .unwrap_or(false),
                PredatorPredicate::CanFeint => target
                    .check_if_predator(&|predator| predator.feint_time < 0)
                    .unwrap_or(false),
                PredatorPredicate::Fleshbaned => target.predator_board.fleshbane.is_active(),
                PredatorPredicate::FleshbanedOver(count) => {
                    target.predator_board.fleshbane_count >= *count
                }
                PredatorPredicate::Bloodscourged => target.predator_board.bloodscourge.is_active(),
                PredatorPredicate::TidalslashReady => target
                    .check_if_predator(&|predator| predator.tidalslash)
                    .unwrap_or(false),
                PredatorPredicate::Veinripped => target.predator_board.veinrip.is_active(),
                PredatorPredicate::Intoxicating(other_target) => target
                    .check_if_predator(&|predator| {
                        predator.is_intoxicating(&other_target.get_name(model, controller))
                    })
                    .unwrap_or(false),
                PredatorPredicate::Intoxicated => target.predator_board.is_intoxicated(),
                PredatorPredicate::Negated => target.predator_board.is_negated(),
                PredatorPredicate::ApexAtLeast(apex) => target
                    .check_if_predator(&|predator| predator.apex >= *apex)
                    .unwrap_or(false),
                PredatorPredicate::HasOrgyuk => target
                    .check_if_predator(&|predator| predator.has_orgyuk())
                    .unwrap_or(false),
                PredatorPredicate::HasSpider => target
                    .check_if_predator(&|predator| predator.has_spider())
                    .unwrap_or(false),
                PredatorPredicate::HasOrel => target
                    .check_if_predator(&|predator| predator.has_orel())
                    .unwrap_or(false),
            }
        } else {
            false
        }
    }
}
