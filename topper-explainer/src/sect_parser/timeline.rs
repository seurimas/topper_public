use topper_aetolia::timeline::{AetPrompt, AetTimeSlice, AetTimeline, AetTimelineState};
use topper_core::timeline::{db::DummyDatabaseModule, BaseTimeline};

use crate::{colored_lines::get_content_of_raw_colored_text, explainer::ExplainerPage};

use super::{
    observations::OBSERVER,
    parser::{parse_me_and_you, parse_prompt_time},
};

pub fn build_time_slices(page: &ExplainerPage) -> Vec<AetTimeSlice> {
    let (me, _you) = parse_me_and_you(page);
    let mut slices = Vec::new();
    let mut slice_lines = Vec::new();
    let mut last_time = 0;
    for (line_idx, line_text) in page.get_body().iter().enumerate() {
        let line_text = get_content_of_raw_colored_text(line_text);
        if let Some(time) = parse_prompt_time(&line_text, last_time) {
            last_time = time;
            let mut slice = AetTimeSlice {
                observations: None,
                gmcp: Vec::new(),
                lines: slice_lines,
                prompt: AetPrompt::Promptless,
                time,
                me: me.clone(),
            };
            slice.observations = Some(OBSERVER.observe(&slice));
            slices.push(slice);
            slice_lines = Vec::new();
        } else {
            slice_lines.push((line_text, line_idx as u32));
        }
    }
    slices
}

pub fn get_timeline_state(
    me: String,
    slices: &Vec<AetTimeSlice>,
    prompt_line_idx: u32,
) -> AetTimelineState {
    let mut timeline = AetTimeline::new();
    timeline.state.me = me;
    for slice in slices {
        if slice
            .lines
            .iter()
            .find(|(_line, idx)| *idx > prompt_line_idx)
            .is_some()
        {
            break;
        }
        timeline.push_time_slice(slice.clone(), None as Option<&DummyDatabaseModule>);
    }
    timeline.state
}
