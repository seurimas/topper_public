use crate::aetolia::timeline::*;
use crate::aetolia::types::*;
pub mod first_aid;
pub mod statics;
pub use statics::*;

#[cfg(test)]
mod timeline_tests {
    use super::*;
    use crate::timeline::*;

    #[test]
    fn test_pill() {
        let mut timeline = AetTimeline::new();
        {
            let mut updated_seur = timeline.state.get_agent(&"Seurimas".to_string());
            updated_seur.set_flag(FType::ThinBlood, true);
            timeline.state.set_agent(&"Seurimas".into(), updated_seur);
        }
        {
            let mut updated_bene = timeline.state.get_agent(&"Benedicto".to_string());
            updated_bene.set_flag(FType::ThinBlood, true);
            timeline.state.set_agent(&"Benedicto".into(), updated_bene);
        }
        let coag_slice = TimeSlice {
            observations: Some(vec![AetObservation::SimpleCureAction(SimpleCureAction {
                caster: "Benedicto".into(),
                cure_type: SimpleCure::Pill("coagulation".into()),
            })]),
            lines: vec![],
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(coag_slice);
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Pill), true);
        assert_eq!(seur_state.is(FType::ThinBlood), true);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Pill), false);
        assert_eq!(bene_state.is(FType::ThinBlood), false);
    }

    #[test]
    fn test_mending() {
        let mut timeline = AetTimeline::new();
        {
            let mut updated_seur = timeline.state.get_agent(&"Seurimas".to_string());
            updated_seur.set_flag(FType::LeftArmBroken, true);
            timeline.state.set_agent(&"Seurimas".into(), updated_seur);
        }
        {
            let mut updated_bene = timeline.state.get_agent(&"Benedicto".to_string());
            updated_bene.set_flag(FType::LeftLegBroken, true);
            timeline.state.set_agent(&"Benedicto".into(), updated_bene);
        }
        let coag_slice = TimeSlice {
            observations: Some(vec![AetObservation::SimpleCureAction(SimpleCureAction {
                caster: "Benedicto".into(),
                cure_type: SimpleCure::Salve("mending".into(), "skin".into()),
            })]),
            lines: vec![],
            prompt: AetPrompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(coag_slice);
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Salve), true);
        assert_eq!(seur_state.is(FType::LeftArmBroken), true);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Salve), false);
        assert_eq!(bene_state.is(FType::LeftArmBroken), false);
    }
}
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

#[cfg(test)]
mod cure_depth_tests {
    use super::*;

    #[test]
    fn test_pill() {
        let mut agent = AgentState::default();
        agent.set_flag(FType::Clumsiness, true);
        agent.set_flag(FType::Asthma, true);
        let cure_depth = get_cure_depth(&agent, FType::Asthma);
        assert_eq!(cure_depth.affs, vec![FType::Clumsiness, FType::Asthma]);
        assert_eq!(cure_depth.time, 150);
        assert_eq!(cure_depth.cures, 2);
    }

    #[test]
    fn test_pill_off_bal() {
        let mut agent = AgentState::default();
        agent.set_flag(FType::Clumsiness, true);
        agent.set_flag(FType::Asthma, true);
        agent.set_balance(BType::Pill, 1.0);
        let cure_depth = get_cure_depth(&agent, FType::Asthma);
        assert_eq!(cure_depth.affs, vec![FType::Clumsiness, FType::Asthma]);
        assert_eq!(cure_depth.time, 250);
        assert_eq!(cure_depth.cures, 2);
    }

    #[test]
    fn test_salve() {
        let mut agent = AgentState::default();
        agent.set_flag(FType::MuscleSpasms, true);
        agent.set_flag(FType::Stiffness, true);
        let cure_depth = get_cure_depth(&agent, FType::Stiffness);
        assert_eq!(cure_depth.affs, vec![FType::MuscleSpasms, FType::Stiffness]);
        assert_eq!(cure_depth.time, 150);
        assert_eq!(cure_depth.cures, 2);
    }

    #[test]
    fn test_smoke_asthma() {
        let mut agent = AgentState::default();
        agent.set_flag(FType::Disfigurement, true);
        agent.set_flag(FType::Asthma, true);
        let cure_depth = get_cure_depth(&agent, FType::Disfigurement);
        assert_eq!(cure_depth.affs, vec![FType::Asthma, FType::Disfigurement]);
        assert_eq!(cure_depth.time, 0);
        assert_eq!(cure_depth.cures, 2);
    }

    #[test]
    fn test_smoke_asthma_anorexia() {
        let mut agent = AgentState::default();
        agent.set_flag(FType::Disfigurement, true);
        agent.set_flag(FType::Asthma, true);
        agent.set_flag(FType::Anorexia, true);
        let cure_depth = get_cure_depth(&agent, FType::Disfigurement);
        assert_eq!(
            cure_depth.affs,
            vec![FType::Anorexia, FType::Asthma, FType::Disfigurement]
        );
        assert_eq!(cure_depth.time, 0);
        assert_eq!(cure_depth.cures, 3);
    }

    #[test]
    fn test_locked() {
        let mut agent = AgentState::default();
        agent.set_flag(FType::Slickness, true);
        agent.set_flag(FType::Asthma, true);
        agent.set_flag(FType::Anorexia, true);
        let cure_depth = get_cure_depth(&agent, FType::Slickness);
        assert_eq!(
            cure_depth.affs,
            vec![FType::Anorexia, FType::Asthma, FType::Slickness]
        );
        assert_eq!(cure_depth.time, 0);
        assert_eq!(cure_depth.cures, 3);
    }

    #[test]
    fn test_aeon() {
        let mut agent = AgentState::default();
        agent.set_flag(FType::Clumsiness, true);
        agent.set_flag(FType::Asthma, true);
        agent.set_flag(FType::Aeon, true);
        let cure_depth = get_cure_depth(&agent, FType::Aeon);
        assert_eq!(
            cure_depth.affs,
            vec![FType::Clumsiness, FType::Asthma, FType::Aeon]
        );
        assert_eq!(cure_depth.time, 150);
        assert_eq!(cure_depth.cures, 3);
    }
}

#[derive(Debug, Default)]
pub struct CureDepth {
    time: CType,
    cures: CType,
    affs: Vec<FType>,
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
