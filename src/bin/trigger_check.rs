#[macro_use]
extern crate lazy_static;
extern crate topper;
use regex::Regex;
use serde_json::from_str;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use topper::aetolia::timeline::*;
use topper::topper::observations::ObservationParser;
use topper::topper::*;

lazy_static! {
    static ref LOG_LINE: Regex = Regex::new(r"\d\d:\d\d:\d\d \[.{5}\] \(\d+\) (.*)").unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(args.get(1).unwrap()).unwrap();
    let reader = BufReader::new(file);
    let observer =
        ObservationParser::<AetObservation>::new_from_directory("triggers".to_string()).unwrap();

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let capture = LOG_LINE.captures(&line).unwrap().get(1).unwrap().as_str();
        let r_slice: Result<TopperMessage, serde_json::Error> = from_str(capture);
        match r_slice {
            Ok(TopperMessage::AetEvent(slice)) => {
                let new = observer.observe(&slice);
                if let Some(prev) = slice.observations {
                    let prev_without_sent: Vec<&AetObservation> = prev
                        .iter()
                        .filter(|obs| match obs {
                            AetObservation::Sent(_) => false,
                            _ => true,
                        })
                        .collect();
                    if prev_without_sent.len() != new.len() {
                        println!("{}: {:?} {:?}", index, prev_without_sent, new);
                    } else if !prev_without_sent.iter().zip(&new).all(|(a, b)| *a == b) {
                        println!("{}: {:?} {:?}", index, prev_without_sent, new);
                    }
                }
            }
            _ => {}
        }
    }
}
