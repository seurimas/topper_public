use num_enum::TryFromPrimitive;
use std::fmt;
pub type CType = i32;

pub const BALANCE_SCALE: f32 = 100.0;

// Balances
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, TryFromPrimitive)]
#[repr(usize)]
pub enum BType {
    // Actions
    Balance,
    Equil,

    // Curatives
    Elixir,
    Pill,
    Salve,

    SIZE,
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
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, TryFromPrimitive)]
#[repr(u16)]
pub enum FType {
    Dead,
    Shield,
    Player,

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
    Insomnia,

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

    // Defences
    Deathsight,
    Energetic,
    // Insomnia,
    Deafness,
    Blindness,
    Thirdeye,
    Daydreams,
    HardenedSkin,
    Waterbreathing,

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

    // Reishi
    Rebounding,

    // Elixirs
    Levitation,
    Antivenin,
    Speed,
    Frost,
    Vigor,

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
    BrokenLeftArm,
    ModBruiseLeftArm,
    BruiseLeftArm,
    DislocatedLeftArm,

    // Mending Left Arm
    CritBruiseRightArm,
    BrokenRightArm,
    ModBruiseRightArm,
    BruiseRightArm,
    DislocatedRightArm,

    // Mending Left Arm
    CritBruiseLeftLeg,
    BrokenLeftLeg,
    ModBruiseLeftLeg,
    BruiseLeftLeg,
    DislocatedLeftLeg,

    // Mending Left Arm
    CritBruiseRightLeg,
    BrokenRightLeg,
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

    SIZE,
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
                write!(f, "{:?}", FType::Addiction)?;
                wrote = true;
            }
        }
        write!(f, "]")
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

#[derive(Debug, Clone, Default)]
pub struct AgentState {
    pub balances: [CType; BType::SIZE as usize],
    pub stats: [CType; SType::SIZE as usize],
    pub max_stats: [CType; SType::SIZE as usize],
    pub flags: FlagSet,
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

    pub fn set_balance(&mut self, balance: BType, value: CType) {
        self.balances[balance as usize] = value;
    }

    pub fn set_stat(&mut self, stat: SType, value: CType) {
        self.stats[stat as usize] = value;
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
