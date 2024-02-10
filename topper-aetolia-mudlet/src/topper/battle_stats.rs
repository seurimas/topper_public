use serde::Serialize;
use std::collections::HashMap;
use topper_aetolia::classes::infiltrator::{get_hypno_stack, get_hypno_stack_name};
use topper_aetolia::classes::{get_attack, Class, LockType};
use topper_aetolia::curatives::gather_alerts;
use topper_aetolia::db::AetDatabaseModule;
use topper_aetolia::timeline::*;
use topper_aetolia::types::*;
use topper_core::timeline::db::DatabaseModule;
use topper_core_mudlet::topper::{TopperMessage, TopperModule, TopperRequest, TopperResponse};

use super::db::AetMudletDatabaseModule;

#[derive(Serialize)]
pub struct PlayerStats {
    name: String,
    afflictions: Vec<String>,
    unknowns: isize,
    limbs: HashMap<String, LimbState>,
    balances: HashMap<String, f32>,
    warnings: Vec<String>,
    lock_duration: Option<f32>,
    class: String,
}

fn get_hypno_warning(state: &AgentState) -> Option<String> {
    if let Some(aff) = state.hypno_state.get_next_hypno_aff() {
        Some(format!("<magenta>Next aff: <red>{:?}", aff))
    } else if state.hypno_state.is_hypnotized() || state.hypno_state.is_sealed() {
        Some(format!(
            "<magenta>Stack size: <red>{} {}",
            state.hypno_state.suggestion_count(),
            if state.hypno_state.is_sealed() {
                "SEALED"
            } else {
                ""
            },
        ))
    } else {
        None
    }
}

fn get_lock_warning(state: &AgentState) -> Option<String> {
    use topper_aetolia::classes::get_venoms;
    if LockType::Soft.affs_to_lock(state) == 1 {
        Some(format!("<pink>Close to a lock!"))
    } else if LockType::Buffered.affs_to_lock(state) <= 2 {
        Some(format!("<pink>Close to a lock!"))
    } else if LockType::Hard.affs_to_lock(state) <= 2 {
        Some(format!("<magenta>Close to a lock!"))
    } else {
        None
    }
}

impl PlayerStats {
    pub fn new() -> Self {
        PlayerStats {
            name: String::default(),
            afflictions: Vec::new(),
            unknowns: 0,
            limbs: HashMap::new(),
            warnings: Vec::new(),
            balances: HashMap::new(),
            lock_duration: None,
            class: "".to_string(),
        }
    }
    pub fn for_player(name: String, state: &AgentState, class: Option<Class>) -> Self {
        let mut afflictions = Vec::new();
        for aff in state.flags.aff_iter() {
            if state.hidden_state.is_guessed(aff) {
                afflictions.push(format!("{:?}?", aff));
            } else if state.get_count(aff) > 1 {
                afflictions.push(format!("{:?}x{}", aff, state.get_count(aff)));
            } else {
                afflictions.push(format!("{:?}", aff));
            }
        }
        let mut limbs = HashMap::new();
        for limb in vec![
            LType::HeadDamage,
            LType::TorsoDamage,
            LType::LeftArmDamage,
            LType::RightArmDamage,
            LType::LeftLegDamage,
            LType::RightLegDamage,
        ]
        .iter()
        {
            limbs.insert(limb.to_string(), state.get_limb_state(*limb));
        }
        let mut warnings = Vec::new();
        if let Some(warning) = get_hypno_warning(&state) {
            warnings.push(warning);
        }
        if let Some(warning) = get_lock_warning(&state) {
            warnings.push(warning);
        }
        let mut balances = HashMap::new();
        balances.insert("Tree".to_string(), state.get_balance(BType::Tree));
        balances.insert("Focus".to_string(), state.get_balance(BType::Focus));
        balances.insert("Fitness".to_string(), state.get_balance(BType::Fitness));
        balances.insert(
            "ClassCure1".to_string(),
            state.get_balance(BType::ClassCure1),
        );
        balances.insert(
            "Rebounding".to_string(),
            state.get_balance(BType::Rebounding),
        );
        let lock_duration = state.lock_duration();
        /* let first_aid_cure = DEFAULT_FIRST_AID
        .get_next_cure(&"", state)
        .map(|(aff, action)| {
            if action.is_tree() {
                "Tree".to_string()
            } else {
                format!("{:?}", aff)
            }
        }); */
        PlayerStats {
            name,
            afflictions,
            unknowns: state.hidden_state.unknown(),
            limbs,
            warnings,
            balances,
            lock_duration,
            class: class.map_or_else(|| "Unknown".to_string(), |class| format!("{}", class)),
        }
    }
}

pub struct BattleStatsModule {
    plan: Option<String>,
}

impl BattleStatsModule {
    pub fn new() -> Self {
        BattleStatsModule { plan: None }
    }
}

