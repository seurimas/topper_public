use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use super::*;
use crate::types::*;

#[derive(Debug, Copy, Clone, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComboAttack {
    Tidalslash,
    Freefall,
    Pheromones,
    Pindown,
    Mindnumb,
    Jab,
    Pinprick,
    Lateral,
    Vertical,
    Crescentcut,
    Spinslash,
    Lowhook,
    Butterfly,
    Flashkick,
    Trip,
    Veinrip,
    Feint,
    Raze,
    Gouge,
    Bleed,
    Swiftkick,
}

impl ComboAttack {
    pub fn get_crescentcut_damage(agent: &AgentState) -> f32 {
        let mut damage = 1.;
        if agent.is(FType::Fallen) {
            damage += 0.25;
        }
        if agent.is(FType::Paresis) {
            damage += 0.15;
        }
        if agent.is(FType::Shock) {
            damage += 0.40;
        }
        damage += agent.affs_count(&vec![
            FType::LeftLegCrippled,
            FType::LeftArmCrippled,
            FType::RightLegCrippled,
            FType::RightArmCrippled,
        ]) as f32
            * 0.1;
        damage += agent.affs_count(&vec![
            FType::LeftLegBroken,
            FType::LeftArmBroken,
            FType::RightLegBroken,
            FType::RightArmBroken,
        ]) as f32
            * 0.35;
        damage += agent.affs_count(&vec![FType::HeadBroken, FType::TorsoBroken]) as f32 * 0.3;
        damage += agent.affs_count(&vec![FType::HeadMangled, FType::TorsoMangled]) as f32 * 0.5;
        damage
    }

    pub fn is_combo_attack(&self) -> bool {
        match self {
            ComboAttack::Tidalslash => false,
            ComboAttack::Freefall => false,
            ComboAttack::Pheromones => false,
            ComboAttack::Pindown => false,
            ComboAttack::Mindnumb => false,
            _ => true,
        }
    }

    pub fn rebounds(&self) -> bool {
        match self {
            ComboAttack::Freefall => false,
            ComboAttack::Pheromones => false,
            ComboAttack::Pindown => false,
            ComboAttack::Mindnumb => false,
            ComboAttack::Raze => false,
            _ => true,
        }
    }

    pub fn strips_rebounding(&self) -> bool {
        match self {
            ComboAttack::Raze => true,
            _ => false,
        }
    }

    pub fn get_aff_count(&self) -> usize {
        match self {
            ComboAttack::Pinprick => 1,
            ComboAttack::Flashkick => 1,
            ComboAttack::Veinrip => 2,
            ComboAttack::Gouge => 1,
            ComboAttack::Pheromones => 1,
            ComboAttack::Mindnumb => 1,
            ComboAttack::Trip => 1,
            _ => 0,
        }
    }

    // Combo attacks where using it twice is impossible or an anti-pattern.
    pub fn idempotent(&self) -> bool {
        match self {
            ComboAttack::Tidalslash => true,
            ComboAttack::Freefall => true,
            ComboAttack::Pheromones => true,
            ComboAttack::Pindown => true,
            ComboAttack::Mindnumb => true,
            ComboAttack::Pinprick => true,
            ComboAttack::Veinrip => true,
            ComboAttack::Feint => true,
            _ => false,
        }
    }

    pub fn parryable(&self) -> bool {
        match self {
            ComboAttack::Jab
            | ComboAttack::Lateral
            | ComboAttack::Lowhook
            | ComboAttack::Flashkick
            | ComboAttack::Veinrip
            | ComboAttack::Gouge => true,
            _ => false,
        }
    }

    pub fn get_single_limb_target(&self) -> Option<LType> {
        match self {
            ComboAttack::Lateral => Some(LType::TorsoDamage),
            ComboAttack::Flashkick => Some(LType::HeadDamage),
            ComboAttack::Veinrip => Some(LType::HeadDamage),
            ComboAttack::Gouge => Some(LType::HeadDamage),
            _ => None,
        }
    }

    pub fn can_hit(&self, limb: LType) -> bool {
        if Some(limb) == self.get_single_limb_target() {
            true
        } else {
            match (self, limb) {
                (ComboAttack::Jab, LType::LeftArmDamage) => true,
                (ComboAttack::Jab, LType::RightArmDamage) => true,
                (ComboAttack::Lowhook, LType::LeftArmDamage) => true,
                (ComboAttack::Lowhook, LType::RightArmDamage) => true,
                (ComboAttack::Spinslash, _) => true,
                _ => false,
            }
        }
    }

