use std::collections::HashMap;

use serde::*;

use super::*;

pub const GLOBE_AFFS: [FType; 3] = [FType::Dizziness, FType::Confusion, FType::Perplexed];
pub const RUNEBAND_AFFS: [FType; 7] = [
    FType::Stupidity,
    FType::Paranoia,
    FType::RingingEars,
    FType::Loneliness,
    FType::Exhausted,
    FType::Laxity,
    FType::Clumsiness,
];

#[derive(Debug, Deserialize, Serialize, Display, Clone, Copy, EnumString, PartialEq, Eq, Hash)]
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

    pub fn get_time_to_active(&self) -> Option<CType> {
        match self {
            HalfbeatState::WholeBeat(time) => Some(*time),
            _ => None,
        }
    }

    pub fn get_time_to_inactive(&self) -> Option<CType> {
        match self {
            HalfbeatState::HalfBeat(time) => Some(*time),
            _ => None,
        }
    }

    pub fn wait(&mut self, time: CType) {
        match self {
            HalfbeatState::HalfBeat(timer) | HalfbeatState::WholeBeat(timer) => {
                *timer -= time;
            }
            _ => {}
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ThuribleState {
    #[default]
    Inactive,
    Missing,
    InHand,
    InRoom(Timer),
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct BardClassState {
    pub dithering: usize,
    pub tempo: Option<(usize, CType)>,
    pub voice_song: Option<Song>,
    pub instrument_song: Option<Song>,
    pub voice_timeout: CType,
    pub instrument_timeout: CType,
    pub impetus_timer: CType,
    pub half_beat: HalfbeatState,
    pub anelaces: usize,
    pub thurible_location: ThuribleState,
    pub induce_timer: Timer,
}

const SONG_FUDGE: CType = (0.33 * BALANCE_SCALE) as CType;
const RUNEBAND_DELAY: CType = (8.0 * BALANCE_SCALE) as CType;
const RUNEBAND_TIMEOUT: CType = (10.0 * BALANCE_SCALE) as CType;
const IMPETUS_TIMEOUT: CType = (30.0 * BALANCE_SCALE) as CType;
const VOICE_SONG_TIMEOUT: CType = (8.0 * BALANCE_SCALE) as CType;
const INSTRUMENT_SONG_TIMEOUT: CType = (6.0 * BALANCE_SCALE) as CType;
const NEEDLE_TIMEOUT: CType = (3.25 * BALANCE_SCALE) as CType;
const HALFBEAT_TIMEOUT: CType = (20.0 * BALANCE_SCALE) as CType;

impl BardClassState {
    pub fn wait(&mut self, duration: i32) {
        let mut tempo_timeout = false;
        if let Some((count, timer)) = &mut self.tempo {
            *timer += duration;
            tempo_timeout = *timer > (2.0 * BALANCE_SCALE) as CType;
        }
        if tempo_timeout {
            self.tempo = None;
        }
        self.voice_timeout -= duration;
        self.instrument_timeout -= duration;
        if self.voice_timeout <= -SONG_FUDGE {
            self.voice_song = None;
        }
        if self.instrument_timeout <= -SONG_FUDGE {
            self.instrument_song = None;
        }
        self.induce_timer.wait(duration);
        self.half_beat.wait(duration);
    }

    pub fn on_tempo(&mut self, count: usize) {
        if count > 3 {
            self.tempo = None;
        } else {
            self.tempo = Some((count, 0));
        }
    }

    pub fn off_tempo(&mut self) {
        self.tempo = None;
    }

    pub fn is_on_tempo(&self) -> bool {
        self.tempo.is_some()
    }

    pub fn half_beat_pickup(&mut self) {
        self.half_beat = HalfbeatState::HalfBeat(HALFBEAT_TIMEOUT);
    }

    pub fn half_beat_slowdown(&mut self) {
        self.half_beat = HalfbeatState::WholeBeat(HALFBEAT_TIMEOUT);
    }

    pub fn half_beat_end(&mut self) {
        self.half_beat = HalfbeatState::Inactive;
    }

    pub fn start_song(&mut self, song: Song, played: bool) {
        if played {
            self.instrument_song = Some(song);
            self.instrument_timeout = INSTRUMENT_SONG_TIMEOUT;
        } else {
            self.voice_song = Some(song);
            self.voice_timeout = VOICE_SONG_TIMEOUT;
        }
    }

    pub fn end_song(&mut self, song: Song) {
        if self.voice_song == Some(song) {
            self.voice_song = None;
        } else if self.instrument_song == Some(song) {
            self.instrument_song = None;
        }
    }

    pub fn begin_impetus(&mut self) {
        self.impetus_timer = IMPETUS_TIMEOUT;
    }

    pub fn impetus_ready(&self) -> bool {
        self.impetus_timer > 0
    }

    pub fn set_induce_timer(&mut self, time: f32) {
        self.induce_timer = Timer::count_down_seconds(time);
    }

    pub fn get_induce_time_left(&self) -> CType {
        self.induce_timer.get_time_left()
    }

    pub fn induce_ready(&self) -> bool {
        !self.induce_timer.is_active()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RunebandState {
    Inactive,
    Normal(usize, Timer),
    Reverse(usize, Timer),
}

impl Default for RunebandState {
    fn default() -> Self {
        Self::Inactive
    }
}

impl RunebandState {
    pub fn wait(&mut self, time: CType) {
        match self {
            Self::Inactive => {}
            Self::Reverse(_, timer) | Self::Normal(_, timer) => {
                timer.wait(time);
            }
        };
    }

    pub fn initial() -> Self {
        Self::Normal(0, Timer::count_up_observe(RUNEBAND_DELAY, RUNEBAND_TIMEOUT))
    }

    pub fn reverse(&mut self) {
        match &self {
            Self::Normal(next, timer) => {
                *self = Self::Reverse(*next, *timer);
            }
            Self::Reverse(next, timer) => {
                *self = Self::Normal(*next, *timer);
            }
            _ => {}
        }
    }

    pub fn is_active(&self) -> bool {
        match self {
            Self::Inactive => false,
            Self::Normal(_, timer) | Self::Reverse(_, timer) => timer.is_active(),
        }
    }

    pub fn is_forward(&self) -> bool {
        match self {
            Self::Normal(_, _) => true,
            _ => false,
        }
    }

    pub fn is_reversed(&self) -> bool {
        match self {
            Self::Reverse(_, _) => true,
            _ => false,
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

    pub fn is_full(&self) -> bool {
        match self {
            Self::Floating(count) => *count == 3,
            _ => false,
        }
    }
}

pub const FATE_TIMEOUT: CType = (45.0 * BALANCE_SCALE) as CType;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FateState {
    Inactive,
    Active {
        timeout: CType,
        known_exits: Vec<String>,
    },
}

impl FateState {
    fn wait(&mut self, time: CType) {
        match self {
            Self::Active { timeout, .. } => {
                *timeout -= time;
                if *timeout <= 0 {
                    *self = Self::Inactive
                }
            }
            _ => {}
        }
    }

    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active { .. })
    }

    pub fn activate(&mut self) {
        *self = Self::Active {
            timeout: FATE_TIMEOUT,
            known_exits: Vec::new(),
        }
    }

    pub fn add_exit(&mut self, exit: String) {
        match self {
            Self::Active {
                timeout,
                known_exits,
            } => {
                *timeout = FATE_TIMEOUT;
                known_exits.push(exit);
            }
            Self::Inactive => {
                *self = Self::Active {
                    timeout: FATE_TIMEOUT,
                    known_exits: vec![exit],
                }
            }
        }
    }

    pub fn deactivate(&mut self) {
        *self = Self::Inactive
    }
}

impl Default for FateState {
    fn default() -> Self {
        Self::Inactive
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IronCollarState {
    None,
    Locking,
    Locked,
}

impl Default for IronCollarState {
    fn default() -> Self {
        Self::None
    }
}

impl IronCollarState {
    pub fn is_active(&self) -> bool {
        match self {
            Self::None => false,
            _ => true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, EnumString, Copy, PartialEq, Eq, Hash)]
pub enum Emotion {
    Sadness,
    Happiness,
    Surprise,
    Anger,
    Stress,
    Fear,
    Disgust,
}

impl Emotion {
    pub fn try_from_name(name: &str) -> Option<Self> {
        match name {
            "sadness" | "sad" => Some(Emotion::Sadness),
            "happiness" | "happy" => Some(Emotion::Happiness),
            "surprise" | "surprised" => Some(Emotion::Surprise),
            "anger" | "angry" => Some(Emotion::Anger),
            "stress" | "stressed" => Some(Emotion::Stress),
            "fear" | "fearful" => Some(Emotion::Fear),
            "disgust" => Some(Emotion::Disgust),
            _ => None,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Emotion::Sadness => "sadness",
            Emotion::Happiness => "happiness",
            Emotion::Surprise => "surprise",
            Emotion::Anger => "anger",
            Emotion::Stress => "stress",
            Emotion::Fear => "fear",
            Emotion::Disgust => "disgust",
        }
    }

    pub fn get_aff(&self) -> FType {
        match self {
            Emotion::Sadness => FType::Shyness,
            Emotion::Happiness => FType::Perplexed,
            Emotion::Surprise => FType::Dizziness,
            Emotion::Anger => FType::Hatred,
            Emotion::Stress => FType::Masochism,
            Emotion::Fear => FType::SelfLoathing,
            Emotion::Disgust => FType::Besilence,
        }
    }
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

    pub fn get_emotion_over(&self, level: CType) -> Option<Emotion> {
        let mut max_emotion_and_level = None;
        for (my_emotion, my_level) in &self.levels {
            if *my_level > level {
                if let Some((max_emotion, max_level)) = max_emotion_and_level {
                    if *my_level > max_level {
                        max_emotion_and_level = Some((*my_emotion, *my_level));
                    }
                } else {
                    max_emotion_and_level = Some((*my_emotion, *my_level));
                }
            }
        }
        max_emotion_and_level.map(|(emotion, _)| emotion)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct BardBoard {
    pub emotion_state: EmotionState,
    pub runeband_state: RunebandState,
    pub globes_state: GlobesState,
    pub fate_state: FateState,
    pub iron_collar_state: IronCollarState,
    pub blades_count: usize,
    pub needle_venom: Option<String>,
    // Not a timer, because we have some rather specific logic.
    pub needle_timer: CType,
    pub dumb: Option<bool>,
}

impl BardBoard {
    pub fn wait(&mut self, duration: i32) {
        self.fate_state.wait(duration);
        self.runeband_state.wait(duration);
        self.needle_timer -= duration;
        if self.needle_timer < -100 {
            self.needle_venom = None;
        }
    }

    pub fn needle_with(&mut self, venom: &String) {
        self.needle_venom = Some(venom.clone());
        self.needle_timer = NEEDLE_TIMEOUT;
    }

    pub fn needled(&mut self) -> Option<String> {
        println!("Needled at {}", self.needle_timer);
        let needled = self.needle_venom.clone();
        self.needle_venom = None;
        self.needle_timer = 0;
        needled
    }

    pub fn is_needled(&self) -> bool {
        self.needle_venom.is_some()
    }

    pub fn needling(&self) -> bool {
        self.needle_venom.is_some() && self.needle_timer <= 0 as CType
    }

    pub fn almost_needling(&self, time: f32) -> bool {
        self.needle_venom.is_some() && self.needle_timer <= (time * BALANCE_SCALE) as CType
    }

    pub fn next_globe(&self) -> Option<FType> {
        match self.globes_state {
            GlobesState::Floating(count) => Some(GLOBE_AFFS[GLOBE_AFFS.len() - count]),
            _ => None,
        }
    }

    pub fn globed(&mut self) -> Option<FType> {
        match self.globes_state {
            GlobesState::Floating(count) => {
                if count > 1 {
                    self.globes_state = GlobesState::Floating(count - 1);
                } else {
                    self.globes_state = GlobesState::None;
                }
                Some(GLOBE_AFFS[GLOBE_AFFS.len() - count])
            }
            _ => None,
        }
    }

    pub fn next_runeband(&self) -> Option<(FType, Timer)> {
        match self.runeband_state {
            RunebandState::Normal(aff_idx, timer) => Some((RUNEBAND_AFFS[aff_idx], timer)),
            RunebandState::Reverse(aff_idx, timer) => Some((RUNEBAND_AFFS[aff_idx], timer)),
            _ => None,
        }
    }

    pub fn runebanded(&mut self) -> Option<FType> {
        match self.runeband_state {
            RunebandState::Normal(aff_idx, _) => {
                let new_idx = if aff_idx >= RUNEBAND_AFFS.len() - 1 {
                    0
                } else {
                    aff_idx + 1
                };
                self.runeband_state = RunebandState::Normal(
                    new_idx,
                    Timer::count_up_observe(RUNEBAND_DELAY, RUNEBAND_TIMEOUT),
                );
                Some(RUNEBAND_AFFS[aff_idx])
            }
            RunebandState::Reverse(aff_idx, _) => {
                let new_idx = if aff_idx == 0 {
                    RUNEBAND_AFFS.len() - 1
                } else {
                    aff_idx - 1
                };
                self.runeband_state = RunebandState::Reverse(
                    new_idx,
                    Timer::count_up_observe(RUNEBAND_DELAY, RUNEBAND_TIMEOUT),
                );
                Some(RUNEBAND_AFFS[aff_idx])
            }
            _ => None,
        }
    }

    pub fn runeband_timer(&self) -> Option<Timer> {
        match self.runeband_state {
            RunebandState::Normal(_, timer) | RunebandState::Reverse(_, timer) => Some(timer),
            _ => None,
        }
    }

    pub fn awaken(&mut self) {
        self.emotion_state.awakened = true;
        self.emotion_state.levels = vec![];
        self.emotion_state.primary = None;
    }

    pub fn induce(&mut self, primary: Emotion) {
        self.emotion_state.primary = Some(primary);
    }

    pub fn set_emotions(&mut self, primary: Option<Emotion>, levels: &Vec<(Emotion, CType)>) {
        self.emotion_state.awakened = levels.len() > 0;
        self.emotion_state.levels = levels.clone();
        self.emotion_state.primary = primary;
    }

    pub fn dumbness_known(&self) -> bool {
        self.dumb.is_some()
    }

    pub fn is_dumb(&self, default: bool) -> bool {
        self.dumb.unwrap_or(default)
    }

    pub fn observe_dumbness(&mut self, dumb: bool) {
        self.dumb = Some(dumb);
    }
}
