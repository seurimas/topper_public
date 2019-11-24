use crate::battle_stats::*;
use crate::timeline::*;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Result};
use std::io;

#[derive(Deserialize)]
enum TopperRequest {
    Target(String),
    BattleStats,
}

#[derive(Deserialize)]
enum TopperMessage {
    Event(TimeSlice),
    Request(TopperRequest),
}

#[derive(Serialize)]
pub struct TopperResponse {
    pub qeb: Option<String>,
    pub battleStats: Option<BattleStats>,
}

impl TopperResponse {
    pub fn battleStats(battleStats: BattleStats) -> Self {
        TopperResponse {
            qeb: None,
            battleStats: Some(battleStats),
        }
    }
    pub fn silent() -> Self {
        TopperResponse {
            qeb: None,
            battleStats: None,
        }
    }
}

pub struct Topper {
    pub timeline: Timeline,
    pub me: String,
    pub target: Option<String>,
}

pub fn parse_time_slice(line: &String) -> Result<TimeSlice> {
    let slice: TimeSlice = from_str(line)?;
    Ok(slice)
}

impl Topper {
    pub fn new() -> Self {
        Topper {
            timeline: Timeline::new(),
            me: "Seurimas".into(),
            target: None,
        }
    }

    pub fn parse_request_or_event(&mut self, line: &String) -> Result<TopperResponse> {
        let topper_msg: TopperMessage = from_str(line)?;
        match topper_msg {
            TopperMessage::Event(timeslice) => {
                self.timeline.push_time_slice(timeslice);
                Ok(TopperResponse::battleStats(get_battle_stats(self)))
            }
            TopperMessage::Request(request) => match request {
                TopperRequest::Target(target) => {
                    self.target = Some(target);
                    Ok(TopperResponse::battleStats(get_battle_stats(self)))
                }
                _ => Ok(TopperResponse::silent()),
            },
            _ => Ok(TopperResponse::silent()),
        }
    }
}

pub fn provide_action() {
    let mut topper = Topper::new();
    while true {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                let without_newline = &input[..input.len() - 1];
                println!(
                    "{}",
                    serde_json::to_string(
                        &topper
                            .parse_request_or_event(&without_newline.to_string())
                            .unwrap()
                    )
                    .unwrap()
                );
            }
            Err(error) => println!("error: {}", error),
        }
    }
}

pub fn echo_time_slice() {
    let mut iterations = 100;
    while iterations > 0 {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if input == "" {
                    break;
                }
                let without_newline = &input[..input.len() - 1];
                println!(
                    "{} {:?}",
                    without_newline,
                    parse_time_slice(&without_newline.to_string())
                );
            }
            Err(error) => println!("error: {}", error),
        }
        iterations -= 1;
    }
}
