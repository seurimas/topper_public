use super::*;
use crate::timeline::BaseAgentState;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

// Balances
#[derive(Deserialize, Debug, PartialEq, Eq, Hash, Clone, Copy, TryFromPrimitive)]
#[repr(usize)]
pub enum BType {
    // Actions
    Balance,
    Equil,
    Secondary,

    // Curatives
    Elixir,
    Pill,
    Salve,
    Smoke,
    Focus,
    Tree,
    Renew,
    Regenerate,

    // Misc
    ClassCure1,
    ClassCure2, // Fitness

    // Cooldowns
    Wrath,
    Firefist,
    Pendulum,
    Disable,
    Disabled,

    // Timers
    Hypnosis,
    Fangbarrier,
    Rebounding,
    Void,
    ParesisParalysis,

    UNKNOWN,
    SIZE,
}

impl BType {
    pub fn from_name(bal_name: &String) -> Self {
        match bal_name.as_str() {
            "Balance" => BType::Balance,
            "Equilibrium" => BType::Equil,
            "Shadow" => BType::Secondary,
            _ => BType::UNKNOWN,
        }
    }
}

// Stats
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum SType {
    Health,
    Mana,
    SP,
    Sips,
    Shields,

    SIZE,
}

// Flags
#[derive(
    Debug,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Clone,
    Copy,
    TryFromPrimitive,
    EnumString,
    Serialize,
    Deserialize,
    Display,
)]
#[repr(u16)]
pub enum FType {
    Dead,

    // Control
    Player,
    Ally,
    Enemy,

    // Defences
    Shielded,
    Deathsight,
    Insomnia,
    Instawake,
    Deafness,
    Blindness,
    Thirdeye,
    Daydreams,
    Fangbarrier,
    Waterbreathing,
    Waterwalking,
    // Reishi
    Rebounding,
    AssumedRebounding,
    // Elixirs
    Levitation,
    VenomResistance,
    Speed,
    Temperance,
    Vigor,
    // Salves
    Insulation,
    Density,
    // Tattoos
    Flame,
    Cloak,
    // General
    Reflection,

    // Syssin defences
    Shroud,
    Ghosted,
    Shadowslip,
    Weaving,
    Hiding,
    Shadowsight,

    // Zealot defences
    Mindspark,
    Zenith,
    Firefist,
    Swagger,
    Wrath,

    // Antipsychotic
    Sadness, // MUST BE FIRST AFFLICTION
    Confusion,
    Dementia,
    Hallucinations,
    Paranoia,
    Hatred,
    Addiction,
    Hypersomnia,
    BloodCurse,
    Blighted,

    // Euphoriant
    SelfPity,
    Stupidity,
    Dizziness,
    Faintness,
    Shyness,
    Epilepsy,
    Impatience,
    Dissonance,
    Infested,
    // Insomnia,

    // Decongestant
    Baldness,
    Clumsiness,
    Hypochondria,
    Weariness,
    Asthma,
    Sensitivity,
    RingingEars,
    Impairment,
    BloodPoison,

    // Depressant
    CommitmentFear,
    Merciful,
    Recklessness,
    Egocentric,
    Masochism,
    Agoraphobia,
    Loneliness,
    Berserking,
    Vertigo,
    Claustrophobia,
    Nyctophobia,

    // Coagulation
    BodyOdor,
    Lethargy,
    MentalDisruption,
    PhysicalDisruption,
    Vomiting,
    Exhausted,
    ThinBlood,
    Rend,
    Haemophilia,

    // Steroid
    Hubris,
    Pacifism,
    Peace,
    Soulburn,
    Soulfire,
    LimpVeins,
    LoversEffect,
    Laxity,
    Superstition,
    Generosity,
    Justice,
    Magnanimity,

    // Opiate
    Paresis,
    Paralysis,
    Mirroring,
    CrippledBody,
    Crippled,
    Blisters,
    Slickness,
    Heartflutter,
    Sandrot,

    // Anabiotic
    Plodding,
    Idiocy,

    // Panacea
    Stormtouched,
    Patterns,
    Shaderot,
    ShaderotBenign,
    ShaderotSpirit,
    ShaderotHeat,
    ShaderotWither,
    ShaderotBody,

    // Willow
    Aeon,
    Hellsight,
    Deadening,

    // Yarrow
    // Slickness,
    Withering,
    Disfigurement,
    Migraine,
    Squelched,

