use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use super::*;
use crate::{bt::LimbDescriptor, types::*};

#[derive(Debug, Copy, Clone, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MonkComboAttack {
    // Kicks, valid first attacks.
    Sidekick,
    SnapkickLeft,
    SnapkickRight,
    Roundhouse,
    Sweep,
    MoonkickLeft,
    MoonkickRight,
    Cometkick,
    Scythekick,
    Axe,
    Whirlwind,
    Jumpkick,
    // Throws, valid first attacks.
    Slam,
    WrenchLeftLeg,
    WrenchRightLeg,
    WrenchLeftArm,
    WrenchRightArm,
    // Feints, valid first attacks.
    FeintLeftLeg,
    FeintRightLeg,
    FeintLeftArm,
    FeintRightArm,
    FeintHead,
    FeintTorso,
    // Punches, non-first attacks.
    Jab,
    Hook,
    Uppercut,
    Palmstrike,
    HammerfistLeft,
    HammerfistRight,
    SpearLeft,
    SpearRight,
    ThroatStrike,
    Bladehand,
}

impl MonkComboAttack {
    pub fn is_kick(self) -> bool {
        match self {
            MonkComboAttack::Sidekick
            | MonkComboAttack::SnapkickLeft
            | MonkComboAttack::SnapkickRight
            | MonkComboAttack::Roundhouse
            | MonkComboAttack::Sweep
            | MonkComboAttack::MoonkickLeft
            | MonkComboAttack::MoonkickRight
            | MonkComboAttack::Cometkick
            | MonkComboAttack::Scythekick
            | MonkComboAttack::Axe
            | MonkComboAttack::Whirlwind
            | MonkComboAttack::Jumpkick => true,
            _ => false,
        }
    }

    pub fn is_throw(self) -> bool {
        match self {
            MonkComboAttack::Slam
            | MonkComboAttack::WrenchLeftLeg
            | MonkComboAttack::WrenchRightLeg
            | MonkComboAttack::WrenchLeftArm
            | MonkComboAttack::WrenchRightArm => true,
            _ => false,
        }
    }

    pub fn is_feint(self) -> bool {
        match self {
            MonkComboAttack::FeintLeftLeg
            | MonkComboAttack::FeintRightLeg
            | MonkComboAttack::FeintLeftArm
            | MonkComboAttack::FeintRightArm
            | MonkComboAttack::FeintHead
            | MonkComboAttack::FeintTorso => true,
            _ => false,
        }
    }

    pub fn is_punch(self) -> bool {
        match self {
            MonkComboAttack::Jab
            | MonkComboAttack::Hook
            | MonkComboAttack::Uppercut
            | MonkComboAttack::Palmstrike
            | MonkComboAttack::HammerfistLeft
            | MonkComboAttack::HammerfistRight
            | MonkComboAttack::SpearLeft
            | MonkComboAttack::SpearRight
            | MonkComboAttack::ThroatStrike
            | MonkComboAttack::Bladehand => true,
            _ => false,
        }
    }

    pub fn is_first_attack(self) -> bool {
        self.is_kick() || self.is_throw() || self.is_feint()
    }

    pub fn is_non_first_attack(self) -> bool {
        self.is_punch()
    }