    pub fn get_limb_damage(&self) -> CType {
        match self {
            ComboAttack::Lateral => 600,
            ComboAttack::Flashkick => 500,
            ComboAttack::Veinrip => 200,
            ComboAttack::Gouge => 650,
            ComboAttack::Lowhook => 550,
            ComboAttack::Jab => 550,
            ComboAttack::Spinslash => 400,
            _ => 0,
        }
    }

    pub fn can_drop_parry(&self) -> bool {
        if self == &ComboAttack::Feint || self == &ComboAttack::Pindown {
            true
        } else {
            false
        }
    }

    pub fn requires_prone(&self) -> bool {
        match self {
            ComboAttack::Pindown => true,
            _ => false,
        }
    }

    pub fn can_prone(&self) -> bool {
        match self {
            ComboAttack::Trip => true,
            _ => false,
        }
    }

    pub fn can_use_venom(&self) -> bool {
        match self {
            ComboAttack::Freefall
            | ComboAttack::Vertical
            | ComboAttack::Crescentcut
            | ComboAttack::Butterfly => true,
            _ => false,
        }
    }

    pub fn is_good_combo_attack(&self, stance: KnifeStance) -> bool {
        if *self == ComboAttack::Raze || stance == KnifeStance::Bladesurge {
            return true;
        }
        !self.is_combo_attack() || self.get_next_stance(stance) != stance
    }

    pub fn must_begin_combo(&self) -> bool {
        match self {
            ComboAttack::Freefall => true,
            ComboAttack::Pindown => true,
            ComboAttack::Butterfly => true,
            _ => false,
        }
    }

    pub fn should_end_combo(&self) -> bool {
        match self {
            ComboAttack::Feint => false,
            _ => true,
        }
    }

    pub fn get_balance_time(&self, stance: KnifeStance) -> CType {
        let base = match self {
            ComboAttack::Jab => 205,
            ComboAttack::Pinprick => 205,
            ComboAttack::Lateral => 205,
            ComboAttack::Lowhook => 205,
            ComboAttack::Feint => 205,
            ComboAttack::Raze => 205,
            ComboAttack::Pheromones => 205,
            ComboAttack::Mindnumb => 251,
            ComboAttack::Vertical => 251,
            ComboAttack::Spinslash => 251,
            ComboAttack::Trip => 298,
            ComboAttack::Gouge => 298,
            ComboAttack::Bleed => 344,
            ComboAttack::Swiftkick => 344,
            ComboAttack::Crescentcut => 367,
            ComboAttack::Pindown => 372,
            ComboAttack::Tidalslash => 391,
            ComboAttack::Butterfly => 391,
            ComboAttack::Freefall => 391,
            ComboAttack::Flashkick => 391,
            ComboAttack::Veinrip => 391,
        };
        if !self.is_good_combo_attack(stance) {
            base + 98
        } else if stance == KnifeStance::VaeSant || stance == KnifeStance::Bladesurge {
            base - 40
        } else {
            base
        }
    }

    pub fn get_stance_damage(&self, stance: KnifeStance) -> CType {
        let base_damage = match self {
            ComboAttack::Jab => 240,
            ComboAttack::Lowhook => 240,
            ComboAttack::Pinprick => 80,
            ComboAttack::Lateral => 230,
            ComboAttack::Spinslash => 350,
            ComboAttack::Vertical => 475,
            ComboAttack::Gouge => 333,
            ComboAttack::Bleed => 133,
            // Gets bonuses elsewhere.
            ComboAttack::Crescentcut => 470,
            ComboAttack::Veinrip => 170,
            ComboAttack::Swiftkick => {
                // Unmodified by stance.
                return 240;
            }
            ComboAttack::Flashkick => {
                // Unmodified by stance.
                return 575;
            }
            _ => 0,
        };
        if stance == KnifeStance::Rizet || stance == KnifeStance::Bladesurge {
            base_damage * 5 / 4
        } else {
            base_damage
        }
    }

