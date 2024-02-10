use crate::timeline::types::AetTimeline;

use super::VenomType;

pub fn check_config_str(timeline: &AetTimeline, value: &String) -> String {
    timeline.state.get_my_hint(value).unwrap_or("n".to_string())
}

pub fn check_config(timeline: &AetTimeline, value: &String) -> bool {
    timeline
        .state
        .get_my_hint(value)
        .unwrap_or("false".to_string())
        .eq(&"true")
}

pub fn check_config_int(timeline: &AetTimeline, value: &String) -> i32 {
    timeline
        .state
        .get_my_hint(value)
        .unwrap_or("0".to_string())
        .parse::<i32>()
        .unwrap()
}

pub fn call_venom(target: &String, v1: impl ToString, annotation: Option<&'static str>) -> String {
    format!(
        "wt {} {}: {}",
        annotation.unwrap_or("Afflicting"),
        target,
        v1.to_string()
    )
}

pub fn call_venoms(
    target: &String,
    v1: impl ToString,
    v2: impl ToString,
    annotation: Option<&'static str>,
) -> String {
    format!(
        "wt {} {}: {}, {}",
        annotation.unwrap_or("Afflicting"),
        target,
        v1.to_string(),
        v2.to_string()
    )
}

pub fn call_triple_venoms(
    target: &String,
    v1: impl ToString,
    v2: impl ToString,
    v3: impl ToString,
    annotation: Option<&'static str>,
) -> String {
    format!(
        "wt {} {}: {}, {}, {}",
        annotation.unwrap_or("Afflicting"),
        target,
        v1.to_string(),
        v2.to_string(),
        v3.to_string()
    )
}

pub fn should_call_venoms(timeline: &AetTimeline) -> bool {
    check_config(timeline, &"VENOM_CALLING".to_string())
}
