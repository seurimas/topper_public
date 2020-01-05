use num_enum::TryFromPrimitive;
use serde::Deserialize;
use std::convert::TryFrom;
use std::fmt;
pub type CType = i32;

pub const BALANCE_SCALE: f32 = 100.0;

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

    // Timers
    Hypnosis,
    Fangbarrier,
    Rebounding,

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
    Sips,
    Shields,

    SIZE,
}

// Flags
#[derive(
    Debug, PartialEq, PartialOrd, Eq, Hash, Clone, Copy, TryFromPrimitive, Deserialize, EnumString,
)]
#[repr(u16)]
pub enum FType {
    Dead,

    // Control
    Player,
    Ally,
    Enemy,
    Hypnotized,
    Snapped,

    // Defences
    Shield,
    Deathsight,
    Energetic,
    Insomnia,
    Deafness,
    Blindness,
    Thirdeye,
    Daydreams,
    HardenedSkin,
    Waterbreathing,
    // Reishi
    Rebounding,
    // Elixirs
    Levitation,
    Antivenin,
    Speed,
    Frost,
    Vigor,
    // Salves
    Insulation,
    Density,

    // Antipsychotic
    Sadness,
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
    Allergies,
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
    Patterns,
    Shaderot,

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
    Hypothermia,

    // Mending Head
    CritBruiseHead,
    DestroyedThroat,
    CrippledThroat,
    ModBruiseHead,
    BruiseHead,

    // Mending Torso
    CritBruiseTorso,
    LightWound,
    Ablaze,
    CrackedRibs,
    ModBruiseTorso,
    BruiseTorso,

    // Mending Left Arm
    CritBruiseLeftArm,
    LeftArmBroken,
    ModBruiseLeftArm,
    BruiseLeftArm,
    DislocatedLeftArm,

    // Mending Left Arm
    CritBruiseRightArm,
    RightArmBroken,
    ModBruiseRightArm,
    BruiseRightArm,
    DislocatedRightArm,

    // Mending Left Arm
    CritBruiseLeftLeg,
    LeftLegBroken,
    ModBruiseLeftLeg,
    BruiseLeftLeg,
    DislocatedLeftLeg,

    // Mending Left Arm
    CritBruiseRightLeg,
    RightLegBroken,
    ModBruiseRightLeg,
    BruiseRightLeg,
    DislocatedRightLeg,

    // Soothing
    Whiplash,   // Head
    Backstrain, // Torso
    MuscleSpasms,
    Stiffness,
    SoreWrist, // Arms
    WeakGrip,
    // Whiplash // Legs

    // Caloric
    Frozen,
    Shivering,

    // Immunity
    Voyria,

    // Timed
    Blackout,
    Stun,
    Asleep,

    // Uncurable
    Void,
    Weakvoid,
    Backstabbed,

    SIZE,
}

impl FType {
    pub fn is_affliction(&self) -> bool {
        self >= &FType::Sadness
    }

    pub fn from_name(aff_name: &String) -> Option<FType> {
        let pretty = aff_name
            .split("_")
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
}

pub type StateRevert = Box<Fn(&mut AgentState, &mut AgentState)>;

pub type StateChange = Box<Fn(&mut AgentState, &mut AgentState) -> StateRevert>;

pub type StateMatcher = Box<Fn(&AgentState, &AgentState) -> bool>;

pub struct FlagSet([bool; FType::SIZE as usize]);

impl fmt::Debug for FlagSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let mut wrote = false;
        for idx in 0..self.0.len() {
            if self.0[idx] {
                if wrote {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}", FType::try_from(idx as u16))?;
                wrote = true;
            }
        }
        write!(f, "]")
    }
}

impl fmt::Display for FlagSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut wrote = false;
        for idx in 0..self.0.len() {
            if self.0[idx] {
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
        Ok(())
    }
}

pub struct FlagSetIterator<'s> {
    index: usize,
    set: &'s FlagSet,
}

impl<'s> Iterator for FlagSetIterator<'s> {
    type Item = FType;
    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.set.0.len() && !self.set.0[self.index] {
            self.index += 1;
        }
        if self.index < self.set.0.len() {
            self.index += 1;
            Some(FType::try_from((self.index - 1) as u16).unwrap())
        } else {
            None
        }
    }
}

impl<'s> FlagSetIterator<'s> {
    fn new(flagset: &'s FlagSet, affs: bool) -> Self {
        FlagSetIterator {
            index: if affs { FType::Sadness as usize } else { 0 },
            set: flagset,
        }
    }
}

impl FlagSet {
    pub fn aff_iter<'s>(&'s self) -> FlagSetIterator<'s> {
        FlagSetIterator::new(self, true)
    }
}

