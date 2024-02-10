use super::*;
use crate::alpha_beta::ActionPlanner;
use crate::bt::DEBUG_TREES;
use crate::classes::group::call_venom;
use crate::classes::group::call_venoms;
use crate::classes::group::should_call_venoms;
use crate::classes::*;
use crate::curatives::get_cure_depth;
use crate::observables::*;
use crate::timeline::*;
use crate::types::*;
use regex::Regex;
use std::collections::HashMap;
use strum::IntoEnumIterator;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParamComboAttack {
    Tidalslash,
    Freefall,
    Pheromones,
    Pindown,
    Mindnumb,
    Jab(LType),
    Pinprick,
    Lateral,
    Vertical,
    Crescentcut,
    Spinslash,
    Lowhook(LType),
    Butterfly,
    Flashkick,
    Trip,
    Veinrip,
    Feint(LType),
    Raze,
    Gouge,
    Bleed,
    Swiftkick,
}

impl ParamComboAttack {
    pub fn get_param_string(&self) -> String {
        match self {
            ParamComboAttack::Tidalslash => "tidalslash".to_string(),
            ParamComboAttack::Freefall => "freefall".to_string(),
            ParamComboAttack::Pheromones => "pheromones".to_string(),
            ParamComboAttack::Pindown => "pindown".to_string(),
            ParamComboAttack::Mindnumb => "mindnumb".to_string(),
            ParamComboAttack::Jab(limb) => {
                if *limb == LType::LeftArmDamage {
                    "jab left".to_string()
                } else if *limb == LType::RightArmDamage {
                    "jab right".to_string()
                } else {
                    "jab".to_string()
                }
            }
            ParamComboAttack::Lowhook(limb) => {
                if *limb == LType::LeftLegDamage {
                    "lowhook left".to_string()
                } else if *limb == LType::RightLegDamage {
                    "lowhook right".to_string()
                } else {
                    "lowhook".to_string()
                }
            }
            ParamComboAttack::Pinprick => "pinprick".to_string(),
            ParamComboAttack::Lateral => "lateral".to_string(),
            ParamComboAttack::Vertical => "vertical".to_string(),
            ParamComboAttack::Crescentcut => "crescentcut".to_string(),
            ParamComboAttack::Spinslash => "spinslash".to_string(),
            ParamComboAttack::Butterfly => "butterfly".to_string(),
            ParamComboAttack::Flashkick => "flashkick".to_string(),
            ParamComboAttack::Trip => "trip".to_string(),
            ParamComboAttack::Veinrip => "veinrip".to_string(),
            ParamComboAttack::Feint(limb) => format!("feint {}", limb.to_string()),
            ParamComboAttack::Raze => "raze".to_string(),
            ParamComboAttack::Gouge => "gouge".to_string(),
            ParamComboAttack::Bleed => "bleed".to_string(),
            ParamComboAttack::Swiftkick => "swiftkick".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SeriesAttack {
    pub attacks: Vec<ParamComboAttack>,
    pub target: String,
    pub venom: VenomType,
}

impl SeriesAttack {
    pub fn new(attacks: Vec<ParamComboAttack>, target: String, venom: &'static str) -> Self {
        Self {
            attacks,
            target,
            venom,
        }
    }

    pub fn new_random_params(
        base_attacks: Vec<ComboAttack>,
        target: String,
        venom: &'static str,
        preferred_limbs: &Vec<LType>,
    ) -> Self {
        let mut valid_feints = vec![
            LType::HeadDamage,
            LType::LeftArmDamage,
            LType::RightArmDamage,
            LType::LeftLegDamage,
            LType::RightLegDamage,
            LType::TorsoDamage,
        ];
        valid_feints.retain(|l| {
            !base_attacks
                .iter()
                .any(|attack| Some(*l) == attack.get_single_limb_target())
        });
        valid_feints.retain(|l| !preferred_limbs.contains(l));
        if valid_feints.is_empty() {
            println!("Valid feints is empty, defaulting to head");
            valid_feints = vec![LType::HeadDamage];
        }
        unsafe {
            if DEBUG_TREES {
                println!("Valid feints: {:?}", valid_feints);
            }
        }
        let attacks = base_attacks.iter().map(|base| match base {
            ComboAttack::Jab => {
                ParamComboAttack::Jab(if preferred_limbs.contains(&LType::LeftArmDamage) {
                    LType::LeftArmDamage
                } else if preferred_limbs.contains(&LType::RightArmDamage) {
                    LType::RightArmDamage
                } else {
                    LType::SIZE
                })
            }
            ComboAttack::Lowhook => {
                ParamComboAttack::Lowhook(if preferred_limbs.contains(&LType::LeftLegDamage) {
                    LType::LeftLegDamage
                } else if preferred_limbs.contains(&LType::RightLegDamage) {
                    LType::RightLegDamage
                } else {
                    LType::SIZE
                })
            }
            ComboAttack::Feint => ParamComboAttack::Feint(valid_feints[0]),
            ComboAttack::Bleed => ParamComboAttack::Bleed,
            ComboAttack::Gouge => ParamComboAttack::Gouge,
            ComboAttack::Raze => ParamComboAttack::Raze,
            ComboAttack::Swiftkick => ParamComboAttack::Swiftkick,
            ComboAttack::Tidalslash => ParamComboAttack::Tidalslash,
            ComboAttack::Freefall => ParamComboAttack::Freefall,
            ComboAttack::Pheromones => ParamComboAttack::Pheromones,
            ComboAttack::Pindown => ParamComboAttack::Pindown,
            ComboAttack::Mindnumb => ParamComboAttack::Mindnumb,
            ComboAttack::Pinprick => ParamComboAttack::Pinprick,
            ComboAttack::Lateral => ParamComboAttack::Lateral,
            ComboAttack::Vertical => ParamComboAttack::Vertical,
            ComboAttack::Crescentcut => ParamComboAttack::Crescentcut,
            ComboAttack::Spinslash => ParamComboAttack::Spinslash,
            ComboAttack::Butterfly => ParamComboAttack::Butterfly,
            ComboAttack::Flashkick => ParamComboAttack::Flashkick,
            ComboAttack::Trip => ParamComboAttack::Trip,
            ComboAttack::Veinrip => ParamComboAttack::Veinrip,
        });
        Self {
            attacks: attacks.collect(),
            target,
            venom: venom.into(),
        }
    }
}

impl ActiveTransition for SeriesAttack {
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        if should_call_venoms(timeline) {
            Ok(format!(
                "{};;series {} {} {}",
                call_venom(&self.target, self.venom, None),
                self.attacks
                    .iter()
                    .map(|attack| attack.get_param_string())
                    .collect::<Vec<String>>()
                    .join(" "),
                self.target,
                self.venom
            ))
        } else {
            Ok(format!(
                "series {} {} {}",
                self.attacks
                    .iter()
                    .map(|attack| attack.get_param_string())
                    .collect::<Vec<String>>()
                    .join(" "),
                self.target,
                self.venom
            ))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BloodscourgeAction {
    pub target: String,
    pub venom: VenomType,
}

impl BloodscourgeAction {
    pub fn new(target: String, venom: &'static str) -> Self {
        Self {
            target,
            venom: venom.into(),
        }
    }
}

impl ActiveTransition for BloodscourgeAction {
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        if should_call_venoms(timeline) {
            Ok(format!(
                "{};;bloodscourge {} {}",
                call_venom(&self.target, self.venom, None),
                self.target,
                self.venom
            ))
        } else {
            Ok(format!("bloodscourge {} {}", self.target, self.venom))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FleshbaneAction {
    pub target: String,
    pub venom: VenomType,
}

impl FleshbaneAction {
    pub fn new(target: String, venom: &'static str) -> Self {
        Self {
            target,
            venom: venom.into(),
        }
    }
}

impl ActiveTransition for FleshbaneAction {
    fn act(&self, timline: &AetTimeline) -> ActivateResult {
        if should_call_venoms(timline) {
            Ok(format!(
                "{};;fleshbane {} {}",
                call_venom(&self.target, self.venom, None),
                self.target,
                self.venom
            ))
        } else {
            Ok(format!("fleshbane {} {}", self.target, self.venom))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AcidAction {
    pub target: String,
}

impl AcidAction {
    pub fn new(target: String) -> Self {
        Self { target }
    }
}

impl ActiveTransition for AcidAction {
    fn act(&self, timline: &AetTimeline) -> ActivateResult {
        Ok(format!("spider acid {}", self.target))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StrandsAction {
    pub target: String,
}

impl StrandsAction {
    pub fn new(target: String) -> Self {
        Self { target }
    }
}

impl ActiveTransition for StrandsAction {
    fn act(&self, timline: &AetTimeline) -> ActivateResult {
        Ok(format!("spider strands {}", self.target))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NegateAction {
    pub target: String,
}

impl NegateAction {
    pub fn new(target: String) -> Self {
        Self { target }
    }
}

impl ActiveTransition for NegateAction {
    fn act(&self, timline: &AetTimeline) -> ActivateResult {
        Ok(format!("spider negate {}", self.target))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WebAction {
    pub target: String,
}

impl WebAction {
    pub fn new(target: String) -> Self {
        Self { target }
    }
}

impl ActiveTransition for WebAction {
    fn act(&self, timline: &AetTimeline) -> ActivateResult {
        Ok(format!("spider web {}", self.target))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IntoxicateAction {
    pub target: String,
}

impl IntoxicateAction {
    pub fn new(target: String) -> Self {
        Self { target }
    }
}

impl ActiveTransition for IntoxicateAction {
    fn act(&self, timline: &AetTimeline) -> ActivateResult {
        Ok(format!("spider intoxicate {}", self.target))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DartshotAction {
    pub target: String,
    pub venom: VenomType,
}

impl DartshotAction {
    pub fn new(target: String, venom: &'static str) -> Self {
        Self {
            target,
            venom: venom.into(),
        }
    }
}

impl ActiveTransition for DartshotAction {
    fn act(&self, timline: &AetTimeline) -> ActivateResult {
        if should_call_venoms(timline) {
            Ok(format!(
                "{};;dartshot {} {}",
                call_venom(&self.target, self.venom, None),
                self.target,
                self.venom
            ))
        } else {
            Ok(format!("dartshot {} {}", self.target, self.venom))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TwinshotAction {
    pub target: String,
    pub venom_0: VenomType,
    pub venom_1: VenomType,
}

impl TwinshotAction {
    pub fn new(target: String, venom_0: VenomType, venom_1: VenomType) -> Self {
        Self {
            target,
            venom_0,
            venom_1,
        }
    }
}

impl ActiveTransition for TwinshotAction {
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        if should_call_venoms(timeline) {
            Ok(format!(
                "{};;twinshot {} {} {}",
                call_venoms(&self.target, self.venom_0, self.venom_1, None),
                self.target,
                self.venom_0,
                self.venom_1
            ))
        } else {
            Ok(format!(
                "twinshot {} {} {}",
                self.target, self.venom_0, self.venom_1
            ))
        }
    }
}

// ==================
// Orgyuk
// ==================

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RakeAction {
    pub target: String,
}

impl RakeAction {
    pub fn new(target: String) -> Self {
        Self { target }
    }
}

impl ActiveTransition for RakeAction {
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(format!("orgyuk rake {}", self.target))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SwipeAction {
    pub target: String,
}

impl SwipeAction {
    pub fn new(target: String) -> Self {
        Self { target }
    }
}

impl ActiveTransition for SwipeAction {
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(format!("orgyuk swipe {}", self.target))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThrowAction {
    pub target: String,
}

impl ThrowAction {
    pub fn new(target: String) -> Self {
        Self { target }
    }
}

impl ActiveTransition for ThrowAction {
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(format!("orgyuk throw {}", self.target))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RoarAction;

impl RoarAction {
    pub fn new() -> Self {
        Self
    }
}

impl ActiveTransition for RoarAction {
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(format!("orgyuk roar"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WeakenAction {
    pub target: String,
}

impl WeakenAction {
    pub fn new(target: String) -> Self {
        Self { target }
    }
}

impl ActiveTransition for WeakenAction {
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(format!("orgyuk weaken {}", self.target))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MawcrushAction {
    pub target: String,
}

impl MawcrushAction {
    pub fn new(target: String) -> Self {
        Self { target }
    }
}

impl ActiveTransition for MawcrushAction {
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(format!("orgyuk mawcrush {}", self.target))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PummelAction {
    pub target: String,
    pub limb: LType,
}

impl PummelAction {
    pub fn new(target: String, limb: LType) -> Self {
        Self { target, limb }
    }
}

impl ActiveTransition for PummelAction {
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(format!(
            "orgyuk pummel {} {}",
            self.target,
            self.limb.to_string()
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FerocityAction;

impl FerocityAction {
    pub fn new() -> Self {
        Self
    }
}

impl ActiveTransition for FerocityAction {
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(format!("ferocity"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArouseAction;

impl ArouseAction {
    pub fn new() -> Self {
        Self
    }
}

impl ActiveTransition for ArouseAction {
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(format!("arouse"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QuickassessAction {
    pub target: String,
}

impl QuickassessAction {
    pub fn new(target: String) -> Self {
        Self { target }
    }
}

impl ActiveTransition for QuickassessAction {
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(format!("quickassess {}", self.target))
    }
}
