use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum LType {
    HeadDamage,
    TorsoDamage,
    LeftArmDamage,
    RightArmDamage,
    LeftLegDamage,
    RightLegDamage,

    SIZE,
}

impl LType {
    pub fn to_string(&self) -> String {
        match self {
            LType::HeadDamage => "head".to_string(),
            LType::TorsoDamage => "torso".to_string(),
            LType::LeftArmDamage => "left arm".to_string(),
            LType::RightArmDamage => "right arm".to_string(),
            LType::LeftLegDamage => "left leg".to_string(),
            LType::RightLegDamage => "right leg".to_string(),
            _ => "size".to_string(),
        }
    }

    pub fn broken(&self) -> Option<FType> {
        match self {
            LType::LeftArmDamage => Some(FType::LeftArmBroken),
            LType::RightArmDamage => Some(FType::RightArmBroken),
            LType::LeftLegDamage => Some(FType::LeftLegBroken),
            LType::RightLegDamage => Some(FType::RightLegBroken),
            _ => None,
        }
    }

    pub fn rotated(&self, counter: bool) -> Option<LType> {
        if counter {
            match self {
                LType::LeftArmDamage => Some(LType::LeftLegDamage),
                LType::RightArmDamage => Some(LType::LeftArmDamage),
                LType::LeftLegDamage => Some(LType::RightLegDamage),
                LType::RightLegDamage => Some(LType::RightArmDamage),
                _ => None,
            }
        } else {
            match self {
                LType::LeftArmDamage => Some(LType::RightArmDamage),
                LType::RightArmDamage => Some(LType::RightLegDamage),
                LType::LeftLegDamage => Some(LType::LeftArmDamage),
                LType::RightLegDamage => Some(LType::LeftLegDamage),
                _ => None,
            }
        }
    }
}

pub fn get_limb_damage(what: &String) -> Result<LType, String> {
    match what.to_ascii_lowercase().as_ref() {
        "head" => Ok(LType::HeadDamage),
        "torso" => Ok(LType::TorsoDamage),
        "left arm" => Ok(LType::LeftArmDamage),
        "right arm" => Ok(LType::RightArmDamage),
        "left leg" => Ok(LType::LeftLegDamage),
        "right leg" => Ok(LType::RightLegDamage),
        _ => Err(format!("Could not find damage for {}", what)),
    }
}

pub fn get_damage_limb(what: LType) -> Result<String, String> {
    match what {
        LType::HeadDamage => Ok("Head".to_string()),
        LType::TorsoDamage => Ok("Torso".to_string()),
        LType::LeftArmDamage => Ok("LeftArm".to_string()),
        LType::RightArmDamage => Ok("RightArm".to_string()),
        LType::LeftLegDamage => Ok("LeftLeg".to_string()),
        LType::RightLegDamage => Ok("RightLeg".to_string()),
        _ => Err(format!("SIZE? {:?}", what)),
    }
}

pub fn get_damage_barrier(aff: &String) -> Result<(LType, CType), String> {
    match aff.as_ref() {
        "head_mangled" => Ok((LType::HeadDamage, 666)),
        "head_damaged" => Ok((LType::HeadDamage, 333)),
        "torso_mangled" => Ok((LType::TorsoDamage, 666)),
        "torso_damaged" => Ok((LType::TorsoDamage, 333)),
        "left_arm_mangled" => Ok((LType::LeftArmDamage, 666)),
        "left_arm_damaged" => Ok((LType::LeftArmDamage, 333)),
        "right_arm_mangled" => Ok((LType::RightArmDamage, 666)),
        "right_arm_damaged" => Ok((LType::RightArmDamage, 333)),
        "left_leg_mangled" => Ok((LType::LeftLegDamage, 666)),
        "left_leg_damaged" => Ok((LType::LeftLegDamage, 333)),
        "right_leg_mangled" => Ok((LType::RightLegDamage, 666)),
        "right_leg_damaged" => Ok((LType::RightLegDamage, 333)),
        _ => Err(format!("Could not find damage for {}", aff)),
    }
}

// Flags
#[derive(
    Debug,
    PartialEq,
    PartialOrd,
    Eq,
    Hash,
    Clone,
    Copy,
    TryFromPrimitive,
    EnumString,
    Serialize,
    Deserialize,
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
    Hypothermia,

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
            .split(|c| c == '_' || c == '-')
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Hypnosis {
    Aff(FType),
    Action(String),
    Bulimia,
}

#[derive(Clone, Copy, Default)]
pub struct Limb {
    pub damage: CType,
    pub broken: bool,
    pub damaged: bool,
    pub mangled: bool,
    pub welt: bool,
}

