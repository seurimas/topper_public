use std::{fmt::Display, ops::DerefMut};

use serde::*;
use topper_bt::unpowered::*;

use crate::{classes::*, db::AetDatabaseModule, timeline::*, types::*};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum SafetyAlert {
    LowHealth,
    LockThreat(LockType, Vec<FType>),
    HighValueThreat(Vec<FType>),
    InstakillThreat(Vec<FType>),
}

impl Display for SafetyAlert {
    fn fmt(&self, f: &mut __private::Formatter<'_>) -> std::fmt::Result {
        match self {
            SafetyAlert::LowHealth => write!(f, "Low Health"),
            SafetyAlert::LockThreat(lock, affs) => write!(f, "Lock Threat ({})", lock),
            SafetyAlert::HighValueThreat(affs) => write!(f, "High Value Threat"),
            SafetyAlert::InstakillThreat(affs) => write!(f, "Instakill Threat"),
        }
    }
}

pub fn gather_alerts(
    timeline: &AetTimeline,
    who: String,
    db: Option<&impl AetDatabaseModule>,
) -> Vec<SafetyAlert> {
    let mut alerts = vec![];
    let HEALTH_ALERT = db
        .and_then(|db| db.get_hint(&"health_alert".to_string()))
        .and_then(|hint| hint.parse::<i32>().ok())
        .unwrap_or(2000);

    if let Some(me_branches) = timeline.state.get_agent(&who) {
        if me_branches
            .iter()
            .any(|me| me.get_stat(SType::Health) < HEALTH_ALERT)
        {
            alerts.push(SafetyAlert::LowHealth);
        }
    }
    alerts
}
