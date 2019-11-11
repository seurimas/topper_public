use crate::timeline::*;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Result};
use std::io;

#[derive(Deserialize)]
enum TopperRequest {
    TargettedQeb(String),
}

#[derive(Deserialize)]
enum TopperMessage {
    Event(TimeSlice),
    Request(TopperRequest),
}

#[derive(Serialize)]
enum TopperResponse {
    Qeb(String),
    Silent,
}

struct Topper {
    timeline: Timeline,
}

pub fn parse_time_slice(line: &String) -> Result<TimeSlice> {
    let slice: TimeSlice = from_str(line)?;
    Ok(slice)
}

impl Topper {
    pub fn new() -> Self {
        Topper {
            timeline: Timeline::new(),
        }
    }

    pub fn parse_request_or_event(&mut self, line: &String) -> Result<TopperResponse> {
        let topper_msg: TopperMessage = from_str(line)?;
        match topper_msg {
            TopperMessage::Event(timeslice) => {
                self.timeline.push_time_slice(timeslice);
                Ok(TopperResponse::Silent)
            }
            TopperMessage::Request(request) => Ok(TopperResponse::Qeb("stand".to_string())),
            _ => Ok(TopperResponse::Silent),
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