#[derive(Clone, Default)]
pub struct LimbSet {
    pub limbs: [Limb; LType::SIZE as usize],
    pub restoring: Option<LType>,
    pub curing: Option<FType>,
    pub restore_timer: Option<CType>,
    pub regenerating: bool,
}

pub const DAMAGED_VALUE: CType = 3332;
pub const MANGLED_VALUE: CType = 6665;

#[derive(Clone, Debug, Serialize)]
pub struct LimbState {
    pub damage: f32,
    pub broken: bool,
    pub damaged: bool,
    pub mangled: bool,
    pub is_restoring: bool,
    pub is_parried: bool,
    pub is_dislocated: bool,
    pub welt: bool,
}

impl LimbState {
    pub fn restores_to_zero(&self) -> i32 {
        let mut damage = self.damage;
        if self.is_restoring {
            damage -= 30.0;
        }
        i32::max((damage / 30.0) as i32, 0)
    }
    pub fn hits_to_break(&self, damage: f32) -> i32 {
        let damaged_value = (DAMAGED_VALUE + 1) as f32 / 100.0;
        f32::ceil((damaged_value - self.damage) / damage) as i32
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct LimbsState {
    pub head: LimbState,
    pub torso: LimbState,
    pub left_arm: LimbState,
    pub right_arm: LimbState,
    pub left_leg: LimbState,
    pub right_leg: LimbState,
}

impl LimbsState {
    pub fn restores_to_zeroes(&self) -> i32 {
        self.head.restores_to_zero()
            + self.torso.restores_to_zero()
            + self.left_arm.restores_to_zero()
            + self.right_arm.restores_to_zero()
            + self.left_leg.restores_to_zero()
            + self.right_leg.restores_to_zero()
    }

    pub fn damages(&self) -> i32 {
        let mut acc = 0;
        if self.head.damaged {
            acc += 1;
        }
        if self.torso.damaged {
            acc += 1;
        }
        if self.left_arm.damaged {
            acc += 1;
        }
        if self.right_arm.damaged {
            acc += 1;
        }
        if self.left_leg.damaged {
            acc += 1;
        }
        if self.right_leg.damaged {
            acc += 1;
        }
        acc
    }
}

impl fmt::Debug for LimbSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut wrote = false;
        for idx in 0..(LType::SIZE as usize) {
            if let Ok(damage) = LType::try_from(idx as u8) {
                if wrote {
                    write!(f, ", ")?;
                }
                if Some(damage) == self.restoring {
                    write!(f, "*")?;
                }
                write!(f, "{}", (self.limbs[idx].damage / 100))?;
                wrote = true;
            }
        }
        Ok(())
    }
}

impl fmt::Display for LimbSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut wrote = false;
        for idx in 0..(LType::SIZE as usize) {
            if let Ok(damage) = LType::try_from(idx as u8) {
                if let Ok(limb) = get_damage_limb(damage) {
                    if self.limbs[idx].damage > DAMAGED_VALUE {
                        if wrote {
                            write!(f, ", ")?;
                        }
                        if Some(damage) == self.restoring {
                            write!(f, "*")?;
                        }
                        if self.limbs[idx].mangled {
                            write!(f, "{}Mangled", limb)?;
                        } else if self.limbs[idx].damaged {
                            write!(f, "{}Damaged", limb)?;
                        } else {
                            write!(f, "{}Hurt", limb)?;
                        }
                        wrote = true;
                    } else if Some(damage) == self.restoring {
                        if wrote {
                            write!(f, ", ")?;
                        }
                        write!(f, "*Pre<{}>", limb)?;
                        wrote = true;
                    }
                }
            }
        }
        Ok(())
    }
}

lazy_static! {
    pub static ref LIMBS: Vec<LType> = vec![
        LType::HeadDamage,
        LType::TorsoDamage,
        LType::LeftArmDamage,
        LType::RightArmDamage,
        LType::LeftLegDamage,
        LType::RightLegDamage,
    ];
}

