use crate::timeline::types::AetTimeline;

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

pub fn call_venom(target: &String, v1: &String) -> String {
    format!("wt Afflicting {}: {}", target, v1)
}

pub fn call_venoms(target: &String, v1: &String, v2: &String) -> String {
    format!("wt Afflicting {}: {}, {}", target, v1, v2)
}

pub fn call_triple_venoms(target: &String, v1: &String, v2: &String, v3: &String) -> String {
    format!("wt Afflicting {}: {}, {}, {}", target, v1, v2, v3)
}

pub fn should_call_venoms(timeline: &AetTimeline) -> bool {
    check_config(timeline, &"VENOM_CALLING".to_string())
}