    // Epidermal Head
    Indifference,
    Stuttering,
    BlurryVision,
    BurntEyes,
    // Blindness,
    Gloom,
    // Deafness,

    // Epidermal Toros
    Anorexia,
    Gorged,
    EffusedBlood,

    // Mending Head
    HeadBruisedCritical,
    DestroyedThroat,
    CrippledThroat,
    HeadBruisedModerate,
    HeadBruised,

    // Mending Torso
    TorsoBruisedCritical,
    Lightwound,
    CrackedRibs,
    TorsoBruisedModerate,
    TorsoBruised,

    // Mending Left Arm
    LeftArmBruisedCritical,
    LeftArmBruisedModerate,
    LeftArmBruised,
    LeftArmDislocated,

    // Mending Right Arm
    RightArmBruisedCritical,
    RightArmBruisedModerate,
    RightArmBruised,
    RightArmDislocated,

    // Mending Left Leg
    LeftLegBruisedCritical,
    LeftLegBruisedModerate,
    LeftLegBruised,
    LeftLegDislocated,

    // Mending Right Leg
    RightLegBruisedCritical,
    RightLegBruisedModerate,
    RightLegBruised,
    RightLegDislocated,

    // Restoration Head
    Voidgaze,
    Voidtrapped,
    MauledFace,
    SmashedThroat,

    // Restoration Torso
    CollapsedLung,
    SpinalRip,
    BurntSkin,
    CrushedChest,
    Heatspear,
    Deepwound,

    // Soothing
    Whiplash,   // Head
    Backstrain, // Torso
    MuscleSpasms,
    Stiffness,
    SoreWrist, // Arms
    WeakGrip,
    SoreAnkle, // Legs

    // Caloric
    Hypothermia,
    IceEncased,
    Frozen,
    Shivering,

    // Immunity
    Voyria,

    // Timed
    Blackout,
    Stun,
    Asleep,

    // Monk Uncurable
    NumbArms,

    // Syssin Uncurable
    Void,
    Weakvoid,
    Backstabbed,
    NumbedSkin,
    MentalFatigue,
    Thorns,

    // Zealot Uncurable
    InfernalSeal,
    InfernalShroud,

    // Scio Uncurable
    Imbued,
    Impeded,
    Shadowbrand,
    Shadowsphere,

    // Special
    Disrupted,
    Fear,
    Fallen,
    Itchy,

    // Writhes
    WritheArmpitlock,
    WritheBind,
    WritheGrappled,
    WritheGunk,
    WritheHoist,
    WritheImpaled,
    WritheLure,
    WritheNecklock,
    WritheRopes,
    WritheStasis,
    WritheThighlock,
    WritheTransfix,
    WritheVines,
    WritheWeb,

    SIZE,
    // Afflictions that stack.
    Allergies,
    Ablaze,
    SappedStrength,
    FULL,
    // Afflictions stored elsewhere
    LeftLegBroken,
    RightLegBroken,
    LeftArmBroken,
    RightArmBroken,
}

impl FType {
    pub fn is_affliction(&self) -> bool {
        self >= &FType::Sadness
    }

    pub fn from_name(aff_name: &String) -> Option<FType> {
        let pretty = aff_name
            .split(|c| c == ' ' || c == '_' || c == '-')
            .map(|word| {
                let mut c = word.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                }
            })
            .collect::<String>();
        let result: Option<FType> = pretty.parse().ok();
        result
    }

    pub fn to_name(&self) -> String {
        let mut words = vec![];
        let mut word = String::from("");
        self.to_string().chars().for_each(|letter| {
            if word.len() == 0 {
                word.push_str(&letter.to_lowercase().to_string());
            } else if letter.is_uppercase() {
                words.push(word.clone());
                word = String::from("");
            } else {
                word.push_str(&letter.to_string());
            }
        });
        words.push(word.clone());
        words.join("_")
    }

    pub fn try_from_counter_idx(
        idx: usize,
    ) -> Result<FType, num_enum::TryFromPrimitiveError<FType>> {
        FType::try_from(FType::SIZE as u16 + 1 + idx as u16)
    }

    pub fn is_counter(&self) -> bool {
        self > &FType::SIZE && self < &FType::FULL
    }
}

const COUNTERS_SIZE: usize = FType::FULL as usize - FType::SIZE as usize - 1;

#[derive(PartialEq, Eq, Hash)]
pub struct FlagSet {
    simple: [bool; FType::SIZE as usize],
    counters: [u8; COUNTERS_SIZE],
}

