use super::*;
use crate::classes::Class;
use crate::curatives::statics::RESTORE_CURE_ORDERS;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;
use topper_core::timeline::BaseAgentState;

pub const SHOCK_TIME: f32 = 20.0;
pub const BURNOUT_TIME: f32 = 20.0;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct AgentState {
    pub balances: [Timer; BType::SIZE as usize],
    pub stats: [CType; SType::SIZE as usize],
    pub max_stats: [CType; SType::SIZE as usize],
    pub aggro: AggroState,
    pub flags: FlagSet,
    pub limb_damage: LimbSet,
    pub hypno_state: HypnoState,
    pub class_state: ClassState,
    pub relapses: RelapseState,
    pub parrying: Option<LType>,
    pub wield_state: WieldState,
    pub dodge_state: DodgeState,
    pub channel_state: ChannelState,
    pub hidden_state: HiddenState,
    pub branch_state: BranchState,
    pub resin_state: ResinState,
    pub pipe_state: PipesState,
    pub bard_board: BardBoard,
    pub predator_board: PredatorBoard,
    pub room_id: i64,
    pub elevation: Elevation,
}

impl BaseAgentState for AgentState {
    fn wait(&mut self, duration: i32) {
        self.aggro.wait(duration);
        self.relapses.wait(duration);
        self.resin_state.wait(duration);
        self.class_state.wait(duration);
        self.dodge_state.wait(duration);
        self.pipe_state.wait(duration);
        self.bard_board.wait(duration);
        self.predator_board.wait(duration);
        if let Some((cured_limb, heal_modifier, first_person)) = self.limb_damage.wait(duration) {
            if !first_person {
                match self.get_restore_cure(cured_limb) {
                    Some(FType::LeftArmBroken)
                    | Some(FType::LeftArmMangled)
                    | Some(FType::LeftArmAmputated)
                    | Some(FType::LeftLegBroken)
                    | Some(FType::LeftLegMangled)
                    | Some(FType::LeftLegAmputated)
                    | Some(FType::RightArmBroken)
                    | Some(FType::RightArmMangled)
                    | Some(FType::RightArmAmputated)
                    | Some(FType::RightLegBroken)
                    | Some(FType::RightLegMangled)
                    | Some(FType::RightLegAmputated)
                    | Some(FType::TorsoBroken)
                    | Some(FType::TorsoMangled)
                    | Some(FType::HeadBroken)
                    | Some(FType::HeadMangled) => {
                        self.limb_damage.restore(cured_limb, heal_modifier);
                    }
                    Some(aff) => {
                        self.set_flag(aff, false);
                    }
                    _ => {
                        self.limb_damage.restore(cured_limb, heal_modifier);
                    }
                }
            }
        }
        let rebound_pending = !self.balanced(BType::Rebounding) && !self.is(FType::Rebounding);
        for i in 0..self.balances.len() {
            self.balances[i].wait(duration);
        }
        if rebound_pending && self.balanced(BType::Rebounding) {
            self.set_flag(FType::AssumedRebounding, true);
        }
        if self.is(FType::Void) && self.balanced(BType::Void) {
            self.set_flag(FType::Void, false);
        } else if self.is(FType::Weakvoid) && self.balanced(BType::Void) {
            self.set_flag(FType::Weakvoid, false);
        }
        if self.is(FType::Manabarbs) && self.balanced(BType::Manabarbs) {
            self.set_flag(FType::Manabarbs, false);
        }
        if self.is(FType::WritheDartpinned) && self.balanced(BType::WritheDartpinned) {
            self.set_flag(FType::WritheDartpinned, false);
        }
        if self.is(FType::WritheWeb) && self.balanced(BType::WritheWeb) {
            self.set_flag(FType::WritheWeb, false);
        }
        if self.is(FType::Voyria) && self.balanced(BType::Voyria) {
            self.set_flag(FType::Voyria, false);
        }
        if self.is(FType::SelfLoathing) {
            let observed = self.get_count(FType::SelfLoathing);
            if (observed <= 2 && self.get_balance(BType::SelfLoathing) < 3.)
                || (observed <= 1 && self.get_balance(BType::SelfLoathing) < 7.)
            {
                println!(
                    "Tracking lost on self_loathing tick {} somehow...",
                    observed
                );
                self.observe_flag(FType::SelfLoathing, false);
            }
        }
        if self.is(FType::Shock) && self.balanced(BType::Shock) {
            self.set_flag(FType::Shock, false);
        }
        if self.is(FType::Burnout) && self.balanced(BType::Burnout) {
            self.set_flag(FType::Burnout, false);
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
            FType::Acid => self.predator_board.acid.is_active(),
            FType::Fleshbane => self.predator_board.fleshbane.is_active(),
            FType::Bloodscourge => self.predator_board.bloodscourge.is_active(),
            FType::Cirisosis => self.predator_board.cirisosis.is_active(),
            FType::Veinrip => self.predator_board.veinrip.is_active(),
            FType::Intoxicated => self.predator_board.is_intoxicated(),
            FType::Negated => self.predator_board.is_negated(),
            FType::LeftLegCrippled => self.limb_damage.crippled(LType::LeftLegDamage),
            FType::RightLegCrippled => self.limb_damage.crippled(LType::RightLegDamage),
            FType::LeftArmCrippled => self.limb_damage.crippled(LType::LeftArmDamage),
            FType::RightArmCrippled => self.limb_damage.crippled(LType::RightArmDamage),
            FType::HeadBroken => self.limb_damage.broken(LType::HeadDamage),
            FType::TorsoBroken => self.limb_damage.broken(LType::TorsoDamage),
            FType::LeftLegBroken => self.limb_damage.broken(LType::LeftLegDamage),
            FType::RightLegBroken => self.limb_damage.broken(LType::RightLegDamage),
            FType::LeftArmBroken => self.limb_damage.broken(LType::LeftArmDamage),
            FType::RightArmBroken => self.limb_damage.broken(LType::RightArmDamage),
            FType::HeadMangled => self.limb_damage.mangled(LType::HeadDamage),
            FType::TorsoMangled => self.limb_damage.mangled(LType::TorsoDamage),
            FType::LeftLegMangled => self.limb_damage.mangled(LType::LeftLegDamage),
            FType::RightLegMangled => self.limb_damage.mangled(LType::RightLegDamage),
            FType::LeftArmMangled => self.limb_damage.mangled(LType::LeftArmDamage),
            FType::RightArmMangled => self.limb_damage.mangled(LType::RightArmDamage),
            FType::LeftLegAmputated => self.limb_damage.amputated(LType::LeftLegDamage),
            FType::RightLegAmputated => self.limb_damage.amputated(LType::RightLegDamage),
            FType::LeftArmAmputated => self.limb_damage.amputated(LType::LeftArmDamage),
            FType::RightArmAmputated => self.limb_damage.amputated(LType::RightArmDamage),
            _ => {
                if flag.is_mirror() {
                    self.flags.is_flag_set(flag.normalize())
                } else {
                    self.flags.is_flag_set(flag)
                }
            }
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

    pub fn add_guess(&mut self, flag: FType) -> bool {
        self.hidden_state.add_guess(flag)
    }

    // The flag is observed to be a certain value (true or false).
    pub fn observe_flag(&mut self, flag: FType, value: bool) {
        if !value && self.is(flag) {
            self.branch_state.strike_aff(flag, value);
        } else if value && !self.is(flag) {
            if !self.hidden_state.found_out() {
                self.branch_state.strike_aff(flag, value);
            }
        } else if value {
            // We've observed this is true, so no need to guess!
            self.hidden_state.unhide(flag);
        }
        match flag {
            FType::TorsoBroken => {
                if !value {
                    self.limb_damage.set_limb_broken(LType::TorsoDamage, false)
                }
            }
            _ => {}
        }
        self.set_flag(flag, value);
    }

    // The flag is observed switching to a certin value (true or false)
    pub fn toggle_flag(&mut self, flag: FType, value: bool) {
        if value && self.is(flag) {
            self.branch_state.strike_aff(flag, !value);
        } else if !value && !self.is(flag) {
            if !self.hidden_state.found_out() {
                self.branch_state.strike_aff(flag, !value);
            }
        }
        self.set_flag(flag, value);
    }

    pub fn set_flag(&mut self, flag: FType, value: bool) {
        if !value {
            self.hidden_state.unhide(flag);
        }
        match flag {
            FType::Acid => {
                if value {
                    self.predator_board.acid()
                } else {
                    self.predator_board.acid_end();
                }
            }
            FType::Fleshbane => {
                if value {
                    self.predator_board.fleshbaned()
                } else {
                    self.predator_board.fleshbane_end();
                }
            }
            FType::Bloodscourge => {
                if value {
                    self.predator_board.bloodscourged()
                } else {
                    self.predator_board.bloodscourge_end()
                }
            }
            FType::Cirisosis => {
                if value {
                    self.predator_board.cirisosis_start()
                } else {
                    self.predator_board.cirisosis_lost();
                }
            }
            FType::Veinrip => {
                if value {
                    self.predator_board.veinrip_start()
                } else {
                    self.predator_board.veinrip_end();
                }
            }
            FType::Intoxicated => {
                if value {
                    self.predator_board.intoxicate()
                } else {
                    self.predator_board.intoxicate_used();
                }
            }
            FType::Negated => {
                if value {
                    self.predator_board.negate()
                } else {
                    self.predator_board.negate_end();
                }
            }
            FType::LeftLegCrippled => self
                .limb_damage
                .set_limb_crippled(LType::LeftLegDamage, value),
            FType::RightLegCrippled => self
                .limb_damage
                .set_limb_crippled(LType::RightLegDamage, value),
            FType::LeftArmCrippled => self
                .limb_damage
                .set_limb_crippled(LType::LeftArmDamage, value),
            FType::RightArmCrippled => self
                .limb_damage
                .set_limb_crippled(LType::RightArmDamage, value),
            FType::LeftLegBroken
            | FType::LeftArmBroken
            | FType::RightLegBroken
            | FType::RightArmBroken
            | FType::TorsoBroken
            | FType::HeadBroken => {}
            FType::LeftLegMangled
            | FType::LeftArmMangled
            | FType::RightLegMangled
            | FType::RightArmMangled
            | FType::TorsoMangled
            | FType::HeadMangled => {}
            FType::LeftLegAmputated
            | FType::LeftArmAmputated
            | FType::RightLegAmputated
            | FType::RightArmAmputated => {}
            _ => self.flags.set_flag(flag, value),
        }
        if flag == FType::Shock && value {
            self.set_balance(BType::Shock, SHOCK_TIME);
        }
        if flag == FType::Burnout && value {
            self.set_balance(BType::Burnout, BURNOUT_TIME);
        }
        if flag == FType::Rebounding {
            self.flags.set_flag(FType::AssumedRebounding, false);
        }
        if (flag == FType::Weakvoid || flag == FType::Void) && value == true {
            self.set_balance(BType::Void, 10.0);
        }
        if value && flag == FType::Paresis {
            self.set_balance(BType::ParesisParalysis, 4.0);
        }
        if value && flag == FType::SelfLoathing {
            self.set_balance(BType::SelfLoathing, 12.0);
        }
        if value && flag == FType::Pacifism {
            self.set_balance(BType::Pacifism, 1.5);
        }
        if value && flag == FType::WritheDartpinned {
            self.set_balance(BType::WritheDartpinned, 3.0);
        }
        if value && flag == FType::WritheWeb {
            self.set_balance(BType::WritheWeb, 3.0);
        }
        if value && flag == FType::Voyria {
            self.set_balance(BType::Voyria, 12.0);
        }
        match flag {
            FType::Zenith => {
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
            FType::Halfbeat => {
                if value {
                    self.assume_bard(&|bard: &mut BardClassState| {
                        bard.half_beat_slowdown();
                    });
                } else {
                    self.assume_bard(&|bard: &mut BardClassState| {
                        bard.half_beat_end();
                    })
                }
            }
            _ => {}
        }
    }

    // The flag is observed to be a certain value (true or false).
    pub fn observe_flag_ticking(&mut self, flag: FType) {
        if !self.is(flag) {
            self.branch_state.strike_aff(flag, true);
        }
        self.tick_flag_up(flag);
    }

    // The flag is observed to be a certain value (true or false).
    pub fn observe_flag_count(&mut self, flag: FType, count: u8) {
        if !self.is(flag) {
            self.branch_state.strike_aff(flag, true);
        }
        self.set_count(flag, count);
    }

    pub fn tick_flag_up(&mut self, flag: FType) {
        self.flags.tick_counter_up(flag);
    }

    pub fn set_balance(&mut self, balance: BType, value: f32) {
        match balance {
            BType::Induce => self.assume_bard(&|mut bard| bard.set_induce_timer(value)),
            _ => self.balances[balance as usize].start_count_down_seconds(value),
        }
    }

    pub fn get_raw_balance(&self, balance: BType) -> CType {
        match balance {
            BType::Induce => self
                .check_if_bard(&|bard| bard.get_induce_time_left())
                .map(|time: i32| time as CType)
                .unwrap_or(0),
            _ => self.balances[balance as usize].get_time_left(),
        }
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
        match balance {
            BType::Induce => self
                .check_if_bard(&|bard| bard.get_induce_time_left())
                .map(|time: i32| time as f32 / BALANCE_SCALE)
                .unwrap_or(0.0),
            _ => self.balances[balance as usize].get_time_left_seconds(),
        }
    }

    pub fn balanced(&self, balance: BType) -> bool {
        match balance {
            BType::Induce => self
                .check_if_bard(&|bard| bard.induce_ready())
                .unwrap_or(false),
            _ => self.balances[balance as usize].is_active(),
        }
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
                if self.balanced(*balance) {
                    // Do nothing.
                } else if self.get_raw_balance(*balance) < self.get_raw_balance(*earliest_bal) {
                    earliest = Some(balance);
                }
            } else {
                earliest = Some(balance);
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

    pub fn get_max_stat(&self, stat: SType) -> CType {
        self.max_stats[stat as usize]
    }

    pub fn set_max_stat(&mut self, stat: SType, value: CType) {
        self.max_stats[stat as usize] = value;
    }

    pub fn get_health_percent(&self) -> f32 {
        self.stats[SType::Health as usize] as f32 / self.max_stats[SType::Health as usize] as f32
    }

    pub fn get_mana_percent(&self) -> f32 {
        self.stats[SType::Mana as usize] as f32 / self.max_stats[SType::Mana as usize] as f32
    }

    pub fn set_limb_damage(&mut self, limb: LType, value: CType, assume_break: bool) {
        let old_value = self.limb_damage.limbs[limb as usize].damage;
        self.limb_damage.set_limb_damage(limb, value, assume_break);
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
        let broken = limb.broken;
        let mangled = limb.mangled;
        let amputated = limb.amputated;
        let crippled = match what.crippled() {
            Some(broken_aff) => self.is(broken_aff),
            _ => false,
        } || damage > 35.0;
        let welt = limb.welt;
        let is_restoring = self.limb_damage.restoring == Some(what);
        let fleshbaned_count = self.limb_damage.fleshbaned_count;
        let is_parried = self.can_parry() && self.parrying == Some(what);
        let is_dislocated = match what {
            LType::LeftArmDamage => self.is(FType::LeftArmDislocated),
            LType::RightArmDamage => self.is(FType::RightArmDislocated),
            LType::LeftLegDamage => self.is(FType::LeftLegDislocated),
            LType::RightLegDamage => self.is(FType::RightLegDislocated),
            _ => false,
        };
        let bruise_level = match what {
            LType::LeftArmDamage => self.affs_count(&vec![
                FType::LeftArmBruised,
                FType::LeftArmBruisedModerate,
                FType::LeftArmBruisedCritical,
            ]),
            LType::RightArmDamage => self.affs_count(&vec![
                FType::RightArmBruised,
                FType::RightArmBruisedModerate,
                FType::RightArmBruisedCritical,
            ]),
            LType::LeftLegDamage => self.affs_count(&vec![
                FType::LeftLegBruised,
                FType::LeftLegBruisedModerate,
                FType::LeftLegBruisedCritical,
            ]),
            LType::RightLegDamage => self.affs_count(&vec![
                FType::RightLegBruised,
                FType::RightLegBruisedModerate,
                FType::RightLegBruisedCritical,
            ]),
            LType::HeadDamage => self.affs_count(&vec![
                FType::HeadBruised,
                FType::HeadBruisedModerate,
                FType::HeadBruisedCritical,
            ]),
            LType::TorsoDamage => self.affs_count(&vec![
                FType::TorsoBruised,
                FType::TorsoBruisedModerate,
                FType::TorsoBruisedCritical,
            ]),
            _ => 0,
        };
        LimbState {
            damage,
            crippled,
            broken,
            mangled,
            amputated,
            is_restoring,
            is_parried,
            is_dislocated,
            welt,
            bruise_level,
            fleshbaned_count,
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

    pub fn can_wield(&self, left: bool, right: bool) -> bool {
        if left && self.get_limb_state(LType::LeftArmDamage).crippled {
            return false;
        }
        if right && self.get_limb_state(LType::RightArmDamage).crippled {
            return false;
        }
        if self.is(FType::Paralysis) || self.is(FType::Perplexed) {
            return false;
        }
        true
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
                    if other.get_time_left() < focus_time.get_time_left() {
                        Some(other)
                    } else {
                        Some(self.balances[BType::Focus as usize])
                    }
                });
            } else {
                earliest_escape = earliest_escape.or(Some(Timer::count_down_seconds(15.)))
            }
        }
        earliest_escape.map(|escape| escape.get_time_left_seconds())
    }

    pub fn can_touch(&self) -> bool {
        !self.is(FType::Paresis)
            && !self.is(FType::Paralysis)
            && !(self.is(FType::LeftArmCrippled) && self.is(FType::RightArmCrippled))
            && !self.is(FType::NumbArms)
    }

    pub fn can_tree(&self, ignore_bal: bool) -> bool {
        self.can_touch() && (ignore_bal || self.balanced(BType::Tree))
    }

    pub fn can_focus(&self, ignore_bal: bool) -> bool {
        !self.is(FType::Impatience)
            && !self.is(FType::Besilence)
            && (ignore_bal || self.balanced(BType::Focus))
    }

    pub fn can_parry(&self) -> bool {
        self.affs_count(&vec![
            // Fallen counts for prone, but not parrying.
            FType::Indifference,
            FType::Asleep,
            FType::Stun,
            FType::Paralysis,
            FType::WritheImpaled,
            FType::WritheArmpitlock,
            FType::WritheNecklock,
            FType::WritheThighlock,
            FType::WritheTransfix,
            FType::WritheBind,
            FType::WritheGunk,
            FType::WritheRopes,
            FType::WritheVines,
            FType::WritheWeb,
            FType::WritheDartpinned,
            FType::WritheHoist,
            FType::WritheGrappled,
            FType::WritheStasis,
        ]) == 0
            && !(self.is(FType::LeftArmCrippled) && self.is(FType::RightArmCrippled))
    }

    pub fn is_prone(&self) -> bool {
        self.affs_count(&vec![
            // Fallen counts for prone, but not parrying.
            FType::Fallen,
            FType::Indifference,
            FType::Asleep,
            FType::Stun,
            FType::Paralysis,
            FType::WritheImpaled,
            FType::WritheArmpitlock,
            FType::WritheNecklock,
            FType::WritheThighlock,
            FType::WritheTransfix,
            FType::WritheBind,
            FType::WritheGunk,
            FType::WritheRopes,
            FType::WritheVines,
            FType::WritheWeb,
            FType::WritheDartpinned,
            FType::WritheHoist,
            FType::WritheGrappled,
            FType::WritheStasis,
        ]) > 0
    }

    pub fn observe_not_prone(&mut self) {
        if self.is_prone() {
            println!("Not actually prone!");
            self.observe_flag(FType::Fallen, false);
            self.observe_flag(FType::Indifference, false);
            self.observe_flag(FType::Asleep, false);
            self.observe_flag(FType::Stun, false);
            // No writhes!
            self.observe_flag(FType::WritheImpaled, false);
            self.observe_flag(FType::WritheArmpitlock, false);
            self.observe_flag(FType::WritheNecklock, false);
            self.observe_flag(FType::WritheThighlock, false);
            self.observe_flag(FType::WritheTransfix, false);
            self.observe_flag(FType::WritheBind, false);
            self.observe_flag(FType::WritheGunk, false);
            self.observe_flag(FType::WritheRopes, false);
            self.observe_flag(FType::WritheVines, false);
            self.observe_flag(FType::WritheWeb, false);
            self.observe_flag(FType::WritheDartpinned, false);
            self.observe_flag(FType::WritheHoist, false);
            self.observe_flag(FType::WritheGrappled, false);
            self.observe_flag(FType::WritheStasis, false);
        }
    }

    pub fn stuck_fallen(&self) -> bool {
        self.is(FType::Fallen) && !self.can_stand()
    }

    pub fn can_stand(&self) -> bool {
        !self.is(FType::LeftLegCrippled)
            && !self.is(FType::RightLegCrippled)
            && !self.is(FType::Frozen)
            && !self.is(FType::Paralysis)
    }

    pub fn arms_free(&self) -> bool {
        self.arm_free_left() && self.arm_free_right()
    }

    pub fn arm_free(&self) -> bool {
        self.arm_free_left() || self.arm_free_right()
    }

    pub fn arm_free_left(&self) -> bool {
        !self.is(FType::LeftArmCrippled)
    }

    pub fn arm_free_right(&self) -> bool {
        !self.is(FType::RightArmCrippled)
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

    pub fn get_restore_time_left(&self) -> f32 {
        if let Some(timer) = self.limb_damage.restore_timer {
            timer.get_time_left_seconds()
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

    pub fn get_restore_cure(&self, limb: LType) -> Option<FType> {
        let cure_order = RESTORE_CURE_ORDERS.get(&limb).unwrap();
        for cure in cure_order {
            match cure {
                FType::LeftLegAmputated
                | FType::RightLegAmputated
                | FType::LeftArmAmputated
                | FType::RightArmAmputated => {
                    if self.limb_damage.amputated(limb) {
                        return Some(*cure);
                    }
                }
                FType::LeftLegMangled
                | FType::RightLegMangled
                | FType::LeftArmMangled
                | FType::RightArmMangled => {
                    if self.limb_damage.mangled(limb) {
                        return Some(*cure);
                    }
                }
                FType::LeftLegBroken
                | FType::RightLegBroken
                | FType::LeftArmBroken
                | FType::RightArmBroken => {
                    if self.limb_damage.broken(limb) {
                        return Some(*cure);
                    }
                }
                aff => {
                    if self.is(*aff) {
                        return Some(*aff);
                    }
                }
            }
        }
        None
    }

    pub fn get_curing(&self) -> Option<FType> {
        self.limb_damage
            .get_restoring_limb()
            .and_then(|limb| self.get_restore_cure(limb))
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
            } else if self.limb_damage.broken(limb) {
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

    pub fn assume_bard<R>(&mut self, action: &Fn(&mut BardClassState) -> R) -> R {
        if let ClassState::Bard(bard) = &mut self.class_state {
            action(bard)
        } else {
            self.class_state = ClassState::Bard(BardClassState::default());
            self.assume_bard(action)
        }
    }

    pub fn check_if_bard<R>(&self, action: &Fn(&BardClassState) -> R) -> Option<R> {
        if let ClassState::Bard(bard) = &self.class_state {
            Some(action(bard))
        } else {
            None
        }
    }

    pub fn assume_monk<R>(&mut self, action: &Fn(&mut MonkClassState) -> R) -> R {
        if let ClassState::Monk(monk) = &mut self.class_state {
            action(monk)
        } else {
            self.class_state = ClassState::Monk(MonkClassState::default());
            self.assume_monk(action)
        }
    }

    pub fn check_if_monk<R>(&self, action: &Fn(&MonkClassState) -> R) -> Option<R> {
        if let ClassState::Monk(monk) = &self.class_state {
            Some(action(monk))
        } else {
            None
        }
    }

    pub fn assume_infiltrator<R>(&mut self, action: &Fn(&mut InfiltratorClassState) -> R) -> R {
        if let ClassState::Infiltrator(infiltrator) = &mut self.class_state {
            action(infiltrator)
        } else {
            self.class_state = ClassState::Infiltrator(InfiltratorClassState::default());
            self.assume_infiltrator(action)
        }
    }

    pub fn check_if_infiltrator<R>(&self, action: &Fn(&InfiltratorClassState) -> R) -> Option<R> {
        if let ClassState::Infiltrator(infiltrator) = &self.class_state {
            Some(action(infiltrator))
        } else {
            None
        }
    }

    pub fn assume_predator<R>(&mut self, action: &Fn(&mut PredatorClassState) -> R) -> R {
        if let ClassState::Predator(predator) = &mut self.class_state {
            action(predator)
        } else {
            self.class_state = ClassState::Predator(PredatorClassState::default());
            self.assume_predator(action)
        }
    }

    pub fn check_if_predator<R>(&self, action: &Fn(&PredatorClassState) -> R) -> Option<R> {
        if let ClassState::Predator(predator) = &self.class_state {
            Some(action(predator))
        } else {
            None
        }
    }

    pub fn get_predator_stance(&self) -> KnifeStance {
        if let ClassState::Predator(predator) = &self.class_state {
            predator.stance
        } else {
            KnifeStance::None
        }
    }

    pub fn get_monk_stance(&self) -> MonkStance {
        if let ClassState::Monk(monk) = &self.class_state {
            monk.stance
        } else {
            MonkStance::None
        }
    }

    pub fn get_aggro(&self) -> i32 {
        self.aggro.get_aggro_count()
    }

    pub fn register_hit(&mut self, attacker: Option<&String>) {
        self.aggro.register_hit(attacker);
    }

    pub fn get_normalized_class(&self) -> Option<Class> {
        self.class_state.get_normalized_class()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_will_be_rebounding() {
        let mut state = AgentState::default();
        assert!(!state.will_be_rebounding(0.5));

        state.set_balance(BType::Rebounding, 1.);
        assert!(!state.will_be_rebounding(0.5));
        assert!(state.will_be_rebounding(1.5));
    }
}
