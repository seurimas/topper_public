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
pub struct MonkComboAction {
    pub combo: MonkCombo,
    pub target: String,
}

impl ActiveTransition for MonkComboAction {
    fn act(&self, timline: &AetTimeline) -> ActivateResult {
        match &self.combo {
            MonkCombo::Standard(_, attacks) => Ok(format!(
                "combo {} {} {} {}",
                self.target,
                attacks[0].param_str(),
                attacks[1].param_str(),
                attacks[2].param_str()
            )),
            MonkCombo::Cobra(attacks) => Ok(format!(
                "combo {} {} {}",
                self.target,
                attacks[0].param_str(),
                attacks[1].param_str()
            )),
            MonkCombo::ChangeStance(stance, attacks) => Ok(format!(
                "combo {} {} {} {}",
                self.target,
                stance.param_str(),
                attacks[0].param_str(),
                attacks[1].param_str()
            )),
        }
    }
}
