use crate::classes::VenomType;

use super::*;
use serde::*;

pub const FEINT_COOLDOWN: CType = 10 * BALANCE_SCALE as CType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
pub enum KnifeStance {
    None,
    Gyanis,
    VaeSant,
    Rizet,
    EinFasit,
    Laesan,
    Bladesurge,
}

impl Default for KnifeStance {
    fn default() -> Self {
        KnifeStance::None
    }
}

impl KnifeStance {
    pub fn from_name(name: &str) -> KnifeStance {
        match name {
            "Gyanis" => KnifeStance::Gyanis,
            "Vae-Sant" => KnifeStance::VaeSant,
            "Rizet" => KnifeStance::Rizet,
            "Ein-Fasit" => KnifeStance::EinFasit,
            "Laesan" => KnifeStance::Laesan,
            "Bladesurge" => KnifeStance::Bladesurge,
            _ => KnifeStance::None,
        }
    }

    pub fn to_name(&self) -> &'static str {
        match self {
            KnifeStance::Gyanis => "Gyanis",
            KnifeStance::VaeSant => "Vae-Sant",
            KnifeStance::Rizet => "Rizet",
            KnifeStance::EinFasit => "Ein-Fasit",
            KnifeStance::Laesan => "Laesan",
            KnifeStance::Bladesurge => "Bladesurge",
            _ => "None",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PredatorCompanionState {
    Orel {
        venoms: Vec<String>,
        swooping: Option<Timer>,
    },
    Orgyuk {
        roaring: Option<Timer>,
        raking: Option<(Timer, String, u32)>,
    },
    Spider {
        web_cooldown: Timer,
        intoxicate_target: Option<String>,
        strands_target: Option<String>,
    },
}

impl PredatorCompanionState {
    pub fn wait(&mut self, time: CType) {
        match self {
            PredatorCompanionState::Orel { swooping, .. } => {
                if let Some(swooping) = swooping {
                    swooping.wait(time);
                }
            }
            PredatorCompanionState::Orgyuk { roaring, raking } => {
                if let Some(roaring) = roaring {
                    roaring.wait(time);
                }
                if let Some((raking_timer, _, _)) = raking {
                    raking_timer.wait(time);
                    if !raking_timer.is_active() {
                        *raking = None;
                    }
                }
            }
            PredatorCompanionState::Spider { web_cooldown, .. } => {
                web_cooldown.wait(time);
            }
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct PredatorClassState {
    pub apex: u32,
    pub stance: KnifeStance,
    pub feint_time: CType,
    pub tidalslash: bool,
    pub companion: Option<PredatorCompanionState>,
}

impl PredatorClassState {
    pub fn wait(&mut self, time: CType) {
        self.feint_time -= time;
        self.companion.iter_mut().for_each(|c| c.wait(time));
    }

    pub fn feint(&mut self) {
        self.feint_time = FEINT_COOLDOWN;
    }

    pub fn get_spider(&mut self) {
        if !self.has_spider() {
            self.companion = Some(PredatorCompanionState::Spider {
                strands_target: None,
                intoxicate_target: None,
                web_cooldown: Timer::count_up_seconds(20.),
            });
        }
    }

    pub fn has_spider(&self) -> bool {
        if let Some(PredatorCompanionState::Spider { .. }) = self.companion {
            true
        } else {
            false
        }
    }

    pub fn can_web(&self) -> bool {
        if let Some(PredatorCompanionState::Spider { web_cooldown, .. }) = &self.companion {
            !web_cooldown.is_active()
        } else {
            false
        }
    }

    pub fn webbed(&mut self) {
        self.get_spider();
        if let Some(PredatorCompanionState::Spider { web_cooldown, .. }) = &mut self.companion {
            web_cooldown.reset();
        }
    }

    pub fn intoxicate(&mut self, target: String) {
        self.get_spider();
        if let Some(PredatorCompanionState::Spider {
            intoxicate_target, ..
        }) = &mut self.companion
        {
            *intoxicate_target = Some(target);
        }
    }

    pub fn is_intoxicating(&self, target: &String) -> bool {
        if let Some(PredatorCompanionState::Spider {
            intoxicate_target, ..
        }) = &self.companion
        {
            if let Some(intoxicate_target) = intoxicate_target {
                intoxicate_target.eq_ignore_ascii_case(target)
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn strands_on(&self, target: &String) -> bool {
        if let Some(PredatorCompanionState::Spider { strands_target, .. }) = &self.companion {
            if let Some(strands_target) = strands_target {
                strands_target.eq_ignore_ascii_case(target)
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn get_orgyuk(&mut self) {
        if !self.has_orgyuk() {
            self.companion = Some(PredatorCompanionState::Orgyuk {
                roaring: None,
                raking: None,
            });
        }
    }

    pub fn has_orgyuk(&self) -> bool {
        if let Some(PredatorCompanionState::Orgyuk { .. }) = self.companion {
            true
        } else {
            false
        }
    }

    pub fn rake_start(&mut self, who: String) {
        self.get_orgyuk();
        if let Some(PredatorCompanionState::Orgyuk { raking, .. }) = &mut self.companion {
            *raking = Some((Timer::count_up_observe_seconds(2., 3.), who, 1));
        }
    }

    pub fn rake(&mut self, who: &String) {
        if let Some(PredatorCompanionState::Orgyuk { raking, .. }) = &mut self.companion {
            if let Some((raking_timer, who, rake_count)) = raking {
                if who.eq_ignore_ascii_case(who) {
                    *rake_count += 1;
                    if *rake_count >= 4 {
                        *raking = None;
                    } else {
                        raking_timer.reset();
                    }
                }
            }
        }
    }

    pub fn is_raking(&self) -> bool {
        if let Some(PredatorCompanionState::Orgyuk { raking, .. }) = &self.companion {
            if let Some((raking_timer, _, _)) = raking {
                raking_timer.is_active()
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn get_orel(&mut self) {
        if !self.has_orel() {
            self.companion = Some(PredatorCompanionState::Orel {
                venoms: Vec::new(),
                swooping: None,
            });
        }
    }

    pub fn has_orel(&self) -> bool {
        if let Some(PredatorCompanionState::Orel { .. }) = self.companion {
            true
        } else {
            false
        }
    }

    pub fn tidalslash_full(&mut self) {
        self.tidalslash = true;
    }

    pub fn use_tidalslash(&mut self) {
        self.tidalslash = false;
    }

    pub fn get_arouse_time(&self) -> f32 {
        if self.apex >= 3 {
            5.
        } else {
            90. - (self.apex as f32 * 30.)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PredatorBoard {
    pub fleshbane: Timer,
    pub fleshbane_count: u32,
    pub intoxicate: Timer,
    pub bloodscourge: Timer,
    pub veinrip: Timer,
    pub acid: Timer,
    pub cirisosis: Timer,
    pub negate: Timer,
}

impl Default for PredatorBoard {
    fn default() -> Self {
        let mut default = PredatorBoard {
            fleshbane: Timer::count_up_observe_seconds(60., 65.),
            fleshbane_count: 0,
            intoxicate: Timer::count_up_observe_seconds(10., 11.),
            bloodscourge: Timer::count_up_observe_seconds(4., 7.),
            veinrip: Timer::count_up_observe_seconds(14., 20.),
            acid: Timer::count_up_seconds(10.),
            cirisosis: Timer::count_up_observe_seconds(4., 6.),
            negate: Timer::count_up_observe_seconds(20., 21.),
        };
        default.intoxicate.expire();
        default.fleshbane.expire();
        default.bloodscourge.expire();
        default.veinrip.expire();
        default.acid.expire();
        default.cirisosis.expire();
        default.negate.expire();
        default
    }
}

impl PredatorBoard {
    pub fn wait(&mut self, time: CType) {
        self.fleshbane.wait(time);
        if !self.fleshbane.is_active() {
            self.fleshbane_count = 0;
        }
        self.bloodscourge.wait(time);
        self.veinrip.wait(time);
        self.acid.wait(time);
        self.cirisosis.wait(time);
        self.negate.wait(time);
    }

    pub fn fleshbaned(&mut self) {
        self.fleshbane.reset();
    }

    pub fn fleshbane_triggered(&mut self) {
        // Timer NOT reset.
        self.fleshbane_count = 0;
    }

    pub fn fleshbane_end(&mut self) {
        self.fleshbane.expire();
    }

    pub fn sitara_hit(&mut self, count: u32) {
        self.fleshbane_count += count;
    }

    pub fn bloodscourged(&mut self) {
        self.bloodscourge.reset();
    }

    pub fn bloodscourge_hit(&mut self) {
        self.bloodscourge.reset();
    }

    pub fn bloodscourge_end(&mut self) {
        self.bloodscourge.expire();
    }

    pub fn cirisosis_start(&mut self) {
        self.cirisosis.reset();
    }

    pub fn cirisosis_shock(&mut self) {
        self.cirisosis.reset();
    }

    pub fn cirisosis_lost(&mut self) {
        self.cirisosis.expire();
    }

    pub fn veinrip_start(&mut self) {
        self.veinrip.reset();
    }

    pub fn veinrip_end(&mut self) {
        self.veinrip.expire();
    }

    pub fn acid(&mut self) {
        self.acid.reset();
    }

    pub fn acid_end(&mut self) {
        self.acid.expire();
    }

    pub fn is_intoxicated(&self) -> bool {
        self.intoxicate.is_active()
    }

    pub fn intoxicate(&mut self) {
        self.intoxicate.reset();
    }

    pub fn intoxicate_used(&mut self) {
        self.intoxicate.expire();
    }

    pub fn is_negated(&self) -> bool {
        self.negate.is_active()
    }

    pub fn negate(&mut self) {
        self.negate.reset();
    }

    pub fn negate_end(&mut self) {
        self.negate.expire();
    }
}