    pub fn param_str(self) -> &'static str {
        match self {
            MonkComboAttack::Sidekick => "sdk",
            MonkComboAttack::SnapkickLeft => "snk left",
            MonkComboAttack::SnapkickRight => "snk right",
            MonkComboAttack::Roundhouse => "rhk",
            MonkComboAttack::Sweep => "swk",
            MonkComboAttack::MoonkickLeft => "mnk left",
            MonkComboAttack::MoonkickRight => "mnk right",
            MonkComboAttack::Cometkick => "cmk",
            MonkComboAttack::Scythekick => "sck",
            MonkComboAttack::Axe => "axk",
            MonkComboAttack::Whirlwind => "wwk",
            MonkComboAttack::Jumpkick => "jpk",
            MonkComboAttack::Slam => "slam",
            MonkComboAttack::WrenchLeftLeg => "wrt left leg",
            MonkComboAttack::WrenchRightLeg => "wrt right leg",
            MonkComboAttack::WrenchLeftArm => "wrt left arm",
            MonkComboAttack::WrenchRightArm => "wrt right arm",
            MonkComboAttack::FeintLeftLeg => "feint left leg",
            MonkComboAttack::FeintRightLeg => "feint right leg",
            MonkComboAttack::FeintLeftArm => "feint left arm",
            MonkComboAttack::FeintRightArm => "feint right arm",
            MonkComboAttack::FeintHead => "feint head",
            MonkComboAttack::FeintTorso => "feint torso",
            MonkComboAttack::Jab => "jbp",
            MonkComboAttack::Hook => "hkp",
            MonkComboAttack::Uppercut => "ucp",
            MonkComboAttack::Palmstrike => "pmp",
            MonkComboAttack::HammerfistLeft => "hfp left",
            MonkComboAttack::HammerfistRight => "hfp right",
            MonkComboAttack::SpearLeft => "spp left",
            MonkComboAttack::SpearRight => "spp right",
            MonkComboAttack::ThroatStrike => "tsp",
            MonkComboAttack::Bladehand => "blp",
        }
    }

    pub fn get_limb_damage(self) -> Option<(LType, CType)> {
        match self {
            MonkComboAttack::Sidekick => Some((LType::TorsoDamage, SIDEKICK_DAMAGE)),
            MonkComboAttack::SnapkickLeft => Some((LType::LeftLegDamage, SNAPKICK_DAMAGE)),
            MonkComboAttack::SnapkickRight => Some((LType::RightLegDamage, SNAPKICK_DAMAGE)),
            MonkComboAttack::MoonkickLeft => Some((LType::LeftArmDamage, MOONKICK_DAMAGE)),
            MonkComboAttack::MoonkickRight => Some((LType::RightArmDamage, MOONKICK_DAMAGE)),
            MonkComboAttack::Whirlwind => Some((LType::HeadDamage, WHIRLWIND_DAMAGE)),
            MonkComboAttack::Hook => Some((LType::TorsoDamage, HOOK_DAMAGE)),
            MonkComboAttack::Uppercut => Some((LType::HeadDamage, UPPERCUT_DAMAGE)),
            MonkComboAttack::Palmstrike => Some((LType::HeadDamage, PALMSTRIKE_DAMAGE)),
            MonkComboAttack::HammerfistLeft => Some((LType::LeftLegDamage, HAMMERFIST_DAMAGE)),
            MonkComboAttack::HammerfistRight => Some((LType::RightLegDamage, HAMMERFIST_DAMAGE)),
            MonkComboAttack::SpearLeft => Some((LType::LeftArmDamage, SPEAR_DAMAGE)),
            MonkComboAttack::SpearRight => Some((LType::RightArmDamage, SPEAR_DAMAGE)),
            _ => None,
        }
    }

    pub fn is_idempotent(self) -> bool {
        match self {
            MonkComboAttack::ThroatStrike
            | MonkComboAttack::Palmstrike
            | MonkComboAttack::Bladehand => true,
            _ => self.is_first_attack(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum MonkCombo {
    Standard(MonkStance, [MonkComboAttack; 3]),
    ChangeStance(MonkStance, [MonkComboAttack; 2]),
    Cobra([MonkComboAttack; 2]),
}

impl MonkCombo {
    pub fn new(stance: MonkStance, attacks: [MonkComboAttack; 3]) -> Self {
        MonkCombo::Standard(stance, attacks)
    }

    pub fn new_change_stance(stance: MonkStance, attacks: [MonkComboAttack; 2]) -> Self {
        MonkCombo::ChangeStance(stance, attacks)
    }

    pub fn new_cobra(attacks: [MonkComboAttack; 2]) -> Self {
        MonkCombo::Cobra(attacks)
    }
}

#[derive(Debug, Default)]
pub struct MonkComboGenerator {
    valid_attacks: Vec<MonkComboAttack>,
}

impl MonkComboGenerator {
    pub fn new(stance: MonkStance) -> Self {
        MonkComboGenerator {
            ..Default::default()
        }
    }

    pub fn get_valid_attacks(&self) -> &[MonkComboAttack] {
        &self.valid_attacks
    }

    pub fn set_valid_attacks(&mut self, valid_attacks: Vec<MonkComboAttack>) {
        self.valid_attacks = valid_attacks;
    }

    pub fn generate_from_stance(&self, stance: MonkStance) -> Vec<MonkCombo> {
        let mut combos = Vec::new();
        for first_attack in self.valid_attacks.iter().filter(|a| a.is_first_attack()) {
            for second_attack in self
                .valid_attacks
                .iter()
                .filter(|a| a.is_non_first_attack())
            {
                for third_attack in self
                    .valid_attacks
                    .iter()
                    .filter(|a| a.is_non_first_attack())
                {
                    if second_attack == third_attack && !second_attack.is_idempotent() {
                        continue;
                    }
                    combos.push(MonkCombo::new(
                        stance,
                        [*first_attack, *second_attack, *third_attack],
                    ));
                }
            }
        }
        combos
    }

    pub fn generate_with_stance_change(&self, stance: MonkStance) -> Vec<MonkCombo> {
        let mut combos = Vec::new();
        for second_attack in self
            .valid_attacks
            .iter()
            .filter(|a| a.is_non_first_attack())
        {
            for third_attack in self
                .valid_attacks
                .iter()
                .filter(|a| a.is_non_first_attack())
            {
                if second_attack == third_attack && !second_attack.is_idempotent() {
                    continue;
                }
                combos.push(MonkCombo::new_change_stance(
                    stance,
                    [*second_attack, *third_attack],
                ));
            }
        }
        combos
    }

    pub fn generate_cobra_combo(&self) -> Vec<MonkCombo> {
        let mut combos = Vec::new();
        for first_attack in self.valid_attacks.iter().filter(|a| a.is_kick()) {
            for second_attack in self.valid_attacks.iter().filter(|a| a.is_kick()) {
                if second_attack == first_attack && !first_attack.is_idempotent() {
                    continue;
                }
                combos.push(MonkCombo::new_cobra([*first_attack, *second_attack]));
            }
        }
        combos
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MonkComboGrader {
    HitsLastParry,
    DoubleParryable,                          // Can get parried twice.
    Breaks(i32, Vec<LimbDescriptor>),         // Will break a number of limbs
    IntoRange(i32, f32, Vec<LimbDescriptor>), // Will put a number of limbs into range
    InStance(MonkStance),
}

#[derive(Debug, Default)]
pub struct MonkComboSet {
    combos: Vec<MonkCombo>,
}

impl MonkComboSet {
    pub fn new() -> Self {
        MonkComboSet { combos: Vec::new() }
    }

    pub fn get_combos(&self) -> &[MonkCombo] {
        &self.combos
    }

    pub fn add_combo(&mut self, combo: MonkCombo) {
        self.combos.push(combo);
    }
}