impl Default for FlagSet {
    fn default() -> Self {
        FlagSet([false; FType::SIZE as usize])
    }
}

impl Clone for FlagSet {
    fn clone(&self) -> Self {
        let flags = self.0;
        FlagSet(flags)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Hypnosis {
    Aff(FType),
    Action(String),
}

#[derive(Debug, Clone, Default)]
pub struct AgentState {
    pub balances: [CType; BType::SIZE as usize],
    pub stats: [CType; SType::SIZE as usize],
    pub max_stats: [CType; SType::SIZE as usize],
    pub flags: FlagSet,
    pub hypnosis_stack: Vec<Hypnosis>,
    pub relapses: Vec<String>,
}

impl PartialEq for AgentState {
    fn eq(&self, other: &Self) -> bool {
        let mut different = false;
        for i in 0..self.balances.len() {
            if self.balances[i] != other.balances[i] {
                different = true;
                break;
            }
        }
        different
    }
}

impl AgentState {
    pub fn wait(&mut self, duration: i32) {
        for i in 0..self.balances.len() {
            self.balances[i] -= duration;
        }
    }

    pub fn is(&self, flag: FType) -> bool {
        self.flags.0[flag as usize]
    }

    pub fn set_flag(&mut self, flag: FType, value: bool) {
        self.flags.0[flag as usize] = value;
    }

    pub fn get_flag(&self, flag: FType) -> bool {
        self.flags.0[flag as usize]
    }

    pub fn affliction_count(&self) -> i32 {
        let mut count = 0;
        for i in 0..(FType::SIZE as usize) {
            if i >= FType::Sadness as usize && i < FType::SIZE as usize {
                count += 1;
            }
        }
        count
    }

    pub fn set_balance(&mut self, balance: BType, value: f32) {
        self.balances[balance as usize] = (value * BALANCE_SCALE) as CType;
    }

    pub fn balanced(&self, balance: BType) -> bool {
        self.balances[balance as usize] <= 0
    }

    pub fn set_stat(&mut self, stat: SType, value: CType) {
        self.stats[stat as usize] = value;
    }

    pub fn get_stat(&self, stat: SType) -> CType {
        self.stats[stat as usize]
    }

    pub fn initialize_stat(&mut self, stat: SType, value: CType) {
        self.max_stats[stat as usize] = value;
        self.stats[stat as usize] = value;
    }

    pub fn can_smoke(&self) -> bool {
        !self.is(FType::Asthma) && self.balanced(BType::Smoke)
    }

    pub fn can_pill(&self) -> bool {
        !self.is(FType::Anorexia) && self.balanced(BType::Pill)
    }

    pub fn can_salve(&self) -> bool {
        !self.is(FType::Slickness) && self.balanced(BType::Salve)
    }

    pub fn can_tree(&self) -> bool {
        !self.is(FType::Paresis)
            && !self.is(FType::Paralysis)
            && !(self.is(FType::LeftArmBroken) && self.is(FType::RightArmBroken))
            && self.balanced(BType::Tree)
    }

    pub fn can_focus(&self) -> bool {
        !self.is(FType::Impatience) && self.balanced(BType::Focus)
    }

    pub fn push_toxin(&mut self, venom: String) {
        self.relapses.push(venom);
    }

    pub fn relapse(&mut self) -> Option<String> {
        if let Some(aff) = self.relapses.first() {
            Some(self.relapses.remove(0))
        } else {
            None
        }
    }

    pub fn clear_relapses(&mut self) {
        self.relapses = Vec::new();
    }
}

pub fn target(matcher: StateMatcher) -> StateMatcher {
    Box::new(move |_me, them| matcher(them, _me))
}

pub fn has(balance: BType) -> StateMatcher {
    Box::new(move |me, _them| me.balances[balance as usize] <= 0)
}

pub fn is(flag: FType) -> StateMatcher {
    Box::new(move |me, _them| me.is(flag))
}

pub fn lacks(flag: FType) -> StateMatcher {
    Box::new(move |me, _them| !me.is(flag))
}
pub fn lacks_some(afflictions: Vec<FType>) -> StateMatcher {
    Box::new(move |me, _them| {
        for affliction in afflictions.iter() {
            if !me.is(*affliction) {
                return true;
            }
        }
        return false;
    })
}

pub fn some(afflictions: Vec<FType>) -> StateMatcher {
    Box::new(move |me, _them| {
        for affliction in afflictions.iter() {
            if me.is(*affliction) {
                return true;
            }
        }
        return false;
    })
}

pub fn alive() -> StateMatcher {
    lacks(FType::Dead)
}
