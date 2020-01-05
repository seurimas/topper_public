use crate::battle_stats::*;
use crate::classes::get_attack;
use crate::timeline::*;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Result as JsonResult};
use std::collections::HashMap;
use std::io;
use std::panic;

#[derive(Deserialize)]
enum TopperRequest {
    Target(String),
    BattleStats,
    Attack(String),
    Hint(String, String, String),
    Reset,
}

#[derive(Deserialize)]
enum TopperMessage {
    Event(TimeSlice),
    Request(TopperRequest),
    Target(String),
}

#[derive(Serialize)]
pub struct TopperResponse {
    pub qeb: Option<String>,
    pub battleStats: Option<BattleStats>,
    pub error: Option<String>,
}

impl TopperResponse {
    pub fn battleStats(battleStats: BattleStats) -> Self {
        TopperResponse {
            qeb: None,
            battleStats: Some(battleStats),
            error: None,
        }
    }
    pub fn silent() -> Self {
        TopperResponse {
            qeb: None,
            battleStats: None,
            error: None,
        }
    }
    pub fn error(message: String) -> TopperResponse {
        TopperResponse {
            qeb: None,
            battleStats: None,
            error: Some(message),
        }
    }
    pub fn qeb(action: String) -> TopperResponse {
        TopperResponse {
            qeb: Some(action),
            battleStats: None,
            error: None,
        }
    }
}

pub struct Topper {
    pub timeline: Timeline,
    pub me: String,
    pub target: Option<String>,
}

pub fn parse_time_slice(line: &String) -> JsonResult<TimeSlice> {
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

    pub fn parse_request_or_event(&mut self, line: &String) -> Result<TopperResponse, String> {
        let parsed = from_str(line);
        match parsed {
            Ok(topper_msg) => match topper_msg {
                TopperMessage::Event(timeslice) => {
                    self.timeline.push_time_slice(timeslice)?;
                    Ok(TopperResponse::battleStats(get_battle_stats(self)))
                }
                TopperMessage::Request(request) => match request {
                    TopperRequest::Target(target) => {
                        self.target = Some(target);
                        Ok(TopperResponse::battleStats(get_battle_stats(self)))
                    }
                    TopperRequest::Hint(who, hint, value) => {
                        self.timeline.state.add_player_hint(&who, &hint, value);
                        Ok(TopperResponse::silent())
                    }
                    TopperRequest::Reset => {
                        self.timeline.reset();
                        Ok(TopperResponse::silent())
                    }
                    TopperRequest::Attack(strategy) => {
                        if let Some(target) = &self.target {
                            Ok(TopperResponse::qeb(get_attack(self, target, &strategy)))
                        } else {
                            Ok(TopperResponse::error("No target.".into()))
                        }
                    }
                    _ => Ok(TopperResponse::silent()),
                },
                _ => Ok(TopperResponse::silent()),
            },
            Err(error) => Err(error.to_string()),
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
                let response = &topper
                    .parse_request_or_event(&without_newline.to_string())
                    .unwrap_or_else(|err| TopperResponse::error(err.to_string()));
                println!(
                    "{}",
                    serde_json::to_string(response).unwrap_or("{err: \"JSON Error\"}".into())
                );
            }
            Err(error) => println!(
                "{}",
                serde_json::to_string(&TopperResponse::error(error.to_string())).unwrap()
            ),
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
