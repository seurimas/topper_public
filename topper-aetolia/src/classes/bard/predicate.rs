use serde::*;
use topper_bt::unpowered::*;
use topper_core::timeline::CType;

use crate::{bt::*, types::*};

use super::actions::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum BardPredicate {
    InHalfBeat,
    InWholeBeat,
    Runebanded(Option<(usize, usize)>),
    Globed(Option<(usize, usize)>),
    Awakened,
    PrimaryEmotion(Emotion),
    EmotionLevel(Emotion, CType),
    Bladestorm(Option<(usize, usize)>),
    Needled(Option<String>),
    Singing(Option<Song>),
    Playing(Option<Song>),
}

impl TargetPredicate for BardPredicate {
    fn check(
        &self,
        target: &AetTarget,
        world: &BehaviorModel,
        controller: &BehaviorController,
    ) -> bool {
        if let Some(target) = target.get_target(world, controller) {
            match self {
                BardPredicate::InHalfBeat => target
                    .check_if_bard(|bard| bard.half_beat.active())
                    .unwrap_or(false),
                BardPredicate::InWholeBeat => target
                    .check_if_bard(|bard| bard.half_beat.resting())
                    .unwrap_or(false),
                BardPredicate::Runebanded(_) => todo!(),
                BardPredicate::Globed(_) => target.bard_board.globes_state != GlobesState::None,
                BardPredicate::Awakened => todo!(),
                BardPredicate::PrimaryEmotion(_) => todo!(),
                BardPredicate::EmotionLevel(_, _) => todo!(),
                BardPredicate::Bladestorm(_) => todo!(),
                BardPredicate::Needled(_) => todo!(),
                BardPredicate::Singing(_) => todo!(),
                BardPredicate::Playing(_) => todo!(),
            }
        } else {
            false
        }
    }
}
