mod apply_tests {
    use crate::{timeline::BaseTimeline, topper::observations::ObservationParser};

    use super::super::*;

    lazy_static! {
        static ref observer: ObservationParser<AetObservation> =
            ObservationParser::<AetObservation>::new_from_directory("triggers".to_string())
                .unwrap();
    }
    #[test]
    fn test_eliminate_aff_uncertainty() {
        let mut timeline = AetTimeline::new();
        let mut slice = AetTimeSlice {
            observations: None,
            lines: vec![("Saidenn uses Geometrics Shape on you.".to_string(), 0)],
            gmcp: Vec::new(),
            prompt: AetPrompt::Promptless,
            time: 0,
            me: "Seurimas".into(),
        };
        slice.observations = Some(observer.observe(&slice));
        let mut diagnose_slice = AetTimeSlice {
            observations: Some(vec![AetObservation::Sent("qeb diagnose".to_string())]),
            lines: vec![
                ("You are:".to_string(), 0),
                ("afflicted with laxity.".to_string(), 1),
            ],
            gmcp: Vec::new(),
            prompt: AetPrompt::Promptless,
            time: 0,
            me: "Seurimas".into(),
        };
        let mut observations = observer.observe(&diagnose_slice);
        diagnose_slice
            .observations
            .get_or_insert(Vec::new())
            .append(&mut observations);
        println!("{:?}", diagnose_slice);
        timeline.push_time_slice(slice, None);
        {
            let pre_diagnose: &Vec<AgentState> =
                timeline.state.get_agent(&"Seurimas".to_string()).unwrap();
            assert_eq!(pre_diagnose.len(), 3);
        }
        timeline.push_time_slice(diagnose_slice, None);
        {
            let post_diagnose: &Vec<AgentState> =
                timeline.state.get_agent(&"Seurimas".to_string()).unwrap();
            assert_eq!(post_diagnose.len(), 1);
        }
    }
}