impl LimbSet {
    pub fn rotate(&mut self, counter: bool) {
        let left_arm = self.limbs[LType::LeftArmDamage as usize];
        let right_arm = self.limbs[LType::RightArmDamage as usize];
        let left_leg = self.limbs[LType::LeftLegDamage as usize];
        let right_leg = self.limbs[LType::RightLegDamage as usize];
        if counter {
            self.limbs[LType::LeftArmDamage as usize] = right_arm;
            self.limbs[LType::RightArmDamage as usize] = right_leg;
            self.limbs[LType::RightLegDamage as usize] = left_leg;
            self.limbs[LType::LeftLegDamage as usize] = left_arm;
        } else {
            self.limbs[LType::LeftArmDamage as usize] = left_leg;
            self.limbs[LType::RightArmDamage as usize] = left_arm;
            self.limbs[LType::RightLegDamage as usize] = right_arm;
            self.limbs[LType::LeftLegDamage as usize] = right_leg;
        }
    }

    pub fn welt(&mut self, limb: LType) {
        self.limbs[limb as usize].welt = true;
    }

    pub fn dewelt(&mut self, limb: LType) {
        self.limbs[limb as usize].welt = false;
    }

    pub fn set_limb_broken(&mut self, limb: LType, damaged: bool) {
        match limb {
            LType::TorsoDamage | LType::HeadDamage => {}
            _ => {
                self.limbs[limb as usize].broken = damaged;
            }
        }
    }

    pub fn broken(&self, limb: LType) -> bool {
        self.limbs[limb as usize].broken
    }

    pub fn set_limb_damaged(&mut self, limb: LType, damaged: bool) {
        if damaged {
            match limb {
                LType::TorsoDamage | LType::HeadDamage => {}
                _ => {
                    self.limbs[limb as usize].broken = true;
                }
            }
        }
        self.limbs[limb as usize].damaged = damaged;
        if damaged && self.limbs[limb as usize].damage <= DAMAGED_VALUE {
            self.limbs[limb as usize].damage = DAMAGED_VALUE + 1;
        } else if !damaged && self.limbs[limb as usize].damage > DAMAGED_VALUE {
            self.limbs[limb as usize].damage = DAMAGED_VALUE;
        }
    }

    pub fn damaged(&self, limb: LType) -> bool {
        self.limbs[limb as usize].damaged
    }

    pub fn set_limb_mangled(&mut self, limb: LType, damaged: bool) {
        if damaged {
            match limb {
                LType::TorsoDamage | LType::HeadDamage => {}
                _ => {
                    self.limbs[limb as usize].broken = true;
                }
            }
        }
        self.limbs[limb as usize].mangled = damaged;
        if damaged && self.limbs[limb as usize].damage <= MANGLED_VALUE {
            self.limbs[limb as usize].damage = MANGLED_VALUE + 1;
        } else if !damaged && self.limbs[limb as usize].damage > MANGLED_VALUE {
            self.limbs[limb as usize].damage = MANGLED_VALUE;
        }
    }

    pub fn mangled(&self, limb: LType) -> bool {
        self.limbs[limb as usize].mangled
    }

    pub fn wait(&mut self, duration: CType) -> Option<FType> {
        if let (Some(remaining), Some(restored)) = (self.restore_timer, self.restoring) {
            if remaining < duration {
                self.complete_restore(None)
            } else {
                self.restore_timer = Some(remaining - duration);
            }
            None
        } else if let (Some(remaining), Some(cured)) = (self.restore_timer, self.curing) {
            if remaining < duration {
                self.complete_restore(None);
                Some(cured)
            } else {
                self.restore_timer = Some(remaining - duration);
                None
            }
        } else {
            None
        }
    }

    pub fn get_damage(&self, broken: LType) -> CType {
        self.limbs[broken as usize].damage
    }

    pub fn set_limb_damage(&mut self, broken: LType, new_damage: CType) {
        self.limbs[broken as usize].damage = new_damage;
        if self.limbs[broken as usize].damage < DAMAGED_VALUE {
            self.limbs[broken as usize].damaged = false;
            self.limbs[broken as usize].mangled = false;
        } else if self.limbs[broken as usize].damage < MANGLED_VALUE {
            self.limbs[broken as usize].mangled = false;
        }
    }

    pub fn adjust_limb(&mut self, limb: LType, value: CType) {
        self.limbs[limb as usize].damage += value;
        if self.limbs[limb as usize].damage < 0 {
            self.limbs[limb as usize].damage = 0;
        } else if self.limbs[limb as usize].damage > 10000 {
            self.limbs[limb as usize].damage = 10000;
        }
    }

    pub fn complete_restore(&mut self, broken: Option<LType>) {
        if broken == self.restoring || broken == None {
            let expected_heal = if self.regenerating { 4500 } else { 3000 };
            if let Some(broken) = self.restoring {
                let new_damage = self.limbs[broken as usize].damage
                    - i32::min(self.limbs[broken as usize].damage, expected_heal);
                println!(
                    "{} -> {} ({:?})",
                    self.limbs[broken as usize].damage, new_damage, self.restore_timer
                );
                self.set_limb_damage(broken, new_damage);
            }
            self.regenerating = false;
            self.restoring = None;
            self.restore_timer = None;
        }
    }

