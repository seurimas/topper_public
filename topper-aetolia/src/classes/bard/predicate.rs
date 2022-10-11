use serde::*;
use topper_bt::unpowered::*;
use topper_core::timeline::CType;

use crate::{bt::*, classes::VENOM_AFFLICTS, timeline::apply_functions::apply_venom, types::*};

use super::actions::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum BardPredicate {
    Undithered,
    InRhythm,
    InHalfBeat,
    InWholeBeat,
    Runebanded,
    IronCollared,
    Globed,
    GlobeAffIsPriority,
    Awakened,
    PrimaryEmotion(Emotion),
    EmotionLevel(Emotion, CType),
    Bladestorm,
    HasAnelace(Option<usize>),
    Needled(Option<String>),
    NeedlingFor(FType),
    Singing(Option<Song>),
    Playing(Option<Song>),
}

impl TargetPredicate for BardPredicate {
    fn check(
        &self,
        aet_target: &AetTarget,
        model: &BehaviorModel,
        controller: &BehaviorController,
    ) -> bool {
        if let Some(target) = aet_target.get_target(model, controller) {
            match self {
                BardPredicate::Undithered => target
                    .check_if_bard(&|bard| bard.dithering == 0)
                    .unwrap_or(false),
                BardPredicate::InRhythm => target
                    .check_if_bard(&|bard| bard.tempo.is_some())
                    .unwrap_or(false),
                BardPredicate::InHalfBeat => target
                    .check_if_bard(&|bard| bard.half_beat.active())
                    .unwrap_or(false),
                BardPredicate::InWholeBeat => target
                    .check_if_bard(&|bard| bard.half_beat.resting())
                    .unwrap_or(false),
                BardPredicate::HasAnelace(min) => target
                    .check_if_bard(&|bard| bard.anelaces > min.unwrap_or(0))
                    .unwrap_or(false),
                BardPredicate::Runebanded => target.bard_board.runeband_state.is_active(),
                BardPredicate::IronCollared => target.bard_board.iron_collar_state.is_active(),
                BardPredicate::Globed => target.bard_board.globes_state.is_active(),
                BardPredicate::GlobeAffIsPriority => match target.bard_board.globes_state {
                    GlobesState::None => false,
                    GlobesState::Floating(aff_num) => {
                        if let (Some(priority_aff), Some(globe_aff)) = (
                            get_priority_aff(
                                aet_target,
                                model,
                                controller,
                                controller.aff_priorities.clone(),
                            ),
                            GLOBE_AFFS.get(GLOBE_AFFS.len() - aff_num),
                        ) {
                            *globe_aff == priority_aff
                        } else {
                            false
                        }
                    }
                },
                BardPredicate::Awakened => target.bard_board.emotion_state.awakened,
                BardPredicate::PrimaryEmotion(emotion) => {
                    target.bard_board.emotion_state.primary == Some(*emotion)
                }
                BardPredicate::EmotionLevel(_, _) => todo!(),
                BardPredicate::Bladestorm => target.bard_board.blades_count > 0,
                BardPredicate::Needled(None) => target.bard_board.needle_venom.is_some(),
                BardPredicate::NeedlingFor(aff) => {
                    if !target.bard_board.needling() {
                        false
                    } else if let Some(venom_aff) = target
                        .bard_board
                        .needle_venom
                        .as_ref()
                        .and_then(|venom| VENOM_AFFLICTS.get(venom))
                    {
                        venom_aff == aff
                    } else {
                        false
                    }
                }
                BardPredicate::Needled(Some(venom)) => target
                    .bard_board
                    .needle_venom
                    .as_ref()
                    .map(|needled| venom.eq(needled))
                    .unwrap_or(false),
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
