use std::collections::HashMap;

use serde::*;

use super::*;

#[derive(Debug, Deserialize, Serialize, Display, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Song {
    Origin,
    Charity,
    Fascination,
    Youth,
    Feasting,
    Decadence,
    Unheard,
    Sorrow,
    Merriment,
    Doom,
    Foundation,
    Destiny,
    Tranquility,
    Awakening,
    Harmony,
    Remembrance,
    Hero,
    Mythics,
    Fate,
    Oblivion,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HalfbeatState {
    Inactive,
    HalfBeat(CType),
    WholeBeat(CType),
}

impl Default for HalfbeatState {
    fn default() -> Self {
        Self::Inactive
    }
}

impl HalfbeatState {
    pub fn active(&self) -> bool {
        match self {
            HalfbeatState::HalfBeat(_) => true,
            _ => false,
        }
    }
    pub fn resting(&self) -> bool {
        match self {
            HalfbeatState::WholeBeat(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct BardClassState {
    pub dithering: usize,
    pub tempo: Option<(usize, CType)>,
    pub voice_song: Option<Song>,
    pub instrument_song: Option<Song>,
    pub half_beat: HalfbeatState,
}

impl BardClassState {
    pub fn on_tempo(&mut self) {
        let tempo_count = if let Some((count, _timer)) = self.tempo {
            count + 1
        } else {
            1
        };
        if tempo_count > 3 {
            self.tempo = None;
        } else {
            self.tempo = Some((tempo_count, 0));
        }
    }

    pub fn off_tempo(&mut self) {
        self.tempo = None;
    }

    pub fn half_beat_pickup(&mut self) {
        self.half_beat = HalfbeatState::HalfBeat(0);
    }

    pub fn half_beat_slowdown(&mut self) {
        self.half_beat = HalfbeatState::WholeBeat(0);
    }

    pub fn half_beat_end(&mut self) {
        self.half_beat = HalfbeatState::Inactive;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RunebandState {
    Inactive,
    Normal(usize),
    Reverse(usize),
}

impl Default for RunebandState {
    fn default() -> Self {
        Self::Inactive
    }
}

impl RunebandState {
    pub fn initial() -> Self {
        Self::Normal(0)
    }

    pub fn is_active(&self) -> bool {
        match self {
            Self::Inactive => false,
            _ => true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GlobesState {
    None,
    Floating(usize),
}

impl Default for GlobesState {
    fn default() -> Self {
        Self::None
    }
}

impl GlobesState {
    pub fn initial() -> Self {
        Self::Floating(3)
    }

    pub fn is_active(&self) -> bool {
        match self {
            Self::None => false,
            _ => true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Emotion {
    Sadness,
    Happiness,
    Surprise,
    Anger,
    Stress,
    Fear,
    Disgust,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct EmotionState {
    pub awakened: bool,
    pub primary: Option<Emotion>,
    levels: Vec<(Emotion, CType)>,
}

impl EmotionState {
    pub fn get_emotion_level(&self, emotion: Emotion) -> CType {
        for (my_emotion, level) in &self.levels {
            if *my_emotion == emotion {
                return *level;
            }
        }
        return 0;
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct BardBoard {
    pub emotion_state: EmotionState,
    pub runeband_state: RunebandState,
    pub globes_state: GlobesState,
    pub blades_count: usize,
    pub needle_venom: Option<String>,
}

impl BardBoard {
    pub fn needle_with(&mut self, venom: &String) {
        self.needle_venom = Some(venom.clone());
    }

    pub fn needled(&mut self) -> Option<String> {
        let needled = self.needle_venom.clone();
        self.needle_venom = None;
        needled
    }

    pub fn globed(&mut self, affs: &[FType]) -> Option<FType> {
        match self.globes_state {
            GlobesState::Floating(count) => {
                if count > 1 {
                    self.globes_state = GlobesState::Floating(count - 1);
                } else {
                    self.globes_state = GlobesState::None;
                }
                Some(affs[affs.len() - count])
            }
            _ => None,
        }
    }

    pub fn runebanded(&mut self, affs: &[FType]) -> Option<FType> {
        match self.runeband_state {
            RunebandState::Normal(aff_idx) => {
                let new_idx = if aff_idx >= affs.len() - 1 {
                    0
                } else {
                    aff_idx + 1
                };
                self.runeband_state = RunebandState::Normal(new_idx);
                Some(affs[aff_idx])
            }
            RunebandState::Reverse(aff_idx) => {
                let new_idx = if aff_idx == 0 {
                    affs.len() - 1
                } else {
                    aff_idx - 1
                };
                self.runeband_state = RunebandState::Reverse(new_idx);
                Some(affs[aff_idx])
            }
            _ => None,
        }
    }
}
