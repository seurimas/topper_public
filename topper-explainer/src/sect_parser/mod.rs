use std::cmp::Ordering;

use topper_aetolia::timeline::{AetTimeSlice, AetTimeline, AetTimelineTrait, BaseTimeline};
use topper_core::timeline::db::DummyDatabaseModule;

mod loader;
mod observations;
mod parser;
mod timeline;

pub use loader::*;
pub use parser::{is_prompt, parse_me_and_you, AetoliaSectParser};
pub use timeline::{build_time_slices, get_timeline_state};

pub fn get_selected_slice(time_slices: &Vec<AetTimeSlice>, line_idx: usize) -> usize {
    time_slices
        .binary_search_by(|time_slice| {
            if time_slice
                .lines
                .iter()
                .any(|(_, time_slice_line_idx)| *time_slice_line_idx as usize == line_idx)
            {
                Ordering::Equal
            } else if time_slice
                .lines
                .iter()
                .any(|(_, time_slice_line_idx)| *time_slice_line_idx as usize > line_idx)
            {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        })
        .unwrap_or(if line_idx > 100 {
            time_slices.len() - 1
        } else {
            0
        })
}

pub fn update_timeline(
    timeline: &mut AetTimeline,
    time_slices: &Vec<AetTimeSlice>,
    line_idx: usize,
) -> Option<usize> {
    if time_slices.len() == 0 {
        return None;
    }
    let mut prior_time = timeline.state.time;
    let selected_slice = get_selected_slice(time_slices, line_idx);
    let new_time = time_slices.get(selected_slice).unwrap().time;
    if new_time < prior_time {
        prior_time = 0;
        timeline.reset(true);
    }
    for time_slice in time_slices.iter() {
        if time_slice.time > prior_time && time_slice.time <= new_time {
            timeline.push_time_slice(time_slice.clone(), None as Option<&DummyDatabaseModule>);
        } else if time_slice.time > new_time {
            break;
        }
    }
    if prior_time != timeline.state.time {
        Some(
            timeline
                .slices
                .last()
                .and_then(|slice| slice.lines.last())
                .map(|(_, line_idx)| *line_idx)
                .unwrap_or_default() as usize,
        )
    } else {
        None
    }
}
