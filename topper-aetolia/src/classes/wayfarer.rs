use crate::timeline::*;
use crate::types::*;

lazy_static! {
    pub static ref SHATTER_AFFS: Vec<FType> = vec![
        FType::Dizziness,
        FType::Stupidity,
        FType::Dementia,
        FType::Confusion,
    ];
}

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut AetTimelineState,
    _before: &Vec<AetObservation>,
    after: &Vec<AetObservation>,
) -> Result<(), String> {
    match combat_action.skill.as_ref() {
        "Shatter" => {
            let observations = after.clone();
            let perspective = agent_states.get_perspective(&combat_action);
            if perspective != Perspective::Bystander {
                for_agent_uncertain_closure(
                    agent_states,
                    &combat_action.target,
                    Box::new(move |you| {
                        apply_or_infer_random_afflictions(
                            you,
                            &observations,
                            perspective,
                            Some((
                                1,
                                SHATTER_AFFS
                                    .iter()
                                    .filter(|aff| !you.is(**aff))
                                    .map(|aff| *aff)
                                    .collect(),
                            )),
                        )
                    }),
                );
            }
        }
        _ => {}
    }
    Ok(())
}

mod tests {
    use super::*;

    #[test]
    fn test_handle_shatter_and_focus_for_stupidity() {
        let mut timeline = AetTimeline::new();
        let combat_action = CombatAction::observation("A", "Fury", "Shatter", "", "B");
        let hidden = AetObservation::HiddenAff;
        timeline.test_push_time_slice(AetTimeSlice::new(
            "B".to_string(),
            0,
            vec![combat_action, hidden],
        ));
        assert_eq!(timeline.state.get_agent(&"B".to_string()).unwrap().len(), 4);
        let combat_action = CombatAction::observation("B", "Survival", "Focus", "", "");
        let cure = AetObservation::Cured("stupidity".to_string());
        timeline.test_push_time_slice(AetTimeSlice::new(
            "B".to_string(),
            10,
            vec![combat_action, cure],
        ));
        timeline.state.strikeout();
        assert_eq!(timeline.state.get_agent(&"B".to_string()).unwrap().len(), 1);
    }
}