    pub fn start_restore(&mut self, broken: LType) {
        if let Some(timer) = self.restore_timer {
            if timer < 10 {
                self.complete_restore(None);
            }
        }
        self.restoring = Some(broken);
        self.restore_timer = Some(400);
    }

    pub fn start_restore_cure(&mut self, aff: FType) {
        if let Some(timer) = self.restore_timer {
            if timer < 10 {
                self.complete_restore(None);
            }
        }
        self.curing = Some(aff);
        self.restore_timer = Some(400);
    }

    pub fn get_limbs_damage(&self, limbs: Vec<LType>) -> f32 {
        let mut total = 0;
        for limb in limbs.iter() {
            total = total + self.limbs[*limb as usize].damage;
        }
        total as f32 / 100.0
    }

    pub fn get_total_damage(&self) -> f32 {
        (self.limbs[0].damage
            + self.limbs[1].damage
            + self.limbs[2].damage
            + self.limbs[3].damage
            + self.limbs[4].damage
            + self.limbs[5].damage) as f32
            / 100.0
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum RelapseState {
    Inactive,
    Active(Vec<(CType, String)>),
}

pub enum RelapseResult {
    Concrete(Vec<String>),
    Uncertain(Vec<(CType, String)>),
    None,
}

impl Default for RelapseState {
    fn default() -> Self {
        RelapseState::Inactive
    }
}

impl RelapseState {
    pub fn wait(&mut self, duration: CType) {
        match self {
            RelapseState::Active(relapses) => {
                for relapse in relapses.iter_mut() {
                    relapse.0 += duration;
                }
            }
            RelapseState::Inactive => {}
        }
    }

    pub fn push(&mut self, venom: String) {
        match self {
            RelapseState::Active(relapses) => {
                relapses.push((0 as CType, venom));
            }
            RelapseState::Inactive => {
                *self = RelapseState::Active(vec![(0 as CType, venom)]);
            }
        }
    }

    fn is_venom_ripe(time: CType) -> bool {
        time > (1.9 * BALANCE_SCALE as f32) as CType && time < (7.1 * BALANCE_SCALE as f32) as CType
    }

    fn is_venom_alive(time: CType) -> bool {
        time < (7.1 * BALANCE_SCALE as f32) as CType
    }

    pub fn stalest(&self, venoms: Vec<String>) -> Option<String> {
        match self {
            RelapseState::Active(relapses) => {
                let mut ages = HashMap::new();
                for venom in venoms.iter() {
                    ages.insert(venom, BALANCE_SCALE as CType * 10);
                }
                for (time, venom) in relapses.iter() {
                    if ages.contains_key(venom) {
                        ages.insert(venom, *time);
                    }
                }
                ages.iter()
                    .max_by_key(|(venom, age)| *age)
                    .map(|(venom, age)| venom.to_string())
            }
            _ => venoms.get(0).cloned(),
        }
    }

    pub fn get_relapses(&mut self, relapse_count: usize) -> RelapseResult {
        match self {
            RelapseState::Active(relapses) => {
                let mut possible = Vec::new();
                for (time, venom) in relapses.iter() {
                    if RelapseState::is_venom_ripe(*time) {
                        possible.push(venom.to_string());
                    }
                }
                if possible.len() == relapse_count {
                    relapses.retain(|(time, _venom)| {
                        !RelapseState::is_venom_ripe(*time) && RelapseState::is_venom_alive(*time)
                    });
                    RelapseResult::Concrete(possible)
                } else if possible.len() > 0 {
                    relapses.retain(|(time, _venom)| RelapseState::is_venom_alive(*time));
                    RelapseResult::Uncertain(relapses.clone())
                } else {
                    RelapseResult::None
                }
            }
            RelapseState::Inactive => RelapseResult::None,
        }
    }
}

const SOFT_COOLDOWN: f32 = 2.0;
const HARD_COOLDOWN: f32 = 6.0;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Default)]
pub struct HypnoState {
    pub hypnotized: bool,
    pub active: bool,
    pub sealed: Option<f32>,
    pub hypnosis_stack: Vec<Hypnosis>,
}

