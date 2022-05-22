use serde::*;
use topper_bt::unpowered::*;
use topper_core::timeline::CType;

use crate::{bt::*, types::*};

use super::actions::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum BardPredicate {
    Undithered,
    InHalfBeat,
    InWholeBeat,
    Runebanded,
    Globed,
    Awakened,
    PrimaryEmotion(Emotion),
    EmotionLevel(Emotion, CType),
    Bladestorm,
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
                BardPredicate::Undithered => target
                    .check_if_bard(&|bard| bard.dithering == 0)
                    .unwrap_or(false),
                BardPredicate::InHalfBeat => target
                    .check_if_bard(&|bard| bard.half_beat.active())
                    .unwrap_or(false),
                BardPredicate::InWholeBeat => target
                    .check_if_bard(&|bard| bard.half_beat.resting())
                    .unwrap_or(false),
                BardPredicate::Runebanded => target.bard_board.runeband_state.is_active(),
                BardPredicate::Globed => target.bard_board.globes_state.is_active(),
                BardPredicate::Awakened => target.bard_board.emotion_state.awakened,
                BardPredicate::PrimaryEmotion(emotion) => {
                    target.bard_board.emotion_state.primary == Some(*emotion)
                }
                BardPredicate::EmotionLevel(_, _) => todo!(),
                BardPredicate::Bladestorm => todo!(),
                BardPredicate::Needled(_) => todo!(),
                BardPredicate::Singing(Some(song)) => target
                    .check_if_bard(&|bard| bard.voice_song == Some(*song))
                    .unwrap_or(false),
                BardPredicate::Singing(None) => target
                    .check_if_bard(&|bard| bard.voice_song.is_some())
                    .unwrap_or(false),
                BardPredicate::Playing(Some(song)) => target
                    .check_if_bard(&|bard| bard.instrument_song == Some(*song))
                    .unwrap_or(false),
                BardPredicate::Playing(None) => target
                    .check_if_bard(&|bard| bard.instrument_song.is_some())
                    .unwrap_or(false),
            }
        } else {
            false
        }
    }
}
