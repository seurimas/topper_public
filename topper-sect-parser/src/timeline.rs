use crate::bindings::*;
use regex::Regex;
use std::marker::PhantomData;
use topper_aetolia::timeline::{
    aet_observation_creator, AetObservation, AetPrompt, AetTimeSlice, AetTimeline, AetTimelineTrait,
};
use topper_core::{
    observations::ObservationParser,
    timeline::{
        db::{DatabaseModule, DummyDatabaseModule},
        BaseTimeline,
    },
};
use web_sys::Element;

lazy_static! {
    static ref PROMPT_REGEX: Regex =
        Regex::new(r"\[(?P<hour>\d\d):(?P<minute>\d\d):(?P<second>\d\d):(?P<centi>\d\d)\]")
            .unwrap();
    static ref WHO_REGEX: Regex = Regex::new(r"^Who:\s+(?P<who>\w+)$").unwrap();
}

lazy_static! {
    static ref OBSERVATIONS: Vec<String> = {
        let mut results = vec![];
        results.push(include_str!("../../triggers/Attack Observations.json").to_string());
        results.push(include_str!("../../triggers/CombatActions.json").to_string());
        results.push(include_str!("../../triggers/Cures.json").to_string());
        results.push(include_str!("../../triggers/Hypnosis Spoofs.json").to_string());
        results.push(include_str!("../../triggers/Indorani Spoof.json").to_string());
        results.push(include_str!("../../triggers/Luminary Spoof.json").to_string());
        results.push(include_str!("../../triggers/Observations.json").to_string());
        results.push(include_str!("../../triggers/Simple Aff Messages.json").to_string());
        results.push(include_str!("../../triggers/Sentinel Spoof.json").to_string());
        results.push(include_str!("../../triggers/Subterfuge Spoofs.json").to_string());
        results.push(include_str!("../../triggers/Titan Lord Spoofs.json").to_string());
        results.push(include_str!("../../triggers/Wielding.json").to_string());
        results.push(include_str!("../../triggers/Zealot Spoof.json").to_string());
        results.push(include_str!("../../triggers/Lists/Diagnose.json").to_string());
        results.push(include_str!("../../triggers/Lists/Wounds.json").to_string());
        results
    };
    static ref OBSERVER: ObservationParser<AetObservation> = {
        ObservationParser::<AetObservation>::new_from_strings(
            OBSERVATIONS.to_vec(),
            aet_observation_creator,
        )
        .unwrap()
    };
}

pub fn parse_time_slices(line_nodes: &Vec<Element>) -> Vec<AetTimeSlice> {
    let mut slices = vec![];
    let mut lines = vec![];
    let mut last_time = 0;
    let mut me = String::new();
    for (line_idx, line_node) in line_nodes.iter().enumerate() {
        let line_text = line_node.text_content().unwrap();
        let line_text = line_text.trim().to_string();
        if line_text.contains("\n") {
            for line_text in line_text.split("\n") {
                lines.push((line_text.to_string(), line_idx as u32));
            }
        } else {
            lines.push((line_text.to_string(), line_idx as u32));
        }
        if let Some(captures) = WHO_REGEX.captures(line_text.as_ref()) {
            if let Some(who) = captures.name("who") {
                me = who.as_str().to_string();
            }
        }
        if let Some(captures) = PROMPT_REGEX.captures(line_text.as_ref()) {
            if let (Some(hour), Some(minute), Some(second), Some(centi)) = (
                captures.name("hour"),
                captures.name("minute"),
                captures.name("second"),
                captures.name("centi"),
            ) {
                let hour: i32 = hour.as_str().parse().unwrap();
                let minute: i32 = minute.as_str().parse().unwrap();
                let second: i32 = second.as_str().parse().unwrap();
                let centi: i32 = centi.as_str().parse().unwrap();
                let mut time = centi + (((((hour * 60) + minute) * 60) + second) * 100);
                if time < last_time {
                    // It's a braaand neww day, and the sun is hiiigh.
                    time = time + (24 * 360000);
                }
                last_time = time;
                let mut slice = AetTimeSlice {
                    observations: None,
                    gmcp: Vec::new(),
                    lines: lines,
                    prompt: AetPrompt::Promptless,
                    time,
                    me: me.clone(),
                };
                slice.observations = Some(OBSERVER.observe(&slice));
                slices.push(slice);
                lines = vec![];
            }
        }
    }
    slices
}

pub fn update_timeline(
    timeline: &mut AetTimeline,
    time_slices: &Vec<AetTimeSlice>,
    line_idx: usize,
) {
    timeline.reset(true);
    for time_slice in time_slices.iter() {
        if time_slice
            .lines
            .iter()
            .any(|(_, time_slice_line_idx)| *time_slice_line_idx as usize <= line_idx)
        {
            log("Here");
            timeline.push_time_slice(time_slice.clone(), None as Option<&DummyDatabaseModule>);
        }
    }
}