    pub fn get_next_stance(&self, stance: KnifeStance) -> KnifeStance {
        match (self, stance) {
            // Bladesurge stays in bladesurge.
            (_, KnifeStance::Bladesurge) => KnifeStance::Bladesurge,
            // Non knifeplay attacks.
            (ComboAttack::Tidalslash, _) => stance,
            (ComboAttack::Freefall, _) => stance,
            (ComboAttack::Pheromones, _) => stance,
            (ComboAttack::Pindown, _) => stance,
            (ComboAttack::Mindnumb, _) => stance,
            // Jab
            (ComboAttack::Jab, KnifeStance::None) => KnifeStance::Gyanis,
            (ComboAttack::Jab, KnifeStance::Gyanis) => KnifeStance::Rizet,
            (ComboAttack::Jab, KnifeStance::VaeSant) => KnifeStance::Gyanis,
            (ComboAttack::Jab, KnifeStance::Rizet) => stance,
            (ComboAttack::Jab, KnifeStance::EinFasit) => KnifeStance::VaeSant,
            (ComboAttack::Jab, KnifeStance::Laesan) => KnifeStance::Rizet,
            // Pinprick
            (ComboAttack::Pinprick, KnifeStance::None) => KnifeStance::Gyanis,
            (ComboAttack::Pinprick, KnifeStance::Gyanis) => KnifeStance::Rizet,
            (ComboAttack::Pinprick, KnifeStance::VaeSant) => KnifeStance::Rizet,
            (ComboAttack::Pinprick, KnifeStance::Rizet) => KnifeStance::VaeSant,
            (ComboAttack::Pinprick, KnifeStance::EinFasit) => stance,
            (ComboAttack::Pinprick, KnifeStance::Laesan) => KnifeStance::Gyanis,
            // Lateral
            (ComboAttack::Lateral, KnifeStance::None) => KnifeStance::Gyanis,
            (ComboAttack::Lateral, KnifeStance::Gyanis) => KnifeStance::VaeSant,
            (ComboAttack::Lateral, KnifeStance::VaeSant) => KnifeStance::EinFasit,
            (ComboAttack::Lateral, KnifeStance::Rizet) => KnifeStance::EinFasit,
            (ComboAttack::Lateral, KnifeStance::EinFasit) => KnifeStance::Laesan,
            (ComboAttack::Lateral, KnifeStance::Laesan) => stance,
            // Vertical
            (ComboAttack::Vertical, KnifeStance::None) => KnifeStance::Laesan,
            (ComboAttack::Vertical, KnifeStance::Gyanis) => KnifeStance::Laesan,
            (ComboAttack::Vertical, KnifeStance::VaeSant) => KnifeStance::Rizet,
            (ComboAttack::Vertical, KnifeStance::Rizet) => KnifeStance::EinFasit,
            (ComboAttack::Vertical, KnifeStance::EinFasit) => KnifeStance::VaeSant,
            (ComboAttack::Vertical, KnifeStance::Laesan) => stance,
            // Crescentcut
            (ComboAttack::Crescentcut, KnifeStance::None) => KnifeStance::VaeSant,
            (ComboAttack::Crescentcut, KnifeStance::Gyanis) => KnifeStance::EinFasit,
            (ComboAttack::Crescentcut, KnifeStance::VaeSant) => stance,
            (ComboAttack::Crescentcut, KnifeStance::Rizet) => KnifeStance::Laesan,
            (ComboAttack::Crescentcut, KnifeStance::EinFasit) => KnifeStance::Gyanis,
            (ComboAttack::Crescentcut, KnifeStance::Laesan) => KnifeStance::VaeSant,
            // Spinslash
            (ComboAttack::Spinslash, KnifeStance::None) => KnifeStance::VaeSant,
            (ComboAttack::Spinslash, KnifeStance::Gyanis) => KnifeStance::VaeSant,
            (ComboAttack::Spinslash, KnifeStance::VaeSant) => KnifeStance::EinFasit,
            (ComboAttack::Spinslash, KnifeStance::Rizet) => KnifeStance::Laesan,
            (ComboAttack::Spinslash, KnifeStance::EinFasit) => stance,
            (ComboAttack::Spinslash, KnifeStance::Laesan) => KnifeStance::EinFasit,
            // Lowhook
            (ComboAttack::Lowhook, KnifeStance::None) => KnifeStance::VaeSant,
            (ComboAttack::Lowhook, KnifeStance::Gyanis) => KnifeStance::VaeSant,
            (ComboAttack::Lowhook, KnifeStance::VaeSant) => KnifeStance::Gyanis,
            (ComboAttack::Lowhook, KnifeStance::Rizet) => stance,
            (ComboAttack::Lowhook, KnifeStance::EinFasit) => KnifeStance::Gyanis,
            (ComboAttack::Lowhook, KnifeStance::Laesan) => KnifeStance::Gyanis,
            // Butterfly
            (ComboAttack::Butterfly, KnifeStance::None) => KnifeStance::Rizet,
            (ComboAttack::Butterfly, KnifeStance::Gyanis) => stance,
            (ComboAttack::Butterfly, KnifeStance::VaeSant) => KnifeStance::Gyanis,
            (ComboAttack::Butterfly, KnifeStance::Rizet) => KnifeStance::Gyanis,
            (ComboAttack::Butterfly, KnifeStance::EinFasit) => KnifeStance::Laesan,
            (ComboAttack::Butterfly, KnifeStance::Laesan) => KnifeStance::Rizet,
            // Flashkick
            (ComboAttack::Flashkick, KnifeStance::None) => KnifeStance::Rizet,
            (ComboAttack::Flashkick, KnifeStance::Gyanis) => KnifeStance::Rizet,
            (ComboAttack::Flashkick, KnifeStance::VaeSant) => KnifeStance::Laesan,
            (ComboAttack::Flashkick, KnifeStance::Rizet) => stance,
            (ComboAttack::Flashkick, KnifeStance::EinFasit) => KnifeStance::Laesan,
            (ComboAttack::Flashkick, KnifeStance::Laesan) => KnifeStance::VaeSant,
            // Trip
            (ComboAttack::Trip, KnifeStance::None) => KnifeStance::EinFasit,
            (ComboAttack::Trip, KnifeStance::Gyanis) => KnifeStance::VaeSant,
            (ComboAttack::Trip, KnifeStance::VaeSant) => stance,
            (ComboAttack::Trip, KnifeStance::Rizet) => KnifeStance::Gyanis,
            (ComboAttack::Trip, KnifeStance::EinFasit) => KnifeStance::Gyanis,
            (ComboAttack::Trip, KnifeStance::Laesan) => KnifeStance::Rizet,
            // Veinrip
            (ComboAttack::Veinrip, KnifeStance::None) => stance,
            (ComboAttack::Veinrip, KnifeStance::Gyanis) => KnifeStance::EinFasit,
            (ComboAttack::Veinrip, KnifeStance::VaeSant) => KnifeStance::EinFasit,
            (ComboAttack::Veinrip, KnifeStance::Rizet) => KnifeStance::Gyanis,
            (ComboAttack::Veinrip, KnifeStance::EinFasit) => KnifeStance::Laesan,
            (ComboAttack::Veinrip, KnifeStance::Laesan) => KnifeStance::VaeSant,
            // Feint
            (ComboAttack::Feint, KnifeStance::None) => KnifeStance::EinFasit,
            (ComboAttack::Feint, KnifeStance::Gyanis) => KnifeStance::EinFasit,
            (ComboAttack::Feint, KnifeStance::VaeSant) => KnifeStance::Laesan,
            (ComboAttack::Feint, KnifeStance::Rizet) => stance,
            (ComboAttack::Feint, KnifeStance::EinFasit) => KnifeStance::Gyanis,
            (ComboAttack::Feint, KnifeStance::Laesan) => KnifeStance::EinFasit,
            // Raze
            (ComboAttack::Raze, KnifeStance::None) => KnifeStance::Laesan,
            (ComboAttack::Raze, KnifeStance::Gyanis) => KnifeStance::Laesan,
            (ComboAttack::Raze, KnifeStance::VaeSant) => stance,
            (ComboAttack::Raze, KnifeStance::Rizet) => KnifeStance::VaeSant,
            (ComboAttack::Raze, KnifeStance::EinFasit) => KnifeStance::Rizet,
            (ComboAttack::Raze, KnifeStance::Laesan) => KnifeStance::EinFasit,
            // Gouge
            (ComboAttack::Gouge, KnifeStance::None) => KnifeStance::Laesan,
            (ComboAttack::Gouge, KnifeStance::Gyanis) => KnifeStance::EinFasit,
            (ComboAttack::Gouge, KnifeStance::VaeSant) => KnifeStance::Gyanis,
            (ComboAttack::Gouge, KnifeStance::Rizet) => KnifeStance::VaeSant,
            (ComboAttack::Gouge, KnifeStance::EinFasit) => KnifeStance::Rizet,
            (ComboAttack::Gouge, KnifeStance::Laesan) => stance,
            // Bleed
            (ComboAttack::Bleed, KnifeStance::None) => stance,
            (ComboAttack::Bleed, KnifeStance::Gyanis) => KnifeStance::Laesan,
            (ComboAttack::Bleed, KnifeStance::VaeSant) => KnifeStance::Rizet,
            (ComboAttack::Bleed, KnifeStance::Rizet) => KnifeStance::EinFasit,
            (ComboAttack::Bleed, KnifeStance::EinFasit) => stance,
            (ComboAttack::Bleed, KnifeStance::Laesan) => KnifeStance::VaeSant,
            // Swiftkick
            (ComboAttack::Swiftkick, KnifeStance::None) => KnifeStance::Gyanis,
            (ComboAttack::Swiftkick, KnifeStance::Gyanis) => KnifeStance::Laesan,
            (ComboAttack::Swiftkick, KnifeStance::VaeSant) => KnifeStance::EinFasit,
            (ComboAttack::Swiftkick, KnifeStance::Rizet) => stance,
            (ComboAttack::Swiftkick, KnifeStance::EinFasit) => KnifeStance::VaeSant,
            (ComboAttack::Swiftkick, KnifeStance::Laesan) => KnifeStance::Rizet,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct PredatorCombo(KnifeStance, Vec<ComboAttack>);

impl PredatorCombo {
    pub fn new(stance: KnifeStance, attacks: Vec<ComboAttack>) -> Self {
        Self(stance, attacks)
    }

    pub fn get_attacks(&self) -> &Vec<ComboAttack> {
        &self.1
    }

    pub fn get_starting_stance(&self) -> KnifeStance {
        self.0
    }

    pub fn has_venom(&self) -> bool {
        self.1.iter().any(|attack| attack.can_use_venom())
    }

    pub fn get_final_stance(&self) -> KnifeStance {
        self.1
            .iter()
            .fold(self.0, |stance, attack| attack.get_next_stance(stance))
    }

    pub fn get_balance_time(&self) -> CType {
        self.1
            .iter()
            .fold((self.0, 0), |(stance, balance), attack| {
                (
                    attack.get_next_stance(stance),
                    CType::max(balance, attack.get_balance_time(stance)),
                )
            })
            .1
    }

    pub fn estimate_damage(&self, start_fallen: bool, crescentcut_value: f32) -> CType {
        self.1
            .iter()
            .fold(
                (self.0, start_fallen, 0),
                |(stance, prone, damage), attack| {
                    let mut modded_value = crescentcut_value;
                    if !start_fallen && prone {
                        modded_value += 0.25;
                    }
                    (
                        attack.get_next_stance(stance),
                        prone || attack.can_prone(),
                        damage
                            + if *attack == ComboAttack::Crescentcut {
                                (attack.get_stance_damage(stance) as f32 * modded_value) as CType
                            } else {
                                attack.get_stance_damage(stance)
                            },
                    )
                },
            )
            .2
    }

    pub fn estimate_aff_rate(&self) -> f32 {
        let balance = self.get_balance_time();
        let mut affs = self
            .1
            .iter()
            .fold(0, |affs, attack| affs + attack.get_aff_count());
        if self.has_venom() {
            affs += 1;
        }
        affs as f32 / balance as f32
    }

    pub fn estimate_dps(&self, start_fallen: bool, crescentcut_value: f32) -> f32 {
        let damage = self.estimate_damage(start_fallen, crescentcut_value);
        let balance = self.get_balance_time();
        damage as f32 / balance as f32
    }

    pub fn score_combo(&self, graders: &Vec<ComboGrader>, start_parrying: Option<LType>) -> i32 {
        graders.iter().fold(0, |score, grader| {
            score + grader.grade(self, start_parrying)
        })
    }
}

#[derive(Debug)]
pub struct ComboSolver {
    attacks: Vec<ComboAttack>,
    starting_stance: KnifeStance,
    start_parry: bool,
    start_prone: bool,
    start_rebounds: u32,
    blade_surge: bool,
    allow_bad_stances: bool,
    allow_parries: bool,
}

impl Default for ComboSolver {
    fn default() -> Self {
        Self::new(KnifeStance::None)
    }
}

impl ComboSolver {
    pub fn new(stance: KnifeStance) -> Self {
        Self {
            attacks: vec![],
            starting_stance: stance,
            start_parry: false,
            start_prone: false,
            start_rebounds: 0,
            blade_surge: false,
            allow_bad_stances: false,
            allow_parries: false,
        }
    }

    pub fn set_stance(&mut self, stance: KnifeStance) -> &mut Self {
        self.starting_stance = stance;
        self
    }

    pub fn set_attacks(&mut self, attacks: Vec<ComboAttack>) -> &mut Self {
        self.attacks = attacks;
        self
    }

    pub fn add_attacks<'a>(&mut self, attacks: impl Iterator<Item = &'a ComboAttack>) {
        self.attacks.extend(attacks);
    }

    pub fn set_parry(&mut self, parry: bool) -> &mut Self {
        self.start_parry = parry;
        self
    }

    pub fn set_prone(&mut self, prone: bool) -> &mut Self {
        self.start_prone = prone;
        self
    }

    pub fn set_rebounds(&mut self, rebounds: u32) -> &mut Self {
        self.start_rebounds = rebounds;
        self
    }

    pub fn set_blade_surge(&mut self, blade_surge: bool) -> &mut Self {
        self.blade_surge = blade_surge;
        self
    }

    pub fn set_allow_bad_stances(&mut self, allow_bad_stances: bool) -> &mut Self {
        self.allow_bad_stances = allow_bad_stances;
        self
    }

    pub fn set_allow_parries(&mut self, allow_parries: bool) -> &mut Self {
        self.allow_parries = allow_parries;
        self
    }

    fn add_combos(
        &self,
        combos: &mut Vec<PredatorCombo>,
        balance_time: CType,
        current_stance: KnifeStance,
        attack: ComboAttack,
        previous_attacks: Vec<ComboAttack>,
        mut parrying: bool,
        mut prone: bool,
        mut rebounds: u32,
    ) {
        if combos.len() > 1000 {
            return;
        }
        let next_stance = attack.get_next_stance(current_stance);
        let mut new_attacks = previous_attacks.clone();
        new_attacks.push(attack);
        if attack.should_end_combo() {
            combos.push(PredatorCombo::new(
                self.starting_stance,
                new_attacks.clone(),
            ));
        }
        if new_attacks.len() == 3
            && next_stance != KnifeStance::Laesan
            && next_stance != KnifeStance::Bladesurge
        {
            return;
        } else if new_attacks.len() == 4 {
            return;
        }
        parrying &= !attack.can_drop_parry();
        prone |= attack.can_prone();
        if attack.strips_rebounding() && rebounds > 0 {
            rebounds -= 1;
        }
        for next_attack in self.attacks.iter() {
            self.add_next_attack(
                combos,
                balance_time,
                next_attack,
                next_stance,
                &new_attacks,
                parrying,
                prone,
                rebounds,
            );
        }
    }

    fn add_next_attack(
        &self,
        combos: &mut Vec<PredatorCombo>,
        balance_time: CType,
        next_attack: &ComboAttack,
        next_stance: KnifeStance,
        new_attacks: &Vec<ComboAttack>,
        parrying: bool,
        prone: bool,
        rebounds: u32,
    ) {
        if (self.allow_bad_stances
            || next_attack.is_good_combo_attack(next_stance)
            || next_attack.get_balance_time(next_stance) <= balance_time)
            && (new_attacks.len() == 0 || !next_attack.must_begin_combo())
            && (!next_attack.idempotent() || !new_attacks.contains(&next_attack))
            && (!parrying || !next_attack.parryable() || self.allow_parries)
            && (prone || !next_attack.requires_prone())
            && (rebounds == 0 || !next_attack.rebounds())
        {
            self.add_combos(
                combos,
                balance_time.max(next_attack.get_balance_time(next_stance)),
                next_stance,
                *next_attack,
                new_attacks.clone(),
                parrying,
                prone,
                rebounds,
            );
        }
    }

    pub fn find_combos(&self) -> ComboSet {
        let mut combos = vec![];
        for attack in self.attacks.iter() {
            self.add_next_attack(
                &mut combos,
                0,
                attack,
                self.starting_stance,
                &vec![],
                self.start_parry,
                self.start_prone,
                self.start_rebounds,
            );
        }
        ComboSet(combos)
    }
}

#[derive(Debug, Default)]
pub struct ComboSet(Vec<PredatorCombo>);

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum ComboPredicate {
    HasVenom,
    WithAttack(ComboAttack),
    EndsInStance(KnifeStance),
    MinimumAttacks(usize),
    MaxBalanceTime(CType),
    ScoreOver(i32),
}

impl ComboPredicate {
    pub fn matches(&self, combo: &PredatorCombo, score: Option<i32>) -> bool {
        match self {
            ComboPredicate::HasVenom => combo.has_venom(),
            ComboPredicate::WithAttack(attack) => combo.get_attacks().contains(attack),
            ComboPredicate::EndsInStance(stance) => {
                combo.0 == KnifeStance::Bladesurge || combo.get_final_stance() == *stance
            }
            ComboPredicate::MinimumAttacks(minimum) => combo.get_attacks().len() >= *minimum,
            ComboPredicate::MaxBalanceTime(max_balance) => combo.get_balance_time() <= *max_balance,
            ComboPredicate::ScoreOver(min_score) => {
                if let Some(score) = score {
                    score >= *min_score
                } else {
                    false
                }
            }
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum ComboGrader {
    Reuse(i32),
    Hits(LType, i32),
    ValueMove(ComboAttack, i32, i32),
    ValueMoveUnparried(ComboAttack, i32, i32),
    ValueMoveInStance(ComboAttack, KnifeStance, i32),
    HasVenom(i32),
    EndsInStance(KnifeStance, i32),
}

impl ComboGrader {
    pub fn grade(&self, combo: &PredatorCombo, start_parrying: Option<LType>) -> i32 {
        match self {
            ComboGrader::Reuse(value) => {
                let mut seen_hits = vec![];
                for attack in combo.get_attacks().iter() {
                    if seen_hits.contains(&attack) {
                        return *value;
                    } else {
                        seen_hits.push(attack);
                    }
                }
                0
            }
            ComboGrader::Hits(limb, value) => {
                let mut total_value = 0;
                for attack in combo.get_attacks().iter() {
                    if attack.can_hit(*limb) {
                        total_value += *value;
                    }
                }
                total_value
            }
            ComboGrader::ValueMove(attack, first_value, rest_value) => {
                combo
                    .get_attacks()
                    .iter()
                    .fold((0, false), |(total, seen), combo_attack| {
                        if combo_attack == attack {
                            if seen {
                                (total + *rest_value, seen)
                            } else {
                                (total + *first_value, true)
                            }
                        } else {
                            (total, seen)
                        }
                    })
                    .0
            }
            ComboGrader::ValueMoveUnparried(attack, unparried_value, off_limb) => {
                combo
                    .get_attacks()
                    .iter()
                    .fold(
                        (0, start_parrying),
                        |(total, mut parrying), combo_attack| {
                            if combo_attack.can_drop_parry() {
                                parrying = None;
                            }
                            if parrying.is_none() && combo_attack == attack {
                                (total + *unparried_value, parrying)
                            } else if parrying.is_some()
                                && combo_attack == attack
                                && !attack.can_hit(parrying.unwrap())
                            {
                                (total + *off_limb, parrying)
                            } else {
                                (total, parrying)
                            }
                        },
                    )
                    .0
            }
            ComboGrader::ValueMoveInStance(attack, stance, value) => {
                combo
                    .get_attacks()
                    .iter()
                    .fold((combo.0, 0), |(current_stance, total), combo_attack| {
                        if combo_attack == attack {
                            if combo.0 == KnifeStance::Bladesurge || combo.0 == *stance {
                                (combo_attack.get_next_stance(current_stance), total + *value)
                            } else {
                                (combo_attack.get_next_stance(current_stance), total)
                            }
                        } else {
                            (combo_attack.get_next_stance(current_stance), total)
                        }
                    })
                    .1
            }
            ComboGrader::HasVenom(value) => {
                if combo.has_venom() {
                    *value
                } else {
                    0
                }
            }
            ComboGrader::EndsInStance(stance, value) => {
                if combo.0 == KnifeStance::Bladesurge || combo.get_final_stance() == *stance {
                    *value
                } else {
                    0
                }
            }
        }
    }
}

impl ComboSet {
    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn get_fastest_combo(&self, predicates: &Vec<ComboPredicate>) -> Option<PredatorCombo> {
        let mut fastest_combo = None;
        let mut fastest_time = CType::max_value();
        for combo in self.0.iter() {
            let mut valid = true;
            for predicate in predicates.iter() {
                if !predicate.matches(combo, None) {
                    valid = false;
                    break;
                }
            }
            if valid {
                let balance_time = combo.get_balance_time();
                if balance_time < fastest_time {
                    fastest_time = balance_time;
                    fastest_combo = Some(combo);
                }
            }
        }
        fastest_combo.cloned()
    }

    pub fn get_highest_aff_rate_combo(
        &self,
        predicates: &Vec<ComboPredicate>,
    ) -> Option<PredatorCombo> {
        let mut highest_combo = None;
        let mut highest_aff_rate = 0.0;
        for combo in self.0.iter() {
            let mut valid = true;
            for predicate in predicates.iter() {
                if !predicate.matches(combo, None) {
                    valid = false;
                    break;
                }
            }
            if valid {
                let aff_rate = combo.estimate_aff_rate();
                if aff_rate > highest_aff_rate {
                    highest_aff_rate = aff_rate;
                    highest_combo = Some(combo);
                }
            }
        }
        highest_combo.cloned()
    }

    pub fn get_highest_dps_combo(
        &self,
        predicates: &Vec<ComboPredicate>,
        start_fallen: bool,
        crescentcut_value: f32,
    ) -> Option<PredatorCombo> {
        let mut highest_combo = None;
        let mut highest_dps = 0.0;
        for combo in self.0.iter() {
            let mut valid = true;
            for predicate in predicates.iter() {
                if !predicate.matches(combo, None) {
                    valid = false;
                    break;
                }
            }
            if valid {
                let dps = combo.estimate_dps(start_fallen, crescentcut_value);
                if dps > highest_dps {
                    highest_dps = dps;
                    highest_combo = Some(combo);
                }
            }
        }
        highest_combo.cloned()
    }

    pub fn get_highest_scored_combo(
        &self,
        predicates: &Vec<ComboPredicate>,
        base_graders: &Vec<ComboGrader>,
        graders: &Vec<ComboGrader>,
        start_parrying: Option<LType>,
    ) -> Option<PredatorCombo> {
        let mut highest_combo = None;
        let mut highest_score = 0.0;
        for combo in self.0.iter() {
            let mut valid = true;
            let score = combo.score_combo(base_graders, start_parrying)
                + combo.score_combo(graders, start_parrying);
            for predicate in predicates.iter() {
                if !predicate.matches(combo, Some(score)) {
                    valid = false;
                    break;
                }
            }
            if valid {
                let balance_score = score as f32 / (combo.get_balance_time() as f32);
                if balance_score > highest_score {
                    highest_score = balance_score;
                    highest_combo = Some(combo);
                }
            }
        }
        highest_combo.cloned()
    }
}

mod predator_tests {
    use super::*;

    #[test]
    pub fn test_find_combos() {
        let mut solver = ComboSolver::new(KnifeStance::Rizet);
        solver
            .set_attacks(vec![
                ComboAttack::Jab,
                ComboAttack::Pinprick,
                ComboAttack::Mindnumb,
                ComboAttack::Vertical,
                ComboAttack::Veinrip,
                ComboAttack::Lowhook,
                ComboAttack::Pheromones,
                ComboAttack::Gouge,
                ComboAttack::Trip,
                ComboAttack::Raze,
            ])
            .set_prone(false)
            .set_parry(false)
            .set_rebounds(0);
        let combos = solver.find_combos();
        assert_eq!(combos.0.len(), 921);
        for combo in combos.0.iter() {
            println!("{:?}", combo);
        }
        assert!(combos.0.contains(
            (&PredatorCombo::new(
                KnifeStance::Rizet,
                vec![
                    ComboAttack::Pinprick,
                    ComboAttack::Pheromones,
                    ComboAttack::Vertical,
                ]
            ))
        ));
        assert!(combos.0.contains(
            (&PredatorCombo::new(
                KnifeStance::Rizet,
                vec![
                    ComboAttack::Raze,
                    ComboAttack::Gouge,
                    ComboAttack::Vertical,
                    ComboAttack::Trip,
                ]
            ))
        ));
    }

    #[test]
    fn find_veinrip_combo() {
        let attacks = vec![
            ComboAttack::Veinrip,
            ComboAttack::Vertical,
            ComboAttack::Crescentcut,
            ComboAttack::Jab,
            ComboAttack::Lowhook,
            ComboAttack::Mindnumb,
            ComboAttack::Pheromones,
            ComboAttack::Flashkick,
            ComboAttack::Pinprick,
            ComboAttack::Feint,
            ComboAttack::Raze,
        ];
        let mut solver = ComboSolver::new(KnifeStance::EinFasit);
        solver.set_attacks(attacks).set_parry(true).set_rebounds(1);
        let combos = solver.find_combos();
        assert_eq!(combos.0.len(), 57);
        for combo in combos.0.iter() {
            println!("{:?}", combo);
        }
    }
}
