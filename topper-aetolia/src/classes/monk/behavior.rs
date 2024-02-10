use serde::*;
use topper_bt::unpowered::*;

use crate::{
    bt::*, classes::group::*, non_agent::AetTimelineRoomExt, observables::PlainAction, types::*,
};

use super::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum MonkBehavior {
    // Class cure.
    Push(Option<AetTarget>),
    // Combo attacks
    AddComboAttacks(Vec<MonkComboAttack>),
    Combo(AetTarget, Vec<MonkComboGrader>, Option<CType>),
    // Non-combo Tekura actions
    Backbreaker(AetTarget),
    // Kaido attacks
    Choke(AetTarget),
    Cripple(AetTarget),
    Strike(AetTarget),
    Ripple(AetTarget),
    Enfeeble(AetTarget),
    // Telepathy actions
    MindLock(AetTarget),
    // Telepathy attacks
    Fear(AetTarget),
    Paralyse(AetTarget),
    Confuse(AetTarget),
    Recklessness(AetTarget),
    Epilepsy(AetTarget),
    Pacify(AetTarget),
    Stupidity(AetTarget),
    Anorexia(AetTarget),
    Amnesia(AetTarget),
    Deadening(AetTarget),
    Strip(AetTarget),
    Crush(AetTarget),
    Batter(AetTarget),
}

impl UnpoweredFunction for MonkBehavior {
    type Model = BehaviorModel;
    type Controller = BehaviorController;

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState {
        match self {
            MonkBehavior::MindLock(target) => {
                if let Some(target) = target.get_target(model, controller) {
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            _ => UnpoweredFunctionState::Failed,
        }
    }

    fn reset(self: &mut Self, model: &Self::Model) {}
}
