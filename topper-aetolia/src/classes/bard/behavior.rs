use serde::*;
use topper_bt::unpowered::*;

use crate::{bt::*, types::bards::*};

use super::actions::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum BardBehavior {
    Weave(Weavable),
    WeaveAttack(WeavingAttack),
    PerformanceAttack(PerformanceAttack),
    SingSong(Song),
    PlaySong(Song),
}

impl UnpoweredFunction for BardBehavior {
    type Model = BehaviorModel;
    type Controller = BehaviorController;

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState {
        match self {
            BardBehavior::Weave(weavable) => {
                if model
                    .state
                    .borrow_me()
                    .check_if_bard(&|bard| bard.dithering > 0)
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                controller
                    .plan
                    .add_to_qeb(Box::new(WeavingAction::new(model.who_am_i(), *weavable)));
            }
            BardBehavior::WeaveAttack(weave_attack) => {
                if model
                    .state
                    .borrow_me()
                    .check_if_bard(&|bard| bard.dithering > 0)
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                if let Some(target) = &controller.target {
                    controller
                        .plan
                        .add_to_qeb(Box::new(WeavingAttackAction::new(
                            model.who_am_i(),
                            target.to_string(),
                            *weave_attack,
                        )));
                } else {
                    return UnpoweredFunctionState::Failed;
                }
            }
            BardBehavior::PerformanceAttack(performance_attack) => {
                if let Some(target) = &controller.target {
                    controller
                        .plan
                        .add_to_qeb(Box::new(PerformanceAttackAction::new(
                            model.who_am_i(),
                            target.to_string(),
                            performance_attack.clone(),
                        )));
                } else {
                    return UnpoweredFunctionState::Failed;
                }
            }
            BardBehavior::SingSong(sing_song) => {
                if model
                    .state
                    .borrow_me()
                    .check_if_bard(&|bard| bard.voice_song.is_some())
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                controller
                    .plan
                    .add_to_qeb(Box::new(SongAction::sing(model.who_am_i(), *sing_song)));
            }
            BardBehavior::PlaySong(play_song) => {
                if model
                    .state
                    .borrow_me()
                    .check_if_bard(&|bard| bard.instrument_song.is_some())
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                controller
                    .plan
                    .add_to_qeb(Box::new(SongAction::play(model.who_am_i(), *play_song)));
            }
        }
        UnpoweredFunctionState::Complete
    }

    fn reset(self: &mut Self, parameter: &Self::Model) {
        // Nothing to do.
    }
}
