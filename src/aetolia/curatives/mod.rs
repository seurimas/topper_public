use crate::aetolia::timeline::*;
use crate::aetolia::types::*;
pub mod first_aid;
pub mod statics;
pub use statics::*;

fn noop() -> Box<Fn(&mut AgentState)> {
    Box::new(|_me| {})
}

fn revert_flag(flag: FType, val: bool) -> Box<Fn(&mut AgentState)> {
    Box::new(move |me2: &mut AgentState| me2.set_flag(flag, val))
}

pub fn top_aff(who: &AgentState, afflictions: Vec<FType>) -> Option<FType> {
    let mut top = None;
    for affliction in afflictions.iter() {
        if who.is(*affliction) {
            top = Some(*affliction);
        }
    }
    top
}

pub fn remove_in_order(
    afflictions: Vec<FType>,
) -> Box<Fn(&mut AgentState) -> Box<Fn(&mut AgentState)>> {
    Box::new(move |me| {
        let mut revert = noop();
        for affliction in afflictions.iter() {
            if me.is(*affliction) {
                revert = revert_flag(*affliction, true);
                me.set_flag(*affliction, false);
                break;
            }
        }
        revert
    })
}

pub fn handle_simple_cure_action(
    simple_cure: &SimpleCureAction,
    agent_states: &mut AetTimelineState,
    _before: &Vec<AetObservation>,
    after: &Vec<AetObservation>,
) -> Result<(), String> {
    let mut me = agent_states.get_agent(&simple_cure.caster);
    let results = match &simple_cure.cure_type {
        SimpleCure::Pill(_) => {
            apply_or_infer_balance(&mut me, (BType::Pill, 2.0), after);
            apply_or_infer_cure(&mut me, &simple_cure.cure_type, after)?;
            Ok(())
        }
        SimpleCure::Salve(_salve_name, _salve_loc) => {
            apply_or_infer_balance(&mut me, (BType::Salve, 2.0), after);
            apply_or_infer_cure(&mut me, &simple_cure.cure_type, after)?;
            Ok(())
        }
        SimpleCure::Smoke(_) => {
            apply_or_infer_balance(&mut me, (BType::Smoke, 2.0), after);
            apply_or_infer_cure(&mut me, &simple_cure.cure_type, after)?;
            Ok(())
        } // _ => Ok(()),
    };
    agent_states.set_agent(&simple_cure.caster, me);
    results
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