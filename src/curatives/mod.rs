use crate::timeline::aetolia::*;
use crate::types::*;
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
            observations: vec![AetObservation::SimpleCureAction(SimpleCureAction {
                caster: "Benedicto".into(),
                cure_type: SimpleCure::Pill("coagulation".into()),
            })],
            lines: vec![],
            prompt: Prompt::Blackout,
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
            observations: vec![AetObservation::SimpleCureAction(SimpleCureAction {
                caster: "Benedicto".into(),
                cure_type: SimpleCure::Salve("mending".into(), "skin".into()),
            })],
            lines: vec![],
            prompt: Prompt::Blackout,
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