impl FlagSet {
    pub fn is_flag_set(&self, flag: FType) -> bool {
        if flag.is_counter() {
            self.counters[flag as usize - FType::SIZE as usize - 1] > 0
        } else {
            self.simple[flag as usize]
        }
    }

    pub fn get_flag_count(&self, flag: FType) -> u8 {
        if flag.is_counter() {
            self.counters[flag as usize - FType::SIZE as usize - 1]
        } else {
            if self.simple[flag as usize] {
                1
            } else {
                0
            }
        }
    }

    pub fn set_flag(&mut self, flag: FType, value: bool) {
        if flag.is_counter() {
            let counter_idx = flag as usize - FType::SIZE as usize - 1;
            let old_value = self.counters[counter_idx as usize];
            if value && old_value < 1 {
                self.counters[counter_idx] = 1;
            } else if !value && old_value > 0 {
                self.counters[counter_idx] = 0;
            }
        } else {
            self.simple[flag as usize] = value;
        }
    }

    pub fn set_flag_count(&mut self, flag: FType, value: u8) {
        if flag.is_counter() {
            let counter_idx = flag as usize - FType::SIZE as usize - 1;
            self.counters[counter_idx] = value;
        } else {
            self.simple[flag as usize] = value > 0;
        }
    }

    pub fn tick_counter_up(&mut self, flag: FType) {
        if flag.is_counter() {
            self.counters[flag as usize - FType::SIZE as usize - 1] += 1;
        } else {
            println!("Tried to tick up non-counter.");
        }
    }
}

impl fmt::Debug for FlagSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let mut wrote = false;
        for idx in 0..self.simple.len() {
            if self.simple[idx] {
                if wrote {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}", FType::try_from(idx as u16))?;
                wrote = true;
            }
        }
        for idx in 0..self.counters.len() {
            if self.counters[idx] > 0 {
                if wrote {
                    write!(f, ", ")?;
                }
                write!(
                    f,
                    "{:?}x{}",
                    FType::try_from_counter_idx(idx),
                    self.counters[idx]
                )?;
                wrote = true;
            }
        }
        write!(f, "]")
    }
}

impl fmt::Display for FlagSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut wrote = false;
        for idx in 0..self.simple.len() {
            if self.simple[idx] {
                if let Ok(ftype) = FType::try_from(idx as u16) {
                    if ftype.is_affliction() {
                        if wrote {
                            write!(f, ", ")?;
                        }
                        write!(f, "{:?}", ftype)?;
                        wrote = true;
                    }
                }
            }
        }
        for idx in 0..self.counters.len() {
            if self.counters[idx] > 0 {
                if let Ok(ftype) = FType::try_from_counter_idx(idx) {
                    if ftype.is_affliction() {
                        if wrote {
                            write!(f, ", ")?;
                        }
                        write!(f, "{:?}x{}", ftype, self.counters[idx])?;
                        wrote = true;
                    }
                }
            }
        }
        Ok(())
    }
}

pub struct FlagSetIterator<'s> {
    index: usize,
    simple: bool,
    set: &'s FlagSet,
    predicate: &'s Fn(FType) -> bool,
}

impl<'s> FlagSetIterator<'s> {
    fn next_simple(&mut self) -> Option<FType> {
        while self.index < self.set.simple.len() && !self.set.simple[self.index] {
            self.index += 1;
        }
        if self.index < self.set.simple.len() {
            let ftype = FType::try_from(self.index as u16).unwrap();
            self.index += 1;
            if (self.predicate)(ftype) {
                Some(ftype)
            } else {
                self.next()
            }
        } else {
            None
        }
    }
    fn next_counter(&mut self) -> Option<FType> {
        while self.index < self.set.counters.len() && self.set.counters[self.index] == 0 {
            self.index += 1;
        }
        if self.index < self.set.counters.len() {
            let ftype = FType::try_from_counter_idx(self.index).unwrap();
            self.index += 1;
            if (self.predicate)(ftype) {
                Some(ftype)
            } else {
                self.next()
            }
        } else {
            None
        }
    }
}

impl<'s> Iterator for FlagSetIterator<'s> {
    type Item = FType;
    fn next(&mut self) -> Option<Self::Item> {
        if self.simple {
            if let Some(simple) = self.next_simple() {
                Some(simple)
            } else {
                self.simple = false;
                self.index = 0;
                self.next()
            }
        } else {
            self.next_counter()
        }
    }
}

