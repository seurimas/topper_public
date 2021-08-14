use crate::aetolia::timeline::*;
use crate::aetolia::types::*;
pub mod first_aid;
pub mod statics;
pub use statics::*;

pub fn top_aff(who: &AgentState, afflictions: Vec<FType>) -> Option<FType> {
    for affliction in afflictions.iter() {
        if who.is(*affliction) {
            return Some(*affliction);
        }
    }
    None
}

pub fn top_missing_aff(who: &AgentState, afflictions: Vec<FType>) -> Option<FType> {
    for affliction in afflictions.iter() {
        if !who.is(*affliction) {
            return Some(*affliction);
        }
    }
    None
}

pub fn remove_in_order(afflictions: Vec<FType>) -> Box<Fn(&mut AgentState)> {
    Box::new(move |me| {
        for affliction in afflictions.iter() {
            if me.is(*affliction) {
                me.set_flag(*affliction, false);
                return;
            }
        }
        // No affs found. Assume this is a poor quality branch.
        me.branch_state.strike();
    })
}

pub fn handle_simple_cure_action(
    simple_cure: &SimpleCureAction,
    agent_states: &mut AetTimelineState,
    _before: &Vec<AetObservation>,
    after: &Vec<AetObservation>,
) -> Result<(), String> {
    let observations = after.clone();
    let cure_type = simple_cure.cure_type.clone();
    let first_person = agent_states.me.eq(&simple_cure.caster);
    for_agent_closure(
        agent_states,
        &simple_cure.caster,
        Box::new(move |me| {
            let mut seared = false;
            if let Some(AetObservation::Proc(proc)) = observations.get(1) {
                if proc.skill.eq("Sear") {
                    seared = true;
                }
            }
            apply_or_infer_cure(me, &cure_type, &observations, first_person);
            match &cure_type {
                SimpleCure::Pill(_) => {
                    apply_or_infer_balance(me, (BType::Pill, 2.0), &observations);
                }
                SimpleCure::Salve(_salve_name, _salve_loc) => {
                    apply_or_infer_balance(me, (BType::Salve, 2.0), &observations);
                }
                SimpleCure::Smoke(_) => {
                    apply_or_infer_balance(me, (BType::Smoke, 2.0), &observations);
                }
            };
        }),
    );
    Ok(())
}

#[derive(Debug, Default)]
pub struct CureDepth {
    pub time: CType,
    pub cures: CType,
    pub affs: Vec<FType>,
}

pub struct CureDepths {
    salve: CureDepth,
    pill: CureDepth,
    smoke: CureDepth,
    focus: CureDepth,
}

const PILL_TIME: CType = 150;
const PANACEA_TIME: CType = 250;
const SALVE_TIME: CType = 150;
const RESTORATION_TIME: CType = 250;
const SMOKE_TIME: CType = 150;

fn get_cure_depth_locked(me: &AgentState, target_aff: FType, checked: u32) -> CureDepth {
    let mut val = CureDepth::default();
    if let Some(salve) = AFFLICTION_SALVES.get(&target_aff) {
        if me.is(FType::Slickness) && checked < 2 {
            val = get_cure_depth_locked(me, FType::Slickness, checked + 1);
        }
        for aff in SALVE_CURE_ORDERS.get(salve).unwrap() {
            if me.is(*aff) {
                val.affs.push(*aff);
                val.time = val.time + SALVE_TIME;
                if salve.0.eq("restoration") {
                    val.time = val.time + RESTORATION_TIME;
                }
                val.cures = val.cures + 1;
            }
            if *aff == target_aff {
                if !salve.0.eq("restoration") {
                    val.time = val.time - SALVE_TIME;
                    if !me.balanced(BType::Salve) {
                        val.time = val.time + me.get_raw_balance(BType::Salve);
                    }
                }
                break;
            }
        }
        val
    } else if let Some(smoke) = AFFLICTION_SMOKES.get(&target_aff) {
        if me.is(FType::Asthma) && checked < 2 {
            val = get_cure_depth_locked(me, FType::Asthma, checked + 1);
        }
        for aff in SMOKE_CURE_ORDERS.get(smoke).unwrap() {
            if me.is(*aff) {
                val.affs.push(*aff);
                val.time = val.time + SMOKE_TIME;
                val.cures = val.cures + 1;
            }
            if *aff == target_aff {
                val.time = val.time - SMOKE_TIME;
                if !me.balanced(BType::Smoke) {
                    val.time = val.time + me.get_raw_balance(BType::Smoke);
                }
                break;
            }
        }
        val
    } else if let Some(pill) = AFFLICTION_PILLS.get(&target_aff) {
        if me.is(FType::Anorexia) && checked < 2 {
            val = get_cure_depth_locked(me, FType::Anorexia, checked + 1);
        }
        for aff in PILL_CURE_ORDERS.get(pill).unwrap() {
            if me.is(*aff) {
                val.affs.push(*aff);
                val.time = val.time + PILL_TIME;
                if pill.eq("panacea") {
                    val.time = val.time + PANACEA_TIME;
                }
                val.cures = val.cures + 1;
            }
            if *aff == target_aff {
                if !pill.eq("panacea") {
                    val.time = val.time - PILL_TIME;
                    if !me.balanced(BType::Pill) {
                        val.time = val.time + me.get_raw_balance(BType::Pill);
                    }
                }
                break;
            }
        }
        val
    } else {
        CureDepth::default()
    }
}

pub fn get_cure_depth(me: &AgentState, target_aff: FType) -> CureDepth {
    get_cure_depth_locked(me, target_aff, 0)
}

pub fn get_cure_depths(me: &AgentState) -> CureDepths {
    let mut salve = CureDepth::default();
    let mut pill = CureDepth::default();
    let mut smoke = CureDepth::default();
    let mut focus = CureDepth::default();

    for aff in AFFLICTION_PILLS.keys() {
        if me.is(*aff) {
            pill.affs.push(*aff);
        }
    }

    for aff in AFFLICTION_SMOKES.keys() {
        if me.is(*aff) {
            smoke.affs.push(*aff);
        }
    }

    for aff in AFFLICTION_SALVES.keys() {
        if me.is(*aff) {
            salve.affs.push(*aff);
        }
    }

    for aff in MENTAL_AFFLICTIONS.to_vec().iter() {
        if me.is(*aff) {
            focus.affs.push(*aff);
        }
    }

    CureDepths {
        salve,
        pill,
        smoke,
        focus,
    }
}

#[cfg(test)]
#[path = "./tests/timeline_tests.rs"]
mod curative_timeline_tests;

#[cfg(test)]
#[path = "./tests/cure_depth_tests.rs"]
mod cure_depth_tests;
