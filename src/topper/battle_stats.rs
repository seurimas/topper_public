use crate::classes::{get_attack, Class};
use crate::timeline::*;
use crate::topper::db::DatabaseModule;
use crate::topper::{TopperMessage, TopperModule, TopperRequest, TopperResponse};
use crate::types::*;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct PlayerStats {
    afflictions: Vec<String>,
    limbs: HashMap<String, LimbState>,
    balances: HashMap<String, f32>,
    warnings: Vec<String>,
    lock_duration: Option<f32>,
    class: String,
}

fn get_hypno_warning(state: &AgentState) -> Option<String> {
    if let Some(aff) = state.hypno_state.get_next_hypno_aff() {
        Some(format!("<magenta>Next aff: <red>{:?}", aff))
    } else if state.hypno_state.hypnotized || state.hypno_state.sealed.is_some() {
        Some(format!(
            "<magenta>Stack size: <red>{} {}",
            state.hypno_state.hypnosis_stack.len(),
            if state.hypno_state.sealed.is_some() {
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
    use crate::classes::get_venoms;
    use crate::classes::syssin::{should_lock, SOFT_STACK};
    if should_lock(None, state, &get_venoms(SOFT_STACK.to_vec(), 3, &state)) {
        Some(format!("<pink>Close to a lock!"))
    } else {
        None
    }
}

impl PlayerStats {
    pub fn new() -> Self {
        PlayerStats {
            afflictions: Vec::new(),
            limbs: HashMap::new(),
            warnings: Vec::new(),
            balances: HashMap::new(),
            lock_duration: None,
            class: "".to_string(),
        }
    }
    pub fn for_player(state: &AgentState, class: Option<Class>) -> Self {
        let mut afflictions = Vec::new();
        for aff in state.flags.aff_iter() {
            if state.get_count(aff) > 1 {
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
            afflictions,
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

impl<'s> TopperModule<'s> for BattleStatsModule {
    type Siblings = (&'s Timeline, &'s Option<String>, &'s DatabaseModule);
    fn handle_message(
        &mut self,
        message: &TopperMessage,
        (timeline, target, db): Self::Siblings,
    ) -> Result<TopperResponse, String> {
        match message {
            TopperMessage::Request(request) => match request {
                TopperRequest::BattleStats(_) => Ok(TopperResponse::battle_stats(
                    get_battle_stats(timeline, target, db, &self.plan),
                )),
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
    pub target_stats: Option<PlayerStats>,
    pub plan: String,
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
    timeline: &Timeline,
    target: &Option<String>,
    db: &DatabaseModule,
    plan: &Option<String>,
) -> BattleStats {
    let mut lines = Vec::new();
    let my_stats = PlayerStats::for_player(
        &timeline.state.borrow_me(),
        db.get_class(&timeline.who_am_i()),
    );
    let target_stats = if let Some(target) = target {
        Some(PlayerStats::for_player(
            &timeline.state.borrow_agent(target),
            db.get_class(target),
        ))
    } else {
        None
    };
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
    for timeslice in timeline.slices.iter().rev() {
        for observation in timeslice.observations.iter().rev() {
            if lines_available <= 0 {
                break;
            }
            if let Observation::CombatAction(combat_action) = observation {
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
                if let Observation::SimpleCureAction(simple_cure) = observation {
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
        plan: plan_str,
    }
}
