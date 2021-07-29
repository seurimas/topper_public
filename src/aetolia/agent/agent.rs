use super::*;
use crate::timeline::BaseAgentState;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct AgentState {
    pub balances: [CType; BType::SIZE as usize],
    pub stats: [CType; SType::SIZE as usize],
    pub max_stats: [CType; SType::SIZE as usize],
    pub flags: FlagSet,
    pub limb_damage: LimbSet,
    pub hypno_state: HypnoState,
    pub class_state: ClassState,
    pub relapses: RelapseState,
    pub parrying: Option<LType>,
    pub wield_state: WieldState,
    pub dodge_state: DodgeState,
    pub channel_state: ChannelState,
    pub branch_state: BranchState,
    pub resin_state: ResinState,
}

impl BaseAgentState for AgentState {
    fn wait(&mut self, duration: i32) {
        self.relapses.wait(duration);
        self.resin_state.wait(duration);
        self.class_state.wait(duration);
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
    fn get_base_state() -> Self {
        let mut val = AgentState::default();
        val.initialize_stat(SType::Health, 4000);
        val.initialize_stat(SType::Mana, 4000);
        val.set_flag(FType::Player, true);
        val.set_flag(FType::Blindness, true);
        val.set_flag(FType::Deafness, true);
        val.set_flag(FType::Temperance, true);
        val.set_flag(FType::Levitation, true);
        val.set_flag(FType::Speed, true);
        val.set_flag(FType::Temperance, true);
        val.set_flag(FType::Vigor, true);
        val.set_flag(FType::Rebounding, true);
        val.set_flag(FType::Insomnia, true);
        val.set_flag(FType::Fangbarrier, true);
        val.set_flag(FType::Instawake, true);
        val.set_flag(FType::Insulation, true);
        val
    }

    fn branch(&mut self, time: CType) {
        self.branch_state.branch(time);
    }
}

impl AgentState {
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

    // The flag is observed to be a certain value (true or false).
    pub fn observe_flag(&mut self, flag: FType, value: bool) {
        if !value && self.is(flag) {
            self.branch_state.strike_aff(flag, value);
        } else if value && !self.is(flag) {
            self.branch_state.strike_aff(flag, value);
        }
        self.set_flag(flag, value);
    }

    // The flag is observed switching to a certin value (true or false)
    pub fn toggle_flag(&mut self, flag: FType, value: bool) {
        if value && self.is(flag) {
            self.branch_state.strike_aff(flag, !value);
        } else if value && self.is(flag) {
            self.branch_state.strike_aff(flag, !value);
        }
        self.set_flag(flag, value);
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
                self.assume_zealot(|zealot| {
                    zealot.zenith.activate();
                });
            } else {
                self.assume_zealot(|zealot| {
                    zealot.zenith.deactivate();
                });
            }
        }
    }

    // The flag is observed to be a certain value (true or false).
    pub fn observe_flag_ticking(&mut self, flag: FType) {
        if !self.is(flag) {
            self.branch_state.strike_aff(flag, true);
        }
        self.tick_flag_up(flag);
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
            && !self.is(FType::Paresis)
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

    pub fn set_channel(&mut self, channel: ChannelState) {
        self.channel_state = channel;
    }

    pub fn assume_zealot(&mut self, action: fn(&mut ZealotClassState)) {
        if let ClassState::Zealot(zealot) = &mut self.class_state {
            action(zealot);
        } else {
            self.class_state = ClassState::Zealot(ZealotClassState::default());
            self.assume_zealot(action);
        }
    }
}