impl<'s> FlagSetIterator<'s> {
    fn new(flagset: &'s FlagSet, predicate: &'s Fn(FType) -> bool) -> Self {
        FlagSetIterator {
            simple: true,
            index: 0,
            set: flagset,
            predicate,
        }
    }
}

impl FlagSet {
    pub fn aff_iter<'s>(&'s self) -> FlagSetIterator<'s> {
        FlagSetIterator::new(self, &|ftype: FType| ftype.is_affliction())
    }
}

impl Default for FlagSet {
    fn default() -> Self {
        FlagSet {
            simple: [false; FType::SIZE as usize],
            counters: [0; COUNTERS_SIZE],
        }
    }
}

impl Clone for FlagSet {
    fn clone(&self) -> Self {
        FlagSet {
            simple: self.simple,
            counters: self.counters,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WieldState {
    Normal {
        left: Option<String>,
        right: Option<String>,
    },
    TwoHanded(String),
}

impl Default for WieldState {
    fn default() -> Self {
        WieldState::Normal {
            left: None,
            right: None,
        }
    }
}

const SOFT_COOLDOWN: f32 = 2.0;
const HARD_COOLDOWN: f32 = 6.0;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DodgeState {
    Ready,
    Cooldown(CType),
}

impl Default for DodgeState {
    fn default() -> Self {
        DodgeState::Ready
    }
}

impl DodgeState {
    pub fn wait(&mut self, duration: CType) {
        match self {
            DodgeState::Ready => {}
            DodgeState::Cooldown(remaining) => {
                if *remaining > duration {
                    *self = DodgeState::Cooldown(*remaining - duration);
                } else {
                    *self = DodgeState::Ready
                }
            }
        }
    }
    pub fn register_hit(&mut self) {
        match self {
            DodgeState::Ready => {
                *self = DodgeState::Cooldown((SOFT_COOLDOWN * BALANCE_SCALE) as CType);
            }
            DodgeState::Cooldown(_) => {}
        }
    }
    pub fn register_dodge(&mut self) {
        *self = DodgeState::Cooldown((HARD_COOLDOWN * BALANCE_SCALE) as CType);
    }
    pub fn can_dodge(&self) -> bool {
        match self {
            DodgeState::Ready => true,
            _ => false,
        }
    }
    pub fn can_dodge_at(&self, qeb: f32) -> bool {
        match self {
            DodgeState::Ready => true,
            DodgeState::Cooldown(cooldown) => {
                if *cooldown < ((qeb * BALANCE_SCALE) as CType) {
                    true
                } else {
                    false
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClassState {
    Zealot(ZealotClassState),
    Sentinel(SentinelClassState),
    Shifter(HowlingState),
    Unknown,
}

impl ClassState {
    pub fn wait(&mut self, duration: CType) {
        match self {
            ClassState::Zealot(ZealotClassState { zenith, pyromania }) => {
                zenith.wait(duration);
                pyromania.wait(duration);
            }
            _ => {}
        }
    }
}

impl Default for ClassState {
    fn default() -> ClassState {
        ClassState::Unknown
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChannelState {
    Inactive,
    Heelrush(LType, CType),
}

impl ChannelState {
    pub fn wait(&mut self, duration: CType) {
        match self {
            ChannelState::Heelrush(_, remaining) => {
                if *remaining < duration {
                    *self = ChannelState::Inactive;
                } else {
                    *remaining = *remaining - duration;
                }
            }
            _ => {}
        }
    }
}

impl Default for ChannelState {
    fn default() -> ChannelState {
        ChannelState::Inactive
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimedFlagState {
    Inactive,
    Active(CType),
}

impl Default for TimedFlagState {
    fn default() -> Self {
        TimedFlagState::Inactive
    }
}

impl TimedFlagState {
    pub fn wait(&mut self, duration: CType) {
        match self.clone() {
            TimedFlagState::Inactive => {}
            TimedFlagState::Active(remaining) => {
                if remaining > duration {
                    *self = TimedFlagState::Active(remaining - duration);
                } else {
                    *self = TimedFlagState::Inactive;
                }
            }
        }
    }

    pub fn active(&self) -> bool {
        match self {
            TimedFlagState::Inactive => false,
            _ => true,
        }
    }

    pub fn activate(&mut self, duration: CType) {
        *self = TimedFlagState::Active(duration);
    }

    pub fn deactivate(&mut self) {
        *self = TimedFlagState::Inactive;
    }
}
