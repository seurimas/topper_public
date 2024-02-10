use super::*;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;
use topper_core::timeline::BaseAgentState;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, TryFromPrimitive, Serialize, Deserialize)]
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
    pub fn from_name(name: &String) -> Self {
        match name.to_ascii_lowercase().as_ref() {
            "head" => LType::HeadDamage,
            "torso" => LType::TorsoDamage,
            "left arm" => LType::LeftArmDamage,
            "right arm" => LType::RightArmDamage,
            "left leg" => LType::LeftLegDamage,
            "right leg" => LType::RightLegDamage,
            _ => LType::HeadDamage,
        }
    }
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

    pub fn crippled(&self) -> Option<FType> {
        match self {
            LType::LeftArmDamage => Some(FType::LeftArmCrippled),
            LType::RightArmDamage => Some(FType::RightArmCrippled),
            LType::LeftLegDamage => Some(FType::LeftLegCrippled),
            LType::RightLegDamage => Some(FType::RightLegCrippled),
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
        "head_mangled" => Ok((LType::HeadDamage, 6666)),
        "head_damaged" => Ok((LType::HeadDamage, 3333)),
        "torso_mangled" => Ok((LType::TorsoDamage, 6666)),
        "torso_damaged" => Ok((LType::TorsoDamage, 3333)),
        "left_arm_mangled" => Ok((LType::LeftArmDamage, 6666)),
        "left_arm_damaged" => Ok((LType::LeftArmDamage, 3333)),
        "right_arm_mangled" => Ok((LType::RightArmDamage, 6666)),
        "right_arm_damaged" => Ok((LType::RightArmDamage, 3333)),
        "left_leg_mangled" => Ok((LType::LeftLegDamage, 6666)),
        "left_leg_damaged" => Ok((LType::LeftLegDamage, 3333)),
        "right_leg_mangled" => Ok((LType::RightLegDamage, 6666)),
        "right_leg_damaged" => Ok((LType::RightLegDamage, 3333)),
        "left_arm_amputated" => Ok((LType::LeftArmDamage, 10000)),
        "right_arm_amputated" => Ok((LType::RightArmDamage, 10000)),
        "left_leg_amputated" => Ok((LType::LeftLegDamage, 10000)),
        "right_leg_amputated" => Ok((LType::RightLegDamage, 10000)),
        _ => Err(format!("Could not find damage for {}", aff)),
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Limb {
    pub damage: CType,
    pub crippled: bool,
    pub broken: bool,
    pub mangled: bool,
    pub amputated: bool,
    pub welt: bool,
}

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct LimbSet {
    pub limbs: [Limb; LType::SIZE as usize],
    pub restoring: Option<LType>,
    pub restore_timer: Option<Timer>,
    pub fleshbaned_count: usize,
    pub regenerating: bool,
    pub first_person_restore: bool,
}

pub const DAMAGED_VALUE: CType = 3332;
pub const MANGLED_VALUE: CType = 6665;

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct LimbState {
    pub damage: f32,
    pub crippled: bool,
    pub broken: bool,
    pub mangled: bool,
    pub amputated: bool,
    pub is_restoring: bool,
    pub is_parried: bool,
    pub is_dislocated: bool,
    pub welt: bool,
    pub bruise_level: usize,
    pub fleshbaned_count: usize,
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

    pub fn hits_to_mangle(&self, damage: f32) -> i32 {
        let damaged_value = (MANGLED_VALUE + 1) as f32 / 100.0;
        f32::ceil((damaged_value - self.damage) / damage) as i32
    }

    pub fn assume_restore(&mut self) {
        let mut restore_value = 30.0;
        restore_value -= restore_value * self.fleshbaned_count as f32 * 2.0;
        if restore_value > 0. {
            self.damage -= restore_value;
        }
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
        if self.head.broken {
            acc += 1;
        }
        if self.torso.broken {
            acc += 1;
        }
        if self.left_arm.broken {
            acc += 1;
        }
        if self.right_arm.broken {
            acc += 1;
        }
        if self.left_leg.broken {
            acc += 1;
        }
        if self.right_leg.broken {
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
                    if self.limbs[idx].damage >= DAMAGED_VALUE {
                        if wrote {
                            write!(f, ", ")?;
                        }
                        if Some(damage) == self.restoring {
                            write!(f, "*")?;
                        }
                        if self.limbs[idx].mangled {
                            write!(f, "{}Mangled", limb)?;
                        } else if self.limbs[idx].broken {
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

    pub fn set_limb_crippled(&mut self, limb: LType, damaged: bool) {
        match limb {
            LType::TorsoDamage | LType::HeadDamage => {}
            _ => {
                self.limbs[limb as usize].crippled = damaged;
                if !damaged {
                    self.limbs[limb as usize].broken = false;
                    self.limbs[limb as usize].mangled = false;
                }
            }
        }
    }

    pub fn crippled(&self, limb: LType) -> bool {
        self.limbs[limb as usize].crippled
    }

    pub fn set_limb_broken(&mut self, limb: LType, damaged: bool) {
        if damaged {
            match limb {
                LType::TorsoDamage | LType::HeadDamage => {}
                _ => {
                    self.limbs[limb as usize].crippled = true;
                }
            }
        }
        self.limbs[limb as usize].broken = damaged;
        if !damaged {
            self.limbs[limb as usize].mangled = false;
        }
        if damaged && self.limbs[limb as usize].damage <= DAMAGED_VALUE {
            self.limbs[limb as usize].damage = DAMAGED_VALUE + 1;
        } else if !damaged && self.limbs[limb as usize].damage >= DAMAGED_VALUE {
            self.limbs[limb as usize].damage = DAMAGED_VALUE - 1;
        }
    }

    pub fn broken(&self, limb: LType) -> bool {
        self.limbs[limb as usize].broken
    }

    pub fn set_limb_mangled(&mut self, limb: LType, damaged: bool) {
        if damaged {
            match limb {
                LType::TorsoDamage | LType::HeadDamage => {}
                _ => {
                    self.limbs[limb as usize].crippled = true;
                }
            }
        }
        self.limbs[limb as usize].mangled = damaged;
        if damaged {
            self.limbs[limb as usize].broken = true;
        }
        if damaged && self.limbs[limb as usize].damage <= MANGLED_VALUE {
            self.limbs[limb as usize].damage = MANGLED_VALUE + 1;
        } else if !damaged && self.limbs[limb as usize].damage >= MANGLED_VALUE {
            self.limbs[limb as usize].damage = MANGLED_VALUE - 1;
        }
    }

    pub fn mangled(&self, limb: LType) -> bool {
        self.limbs[limb as usize].mangled
    }

    pub fn amputated(&self, limb: LType) -> bool {
        self.limbs[limb as usize].amputated
    }

    pub fn wait(&mut self, duration: CType) -> Option<(LType, CType, bool)> {
        if let (Some(remaining), Some(restored)) = (&mut self.restore_timer, self.restoring) {
            remaining.wait(duration);
            if !remaining.is_active() {
                self.complete_restore(None)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_damage(&self, broken: LType) -> CType {
        self.limbs[broken as usize].damage
    }

    pub fn set_limb_damage(&mut self, broken: LType, new_damage: CType, assume_break: bool) {
        self.limbs[broken as usize].damage = new_damage;
        if self.limbs[broken as usize].damage < DAMAGED_VALUE {
            self.limbs[broken as usize].broken = false;
            self.limbs[broken as usize].mangled = false;
        } else if self.limbs[broken as usize].damage < MANGLED_VALUE {
            self.limbs[broken as usize].mangled = false;
            if assume_break {
                match broken {
                    LType::TorsoDamage | LType::HeadDamage => {}
                    _ => {
                        self.limbs[broken as usize].crippled = true;
                    }
                }
                self.limbs[broken as usize].broken = true;
            }
        } else if assume_break {
            match broken {
                LType::TorsoDamage | LType::HeadDamage => {}
                _ => {
                    self.limbs[broken as usize].crippled = true;
                }
            }
            self.limbs[broken as usize].broken = true;
            self.limbs[broken as usize].mangled = true;
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

    pub fn complete_restore(&mut self, broken: Option<LType>) -> Option<(LType, CType, bool)> {
        if broken == self.restoring || broken == None {
            if let Some(broken) = self.restoring {
                let regenerating_modifier = if self.regenerating { 1500 } else { 0 };
                let fleshbane_modifier = self.fleshbaned_count as i32 * -200;
                let first_person_restore = self.first_person_restore;
                self.regenerating = false;
                self.restoring = None;
                self.restore_timer = None;
                self.fleshbaned_count = 0;
                let heal_modifier = regenerating_modifier + fleshbane_modifier;
                return Some((broken, heal_modifier, first_person_restore));
            }
            self.regenerating = false;
            self.restoring = None;
            self.restore_timer = None;
        }
        None
    }

    pub fn get_restoring_limb(&self) -> Option<LType> {
        self.restoring
    }

    pub fn restore(&mut self, limb: LType, heal_modifier: CType) {
        let expected_heal = 3000 + heal_modifier;
        let new_damage = self.limbs[limb as usize].damage
            - i32::min(self.limbs[limb as usize].damage, expected_heal);
        self.set_limb_damage(limb, new_damage, false);
    }

    pub fn start_restore(&mut self, broken: LType, first_person: bool) {
        if let Some(timer) = self.restore_timer {
            if !timer.is_active() || timer.get_time_left() < 10 {
                self.complete_restore(None);
            }
        }
        self.restoring = Some(broken);
        self.restore_timer = Some(Timer::count_down(400));
        self.first_person_restore = first_person;
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
