use crate::battle_stats::*;
use crate::classes::get_attack;
use crate::timeline::*;
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::io;
use std::sync::mpsc::Sender;
use std::thread;

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
    Kill,
    Event(TimeSlice),
    Request(TopperRequest),
    Target(String),
}

#[derive(Serialize)]
pub struct TopperResponse {
    pub qeb: Option<String>,
    pub battle_stats: Option<BattleStats>,
    pub error: Option<String>,
    pub die: bool,
}

impl TopperResponse {
    pub fn battle_stats(battle_stats: BattleStats) -> Self {
        TopperResponse {
            qeb: None,
            battle_stats: Some(battle_stats),
            error: None,
            die: false,
        }
    }
    pub fn silent() -> Self {
        TopperResponse {
            qeb: None,
            battle_stats: None,
            error: None,
            die: false,
        }
    }
    pub fn error(message: String) -> TopperResponse {
        TopperResponse {
            qeb: None,
            battle_stats: None,
            error: Some(message),
            die: false,
        }
    }
    pub fn qeb(action: String) -> TopperResponse {
        TopperResponse {
            qeb: Some(action),
            battle_stats: None,
            error: None,
            die: false,
        }
    }
    pub fn die() -> TopperResponse {
        TopperResponse {
            qeb: None,
            battle_stats: None,
            error: None,
            die: true,
        }
    }
}

pub struct Topper {
    pub timeline: Timeline,
    pub target: Option<String>,
    send_lines: Sender<String>,
}

impl Topper {
    pub fn new(send_lines: Sender<String>) -> Self {
        Topper {
            timeline: Timeline::new(),
            target: None,
            send_lines,
        }
    }

    pub fn parse_request_or_event(&mut self, line: &String) -> Result<TopperResponse, String> {
        info!("{}", line);
        let parsed = from_str(line);
        match parsed {
            Ok(topper_msg) => match topper_msg {
                TopperMessage::Kill => Ok(TopperResponse::die()),
                TopperMessage::Event(timeslice) => {
                    for (line, _line_number) in timeslice.lines.iter() {
                        match self.send_lines.send(line.to_string()) {
                            Ok(()) => {}
                            Err(err) => {
                                println!("Line: {:?}", err);
                            }
                        };
                    }
                    self.timeline.push_time_slice(timeslice)?;
                    Ok(TopperResponse::battle_stats(get_battle_stats(self)))
                }
                TopperMessage::Request(request) => match request {
                    TopperRequest::Target(target) => {
                        self.target = Some(target);
                        Ok(TopperResponse::battle_stats(get_battle_stats(self)))
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

pub fn send_response(response: &TopperResponse) {
    println!(
        "{}",
        serde_json::to_string(response).unwrap_or("{err: \"JSON Error\"}".into())
    );
}

pub fn provide_action(send_lines: Sender<String>) {
    let mut topper = Topper::new(send_lines);
    loop {
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
                send_response(&response);
                if response.die {
                    break;
                }
            }
            Err(error) => println!(
                "{}",
                serde_json::to_string(&TopperResponse::error(error.to_string())).unwrap()
            ),
        }
        thread::yield_now();
    }
}
