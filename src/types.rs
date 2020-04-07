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
    Renew,
    Regenerate,

    // Misc
    ClassCure1,

    // Timers
    Hypnosis,
    Fangbarrier,
    Rebounding,
    Restoration,

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

pub fn get_limb_damage(what: &String) -> Result<LType, String> {
    match what.as_ref() {
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
    HeadBruisedCritical,
    DestroyedThroat,
    CrippledThroat,
    HeadBruisedModerate,
    HeadBruised,

    // Mending Torso
    TorsoBruisedCritical,
    Lightwound,
    Ablaze,
    CrackedRibs,
    TorsoBruisedModerate,
    TorsoBruised,

    // Mending Left Arm
    LeftArmBruisedCritical,
    LeftArmBroken,
    LeftArmBruisedModerate,
    LeftArmBruised,
    LeftArmDislocated,

    // Mending Right Arm
    RightArmBruisedCritical,
    RightArmBroken,
    RightArmBruisedModerate,
    RightArmBruised,
    RightArmDislocated,

    // Mending Left Leg
    LeftLegBruisedCritical,
    LeftLegBroken,
    LeftLegBruisedModerate,
    LeftLegBruised,
    LeftLegDislocated,

    // Mending Right Leg
    RightLegBruisedCritical,
    RightLegBroken,
    RightLegBruisedModerate,
    RightLegBruised,
    RightLegDislocated,

    // Restoration Head
    Voidgaze,
    MauledFace,

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

    // Special
    Disrupted,
    Fear,
    Prone,

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
}

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
    predicate: &'s Fn(FType) -> bool,
}