impl HypnoState {
    pub fn fire(&mut self) -> Option<Hypnosis> {
        if self.hypnosis_stack.len() <= 1 {
            self.active = false;
        } else {
            self.active = true;
        }
        if self.hypnosis_stack.len() > 0 {
            let top = self.hypnosis_stack.get(0).cloned();
            self.hypnosis_stack.remove(0);
            top
        } else {
            self.desway();
            None
        }
    }

    pub fn pop_suggestion(&mut self, active: bool) -> Option<Hypnosis> {
        if self.hypnosis_stack.len() > 0 {
            if active {
                if self.hypnosis_stack.len() == 1 {
                    self.active = false;
                } else if !self.active {
                    self.active = true;
                }
            }
            self.hypnosis_stack.pop()
        } else {
            None
        }
    }

    pub fn push_suggestion(&mut self, suggestion: Hypnosis) {
        self.hypnosis_stack.push(suggestion);
        self.active = false;
        self.hypnotized = true;
        self.sealed = None;
    }

    pub fn get_next_hypno_aff(&self) -> Option<FType> {
        if !self.active {
            return None;
        }
        if let Some(Hypnosis::Aff(aff)) = self.hypnosis_stack.get(0) {
            Some(*aff)
        } else {
            None
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
        self.sealed = None;
    }

    pub fn hypnotize(&mut self) {
        self.hypnotized = true;
        self.active = false;
        self.sealed = None;
    }

    pub fn desway(&mut self) {
        self.hypnotized = false;
        self.active = false;
        self.sealed = None;
        self.hypnosis_stack = Vec::new();
    }

    pub fn seal(&mut self, length: f32) {
        self.sealed = Some(length);
        self.hypnotized = false;
        self.active = false;
    }
}

#[derive(Debug, Clone)]
pub enum ZenithState {
    Inactive,
    Rising(CType),
    Active(CType),
}

impl Default for ZenithState {
    fn default() -> Self {
        ZenithState::Inactive
    }
}

impl ZenithState {
    pub fn wait(&mut self, duration: CType) {
        match self.clone() {
            ZenithState::Inactive => {}
            ZenithState::Rising(remaining) => {
                if remaining > duration {
                    *self = ZenithState::Rising(remaining - duration);
                } else {
                    self.activate();
                }
            }
            ZenithState::Active(remaining) => {
                if remaining > duration {
                    *self = ZenithState::Active(remaining - duration);
                } else {
                    self.deactivate();
                }
            }
        }
    }
    pub fn initiate(&mut self) {
        *self = ZenithState::Rising((15.0 * BALANCE_SCALE) as CType);
    }
    pub fn activate(&mut self) {
        *self = ZenithState::Active((10.0 * BALANCE_SCALE) as CType);
    }
    pub fn deactivate(&mut self) {
        *self = ZenithState::Inactive;
    }
    pub fn can_initiate(&self) -> bool {
        match self {
            ZenithState::Inactive => true,
            _ => false,
        }
    }
    pub fn active(&self) -> bool {
        match self {
            ZenithState::Active(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AgentState {
    pub balances: [CType; BType::SIZE as usize],
    pub stats: [CType; SType::SIZE as usize],
    pub max_stats: [CType; SType::SIZE as usize],
    pub flags: FlagSet,
    pub limb_damage: LimbSet,
    pub hypno_state: HypnoState,
    pub zenith_state: ZenithState,
    pub relapses: RelapseState,
    pub parrying: Option<LType>,
    pub wield_state: WieldState,
    pub dodge_state: DodgeState,
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
        self.relapses.wait(duration);
        self.zenith_state.wait(duration);
        self.dodge_state.wait(duration);
        if let Some(cured_aff) = self.limb_damage.wait(duration) {
            self.set_flag(cured_aff, false);
        }
        let rebound_pending = !self.balanced(BType::Rebounding) && !self.is(FType::Rebounding);
        for i in 0..self.balances.len() {
            self.balances[i] -= duration;
        }
        if rebound_pending && self.balanced(BType::Rebounding) {
            self.set_flag(FType::AssumedRebounding, true);
        }
        if self.is(FType::Void) && self.balanced(BType::Void) {
            self.set_flag(FType::Void, false);
        } else if self.is(FType::Weakvoid) && self.balanced(BType::Void) {
            self.set_flag(FType::Weakvoid, false);
        }
    }

    pub fn will_be_rebounding(&self, qeb: f32) -> bool {
        if self.is(FType::Rebounding)
            || (self.is(FType::AssumedRebounding) && self.get_balance(BType::Rebounding) > -1.0)
        {
            true
        } else if !self.balanced(BType::Rebounding) {
            self.get_balance(BType::Rebounding) < qeb
        } else {
            false
        }
    }

    pub fn is(&self, flag: FType) -> bool {
        match flag {
            FType::LeftLegBroken => self.limb_damage.broken(LType::LeftLegDamage),
            FType::RightLegBroken => self.limb_damage.broken(LType::RightLegDamage),
            FType::LeftArmBroken => self.limb_damage.broken(LType::LeftArmDamage),
            FType::RightArmBroken => self.limb_damage.broken(LType::RightArmDamage),
            _ => self.flags.is_flag_set(flag),
        }
    }

    pub fn get_count(&self, flag: FType) -> u8 {
        self.flags.get_flag_count(flag)
    }

    pub fn set_count(&mut self, flag: FType, value: u8) {
        self.flags.set_flag_count(flag, value);
    }

    pub fn some(&self, afflictions: Vec<FType>) -> bool {
        for affliction in afflictions.iter() {
            if self.is(*affliction) {
                return true;
            }
        }
        return false;
    }

    pub fn set_flag(&mut self, flag: FType, value: bool) {
        match flag {
            FType::LeftLegBroken => self
                .limb_damage
                .set_limb_broken(LType::LeftLegDamage, value),
            FType::RightLegBroken => self
                .limb_damage
                .set_limb_broken(LType::RightLegDamage, value),
            FType::LeftArmBroken => self
                .limb_damage
                .set_limb_broken(LType::LeftArmDamage, value),
            FType::RightArmBroken => self
                .limb_damage
                .set_limb_broken(LType::RightArmDamage, value),
            _ => self.flags.set_flag(flag, value),
        }
        if flag == FType::Rebounding && value == true {
            self.flags.set_flag(FType::AssumedRebounding, false);
        }
        if (flag == FType::Weakvoid || flag == FType::Void) && value == true {
            self.set_balance(BType::Void, 10.0);
        }
        if value && flag == FType::Paresis {
            self.set_balance(BType::ParesisParalysis, 4.0);
        }
        if flag == FType::Zenith {
            if value {
                self.zenith_state.activate();
            } else {
                self.zenith_state.deactivate();
            }
        }
    }

    pub fn tick_flag_up(&mut self, flag: FType) {
        self.flags.tick_counter_up(flag);
    }

    pub fn set_balance(&mut self, balance: BType, value: f32) {
        self.balances[balance as usize] = (value * BALANCE_SCALE) as CType;
    }

    pub fn get_raw_balance(&self, balance: BType) -> CType {
        self.balances[balance as usize]
    }

    pub fn get_qeb_balance(&self) -> f32 {
        f32::max(
            0.0,
            f32::max(
                self.get_balance(BType::Balance),
                self.get_balance(BType::Equil),
            ),
        )
    }

    pub fn get_balance(&self, balance: BType) -> f32 {
        (self.balances[balance as usize] as f32) / (BALANCE_SCALE as f32)
    }

    pub fn balanced(&self, balance: BType) -> bool {
        self.balances[balance as usize] <= 0
    }

    pub fn qeb_balance(&self) -> BType {
        if self.get_raw_balance(BType::Balance) <= self.get_raw_balance(BType::Equil) {
            BType::Balance
        } else {
            BType::Equil
        }
    }

    pub fn next_balance<'s>(&self, balances: impl Iterator<Item = &'s BType>) -> Option<BType> {
        let mut earliest: Option<&BType> = None;
        for balance in balances {
            if let Some(earliest_bal) = earliest {
                if self.balances[*earliest_bal as usize] <= 0 {
                    // Do nothing.
                } else if self.balances[*balance as usize] < self.balances[*earliest_bal as usize] {
                    earliest = Some(balance)
                }
            } else {
                earliest = Some(balance)
            }
        }
        earliest.cloned()
    }

    pub fn set_stat(&mut self, stat: SType, value: CType) {
        self.stats[stat as usize] = value;
    }

    pub fn get_stat(&self, stat: SType) -> CType {
        self.stats[stat as usize]
    }

    pub fn set_limb_damage(&mut self, limb: LType, value: CType) {
        let old_value = self.limb_damage.limbs[limb as usize].damage;
        if old_value < value - 200 || old_value > value + 200 {
            println!("{:?} {} -> {}", limb, old_value, value);
        }
        self.limb_damage.set_limb_damage(limb, value);
    }

    pub fn get_limbs_state(&self) -> LimbsState {
        LimbsState {
            head: self.get_limb_state(LType::HeadDamage),
            torso: self.get_limb_state(LType::TorsoDamage),
            left_arm: self.get_limb_state(LType::LeftArmDamage),
            right_arm: self.get_limb_state(LType::RightArmDamage),
            left_leg: self.get_limb_state(LType::LeftLegDamage),
            right_leg: self.get_limb_state(LType::RightLegDamage),
        }
    }

    pub fn get_limb_state(&self, what: LType) -> LimbState {
        let limb = self.limb_damage.limbs[what as usize];
        let damage = limb.damage as f32 / 100.0;
        let damaged = limb.damaged;
        let mangled = limb.mangled;
        let broken = match what.broken() {
            Some(broken_aff) => self.is(broken_aff),
            _ => false,
        } || damage > 35.0;
        let welt = limb.welt;
        let is_restoring = self.limb_damage.restoring == Some(what);
        let is_parried = self.can_parry() && self.parrying == Some(what);
        let is_dislocated = match what {
            LType::LeftArmDamage => self.is(FType::LeftArmDislocated),
            LType::RightArmDamage => self.is(FType::RightArmDislocated),
            LType::LeftLegDamage => self.is(FType::LeftLegDislocated),
            LType::RightLegDamage => self.is(FType::RightLegDislocated),
            _ => false,
        };
        LimbState {
            damage,
            broken,
            damaged,
            mangled,
            is_restoring,
            is_parried,
            is_dislocated,
            welt,
        }
    }

    pub fn clear_parrying(&mut self) {
        self.parrying = None;
    }

    pub fn get_parrying(&self) -> Option<LType> {
        self.parrying
    }

    pub fn set_parrying(&mut self, limb: LType) {
        self.parrying = Some(limb);
    }

    /*
        pub fn adjust_stat(&mut self, stat: SType, value: CType) {
            self.stats[stat as usize] += value;
        }
    */

    pub fn initialize_stat(&mut self, stat: SType, value: CType) {
        self.max_stats[stat as usize] = value;
        self.stats[stat as usize] = value;
    }

    pub fn can_smoke(&self, ignore_bal: bool) -> bool {
        !self.is(FType::Asthma) && (ignore_bal || self.balanced(BType::Smoke))
    }

    pub fn can_pill(&self, ignore_bal: bool) -> bool {
        !self.is(FType::Anorexia) && (ignore_bal || self.balanced(BType::Pill))
    }

    pub fn can_salve(&self, ignore_bal: bool) -> bool {
        !self.is(FType::Slickness) && (ignore_bal || self.balanced(BType::Salve))
    }

    pub fn lock_duration(&self) -> Option<f32> {
        let mut earliest_escape = None;
        if self.is(FType::Asthma) && self.is(FType::Anorexia) && self.is(FType::Slickness) {
            if !self.is(FType::Paralysis) && !self.is(FType::Paresis) {
                earliest_escape = Some(self.balances[BType::Tree as usize]);
            }
            if !self.is(FType::Impatience) && !self.is(FType::Stupidity) {
                let focus_time = self.balances[BType::Focus as usize];
                earliest_escape = earliest_escape.map_or(Some(focus_time), |other| {
                    if other < focus_time {
                        Some(other)
                    } else {
                        Some(focus_time)
                    }
                });
            }
            earliest_escape = earliest_escape.or(Some((15.0 * BALANCE_SCALE) as CType))
        }
        earliest_escape.map(|escape| (escape as f32) / BALANCE_SCALE)
    }

    pub fn can_renew(&self, ignore_bal: bool) -> bool {
        ignore_bal || self.balanced(BType::Renew)
    }

    pub fn can_touch(&self) -> bool {
        !self.is(FType::Paresis)
            && !self.is(FType::Paralysis)
            && !(self.is(FType::LeftArmBroken) && self.is(FType::RightArmBroken))
            && !self.is(FType::NumbArms)
    }

    pub fn can_tree(&self, ignore_bal: bool) -> bool {
        self.can_touch() && (ignore_bal || self.balanced(BType::Tree))
    }

    pub fn can_focus(&self, ignore_bal: bool) -> bool {
        !self.is(FType::Impatience) && (ignore_bal || self.balanced(BType::Focus))
    }

    pub fn can_parry(&self) -> bool {
        !self.is(FType::Indifference)
            && !self.is(FType::Frozen)
            && !self.is(FType::Paralysis)
            && !(self.is(FType::LeftArmBroken) && self.is(FType::RightArmBroken))
    }

    pub fn is_prone(&self) -> bool {
        self.is(FType::Fallen)
            || self.is(FType::Frozen)
            || self.is(FType::Indifference)
            || self.is(FType::Asleep)
            || self.is(FType::Stun)
            || self.is(FType::Paralysis)
            || self.is(FType::WritheBind)
    }

    pub fn can_stand(&self) -> bool {
        !self.is(FType::LeftLegBroken) && !self.is(FType::RightLegBroken)
    }

    pub fn push_toxin(&mut self, venom: String) {
        self.relapses.push(venom);
    }

    pub fn get_relapses(&mut self, relapse_count: usize) -> RelapseResult {
        self.relapses.get_relapses(relapse_count)
    }

    pub fn clear_relapses(&mut self) {
        self.relapses = RelapseState::Inactive;
    }

    pub fn set_restoring(&mut self, damage: LType) {
        if damage == LType::TorsoDamage
            && !self.limb_damage.limbs[LType::TorsoDamage as usize].damaged
            && self.is(FType::Heatspear)
        {
            self.limb_damage.start_restore_cure(FType::Heatspear);
        } else if damage == LType::TorsoDamage
            && !self.limb_damage.limbs[LType::TorsoDamage as usize].damaged
            && self.is(FType::Deepwound)
        {
            self.limb_damage.start_restore_cure(FType::Deepwound);
        } else {
            self.limb_damage.start_restore(damage);
        }
    }

    pub fn get_restore_time_left(&self) -> f32 {
        if let Some(timer) = self.limb_damage.restore_timer {
            timer as f32 / BALANCE_SCALE
        } else {
            0.0
        }
    }

    pub fn get_restoring(&self) -> Option<(LType, CType, bool)> {
        if let Some(limb) = self.limb_damage.restoring {
            Some((
                limb,
                self.limb_damage.limbs[limb as usize].damage,
                self.limb_damage.regenerating,
            ))
        } else {
            None
        }
    }

    pub fn complete_restoration(&mut self, damage: LType) {
        self.limb_damage.complete_restore(Some(damage));
    }

    pub fn regenerate(&mut self) {
        self.limb_damage.regenerating = true;
    }

    pub fn rotate_limbs(&mut self, counter: bool) {
        self.limb_damage.rotate(counter);
        let dislocated_left_arm = self.is(FType::LeftArmDislocated);
        if counter {
            self.set_flag(FType::LeftArmDislocated, self.is(FType::RightArmDislocated));
            self.set_flag(
                FType::RightArmDislocated,
                self.is(FType::RightLegDislocated),
            );
            self.set_flag(FType::RightLegDislocated, self.is(FType::LeftLegDislocated));
            self.set_flag(FType::LeftLegDislocated, dislocated_left_arm);
        } else {
            self.set_flag(FType::LeftArmDislocated, self.is(FType::LeftLegDislocated));
            self.set_flag(FType::LeftLegDislocated, self.is(FType::RightLegDislocated));
            self.set_flag(
                FType::RightLegDislocated,
                self.is(FType::RightArmDislocated),
            );
            self.set_flag(FType::RightArmDislocated, dislocated_left_arm);
        }
    }

    pub fn restore_count(&self) -> CType {
        let mut count = 0;
        for limb in LIMBS.to_vec() {
            if self.limb_damage.mangled(limb) {
                count += 2;
            } else if self.limb_damage.damaged(limb) {
                count += 1;
            }
        }
        count
    }

    pub fn wield_multi(&mut self, left: Option<String>, right: Option<String>) {
        self.wield_state = match &self.wield_state {
            WieldState::Normal {
                left: old_left,
                right: old_right,
            } => WieldState::Normal {
                left: left.or(old_left.clone()),
                right: right.or(old_right.clone()),
            },
            WieldState::TwoHanded(_what) => WieldState::Normal { left, right },
        };
    }

    pub fn unwield_multi(&mut self, left: bool, right: bool) {
        self.wield_state = match &self.wield_state {
            WieldState::Normal {
                left: old_left,
                right: old_right,
            } => WieldState::Normal {
                left: if left { None } else { old_left.clone() },
                right: if right { None } else { old_right.clone() },
            },
            WieldState::TwoHanded(_what) => WieldState::Normal {
                left: None,
                right: None,
            },
        }
    }

    pub fn wield_two_hands(&mut self, what: String) {
        self.wield_state = WieldState::TwoHanded(what);
    }

    pub fn aff_count(&self) -> usize {
        let mut count = 0;
        for _ in self.flags.aff_iter() {
            count += 1;
        }
        count
    }

    pub fn affs_count(&self, affs: &Vec<FType>) -> usize {
        let mut count = 0;
        for aff in affs.iter() {
            if self.is(*aff) {
                count += 1;
            }
        }
        count
    }
}
