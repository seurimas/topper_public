use crate::{db::AetDatabaseModule, observables::*, timeline::*, types::*};

pub struct DodgeAction {
    pub caster: String,
    pub dodge_type: DodgeType,
}

impl DodgeAction {
    pub fn new(caster: String, dodge_type: DodgeType) -> Self {
        DodgeAction { caster, dodge_type }
    }
}

impl ActiveTransition for DodgeAction {
    fn simulate(&self, _timeline: &AetTimeline) -> Vec<ProbableEvent> {
        Vec::new()
    }
    fn act(&self, _timeline: &AetTimeline) -> ActivateResult {
        Ok(match self.dodge_type {
            DodgeType::Unknown => "dodge all".to_string(),
            DodgeType::Melee => "dodge melee".to_string(),
            DodgeType::Ranged => "dodge ranged".to_string(),
            DodgeType::Charge => "dodge charge".to_string(),
            DodgeType::Upset => "dodge upset".to_string(),
        })
    }
}

pub fn get_wanted_dodge<DB: AetDatabaseModule + ?Sized>(
    timeline: &AetTimeline,
    db: Option<&DB>,
) -> DodgeType {
    DodgeType::Melee
}