impl<'s> Iterator for FlagSetIterator<'s> {
    type Item = FType;
    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.set.0.len() && !self.set.0[self.index] {
            self.index += 1;
        }
        if self.index < self.set.0.len() {
            self.index += 1;
            let ftype = FType::try_from((self.index - 1) as u16).unwrap();
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

impl<'s> FlagSetIterator<'s> {
    fn new(flagset: &'s FlagSet, predicate: &'s Fn(FType) -> bool) -> Self {
        FlagSetIterator {
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

#[derive(Clone, Default)]
pub struct LimbSet([CType; LType::SIZE as usize], Option<LType>, bool);

impl fmt::Debug for LimbSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut wrote = false;
        for idx in 0..(LType::SIZE as usize) {
            if let Ok(damage) = LType::try_from(idx as u8) {
                if wrote {
                    write!(f, ", ")?;
                }
                if Some(damage) == self.1 {
                    write!(f, "*")?;
                }
                write!(f, "{}", (self.0[idx] / 100))?;
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
                    if self.0[idx] > 3333 {
                        if wrote {
                            write!(f, ", ")?;
                        }
                        if Some(damage) == self.1 {
                            write!(f, "*")?;
                        }
                        if self.0[idx] > 6666 {
                            write!(f, "{}Mangled", limb)?;
                        } else {
                            write!(f, "{}Damaged", limb)?;
                        }
                        wrote = true;
                    } else if Some(damage) == self.1 {
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
    static ref BROKEN_LIMBS: Vec<FType> = vec![
        FType::LeftLegBroken,
        FType::RightLegBroken,
        FType::LeftArmBroken,
        FType::RightArmBroken,
    ];
}

impl LimbSet {
    pub fn rotate(&mut self, counter: bool) {
        let left_arm = self.0[LType::LeftArmDamage as usize];
        let right_arm = self.0[LType::RightArmDamage as usize];
        let left_leg = self.0[LType::LeftLegDamage as usize];
        let right_leg = self.0[LType::RightLegDamage as usize];
        if counter {
            self.0[LType::LeftArmDamage as usize] = right_arm;
            self.0[LType::RightArmDamage as usize] = right_leg;
            self.0[LType::RightLegDamage as usize] = left_leg;
            self.0[LType::LeftLegDamage as usize] = left_arm;
        } else {
            self.0[LType::LeftArmDamage as usize] = left_leg;
            self.0[LType::RightArmDamage as usize] = left_arm;
            self.0[LType::RightLegDamage as usize] = right_arm;
            self.0[LType::LeftLegDamage as usize] = right_leg;
        }
    }

    pub fn damaged(&self, broken: FType) -> bool {
        match broken {
            FType::LeftArmBroken => self.0[LType::LeftArmDamage as usize] > 3333,
            FType::RightArmBroken => self.0[LType::RightArmDamage as usize] > 3333,
            FType::LeftLegBroken => self.0[LType::LeftLegDamage as usize] > 3333,
            FType::RightLegBroken => self.0[LType::RightLegDamage as usize] > 3333,
            _ => false,
        }
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

#[derive(Debug, Clone, Default)]
pub struct AgentState {
    pub balances: [CType; BType::SIZE as usize],
    pub stats: [CType; SType::SIZE as usize],
    pub max_stats: [CType; SType::SIZE as usize],
    pub flags: FlagSet,
    pub limb_damage: LimbSet,
    pub hypnosis_stack: Vec<Hypnosis>,
    pub relapses: Vec<String>,
    pub parrying: Option<LType>,
    pub wield_state: WieldState,
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

    pub fn some(&self, afflictions: Vec<FType>) -> bool {
        for affliction in afflictions.iter() {
            if self.is(*affliction) {
                return true;
            }
        }
        return false;
    }

    pub fn set_flag(&mut self, flag: FType, value: bool) {
        self.flags.0[flag as usize] = value;
    }

    pub fn get_flag(&self, flag: FType) -> bool {
        self.flags.0[flag as usize]
    }
    /*
        pub fn affliction_count(&self) -> i32 {
            let mut count = 0;
            for i in 0..(FType::SIZE as usize) {
                if i >= FType::Sadness as usize && i < FType::SIZE as usize {
                    count += 1;
                }
            }
            count
        }
    */
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

    pub fn adjust_limb(&mut self, limb: LType, value: CType) {
        self.limb_damage.0[limb as usize] += value;
        if self.limb_damage.0[limb as usize] < 0 {
            self.limb_damage.0[limb as usize] = 0;
        } else if self.limb_damage.0[limb as usize] > 10000 {
            self.limb_damage.0[limb as usize] = 10000;
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

    pub fn can_tree(&self, ignore_bal: bool) -> bool {
        !self.is(FType::Paresis)
            && !self.is(FType::Paralysis)
            && !(self.is(FType::LeftArmBroken) && self.is(FType::RightArmBroken))
            && !self.is(FType::NumbArms)
            && (ignore_bal || self.balanced(BType::Tree))
    }

    pub fn can_focus(&self, ignore_bal: bool) -> bool {
        !self.is(FType::Impatience) && (ignore_bal || self.balanced(BType::Focus))
    }

    pub fn push_toxin(&mut self, venom: String) {
        self.relapses.push(venom);
    }

    pub fn relapse(&mut self) -> Option<String> {
        if let Some(_aff) = self.relapses.first() {
            Some(self.relapses.remove(0))
        } else {
            None
        }
    }

    pub fn clear_relapses(&mut self) {
        self.relapses = Vec::new();
    }

    pub fn set_restoring(&mut self, damage: LType) {
        self.limb_damage.1 = Some(damage);
        self.set_balance(BType::Restoration, 4.0);
    }

    pub fn get_restoring(&self) -> Option<(LType, CType, bool)> {
        if let Some(limb) = self.limb_damage.1 {
            Some((limb, self.limb_damage.0[limb as usize], self.limb_damage.2))
        } else {
            None
        }
    }

    pub fn complete_restoration(&mut self, _damage: LType) {
        self.limb_damage.1 = None;
        self.limb_damage.2 = false;
    }

    pub fn regenerate(&mut self) {
        self.limb_damage.2 = true;
    }

    pub fn rotate_limbs(&mut self, counter: bool) {
        self.limb_damage.rotate(counter);
    }

    pub fn restore_count(&self) -> CType {
        let mut count = 0;
        for limb in BROKEN_LIMBS.to_vec() {
            if self.is(limb) && !self.limb_damage.damaged(limb) {
                count += 1;
            }
        }
        count
    }

    pub fn pop_suggestion(&mut self) -> Option<Hypnosis> {
        if self.hypnosis_stack.len() > 0 {
            self.hypnosis_stack.pop()
        } else {
            None
        }
    }

    pub fn push_suggestion(&mut self, suggestion: Hypnosis) {
        self.hypnosis_stack.push(suggestion);
    }

    pub fn get_next_hypno_aff(&self) -> Option<FType> {
        if !self.is(FType::Snapped) {
            return None;
        }
        if let Some(Hypnosis::Aff(aff)) = self.hypnosis_stack.get(0) {
            Some(*aff)
        } else {
            None
        }
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