impl<'s> TopperModule<'s, AetTimeSlice, BattleStats> for BattleStatsModule {
    type Siblings = (
        &'s mut AetTimeline,
        &'s Option<String>,
        &'s AetMudletDatabaseModule,
    );
    fn handle_message(
        &mut self,
        message: &TopperMessage<AetTimeSlice>,
        (mut timeline, target, db): Self::Siblings,
    ) -> Result<TopperResponse<BattleStats>, String> {
        match message {
            TopperMessage::Request(request) => match request {
                TopperRequest::BattleStats(when) => {
                    timeline.update_time(*when);
                    Ok(TopperResponse::battle_stats(get_battle_stats(
                        timeline, target, db, &self.plan,
                    )))
                }
                TopperRequest::Plan(plan) => {
                    self.plan = Some(plan.to_string());
                    Ok(TopperResponse::battle_stats(get_battle_stats(
                        timeline, target, db, &self.plan,
                    )))
                }
                _ => Ok(TopperResponse::silent()),
            },
            _ => Ok(TopperResponse::silent()),
        }
    }
}

#[derive(Serialize)]
pub struct BattleStats {
    pub feed: Vec<String>,
    pub my_stats: PlayerStats,
    pub alerts: Vec<String>,
    pub target_stats: Option<PlayerStats>,
    pub plan: String,
    pub class_state: String,
}

fn format_self_limbs(state: &AgentState) -> String {
    format!("<green>My Limbs: [{:?}]", state.limb_damage)
}

fn format_target_limbs(state: &AgentState) -> String {
    format!("<red>Target Limbs: [{:?}]", state.limb_damage)
}

fn format_combat_action(combat_action: &CombatAction) -> Vec<String> {
    let lines = vec![format!(
        "{} ={}= @ {}",
        combat_action.caster, combat_action.skill, combat_action.target
    )];
    lines
}

pub fn get_battle_stats(
    timeline: &AetTimeline,
    target: &Option<String>,
    db: &AetMudletDatabaseModule,
    plan: &Option<String>,
) -> BattleStats {
    let mut lines = Vec::new();
    let my_stats = PlayerStats::for_player(
        timeline.who_am_i().clone(),
        &timeline.state.borrow_me(),
        db.get_class(&timeline.who_am_i()),
    );
    let target_stats = if let Some(target) = target {
        Some(PlayerStats::for_player(
            target.clone(),
            &timeline.state.borrow_agent(target),
            db.get_class(target),
        ))
    } else {
        None
    };
    let alerts = gather_alerts(&timeline, timeline.who_am_i(), Some(db))
        .iter()
        .map(|alert| alert.to_string())
        .collect();
    let mut lines_available = 16;
    lines.push(format_self_limbs(&timeline.state.borrow_me()));
    if let Some(target) = target {
        let target = timeline.state.borrow_agent(target);
        lines.push(format_target_limbs(&target));
    }
    let plan_str = if let (Some(plan), Some(target)) = (plan, target) {
        if !plan.eq("") {
            get_attack(timeline, &timeline.who_am_i(), target, &plan, Some(db))
        } else {
            "".to_string()
        }
    } else {
        "".to_string()
    };
    let class_state = match my_stats.class.as_ref() {
        "Infiltrator" => format!(
            "{}: {:?}",
            get_hypno_stack_name(
                &timeline,
                target.as_ref().unwrap_or(&"".to_string()),
                plan.as_ref().unwrap_or(&"".to_string())
            ),
            get_hypno_stack(
                &timeline,
                target.as_ref().unwrap_or(&"".to_string()),
                plan.as_ref().unwrap_or(&"".to_string()),
                Some(db),
            ),
        ),
        "Bard" => topper_aetolia::classes::bard::get_class_state(
            &timeline,
            target.as_ref().unwrap_or(&"".to_string()),
            plan.as_ref().unwrap_or(&"".to_string()),
            Some(db),
        ),
        "Predator" => topper_aetolia::classes::predator::get_class_state(
            &timeline,
            target.as_ref().unwrap_or(&"".to_string()),
            plan.as_ref().unwrap_or(&"".to_string()),
            Some(db),
        ),
        _ => "".to_string(),
    };
    for timeslice in timeline.slices.iter().rev() {
        for observation in timeslice.observations.iter().flatten().rev() {
            if lines_available <= 0 {
                break;
            }
            if let AetObservation::CombatAction(combat_action) = observation {
                if let Some(who) = target {
                    if !who.eq_ignore_ascii_case(&combat_action.target)
                        && !who.eq_ignore_ascii_case(&combat_action.caster)
                        && !who.eq_ignore_ascii_case(&timeline.who_am_i())
                    {
                        continue;
                    }
                }
                let new_lines = format_combat_action(combat_action);
                for line in new_lines.iter().rev() {
                    if lines_available > 0 {
                        lines.push(line.to_string());
                        lines_available -= 1;
                    }
                }
            }
            if target.is_none() {
                if let AetObservation::SimpleCureAction(simple_cure) = observation {
                    lines.push(format!(
                        "{} <= {:?}",
                        simple_cure.caster, simple_cure.cure_type
                    ));
                    lines_available -= 1;
                }
            }
        }
    }
    BattleStats {
        feed: lines,
        my_stats,
        target_stats,
        alerts,
        plan: plan_str,
        class_state,
    }
}
