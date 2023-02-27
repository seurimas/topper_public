use regex::Regex;
use std::collections::{HashMap, HashSet};
use topper_aetolia::timeline::{
    for_agent, AetObservation, AetTimeSlice, AetTimeline, CombatAction,
};
use topper_core::observations::strip_ansi;
use topper_core::timeline::db::DatabaseModule;
use topper_core::timeline::CType;
use topper_core_mudlet::topper::{TopperMessage, TopperModule, TopperRequest, TopperResponse};

use super::battle_stats::BattleStats;
use super::db::AetMudletDatabaseModule;

lazy_static! {
    static ref EVAL: Regex = Regex::new(r#"^"(\w+)"\s+(.*)\. \((.*)\)$"#).unwrap();
    static ref START: Regex = Regex::new(r"^start (\w+)$").unwrap();
}

fn get_eval(line: String) -> Option<(String, String, String)> {
    if let Some(captures) = EVAL.captures(&line) {
        let id = captures.get(1).unwrap().as_str().to_string();
        let full_name = captures.get(2).unwrap().as_str().to_string();
        let status = captures.get(3).unwrap().as_str().to_string();
        Some((id, full_name, status))
    } else {
        None
    }
}

#[derive(Default, Debug)]
pub struct BasherModule {
    active_area: Option<String>,
    my_aggros: HashSet<String>,
}

impl BasherModule {
    pub fn new() -> Self {
        Self {
            active_area: None,
            my_aggros: HashSet::new(),
        }
    }
}

impl<'s> TopperModule<'s, AetTimeSlice, BattleStats> for BasherModule {
    type Siblings = (&'s String, &'s mut AetTimeline, &'s AetMudletDatabaseModule);
    fn handle_message(
        &mut self,
        message: &TopperMessage<AetTimeSlice>,
        (me, mut timeline, db): Self::Siblings,
    ) -> Result<TopperResponse<BattleStats>, String> {
        let mut calls = None;
        match message {
            TopperMessage::TimeSlice(timeslice) => {
                if let Some(observations) = &timeslice.observations {
                    for event in observations.iter() {
                        match event {
                            AetObservation::CombatAction(CombatAction {
                                caster, target, ..
                            }) => {}
                            _ => {}
                        }
                    }
                }
                for line in timeslice.lines.iter() {
                    let line = strip_ansi(&line.0);
                    if let Some(captures) = EVAL.captures(&line) {}
                }
            }
            TopperMessage::Request(TopperRequest::ModuleMsg(module, command)) => {
                if module.eq("basher") {
                    match command.as_ref() {
                        "check" => {
                            println!("Basher: {:?}", self);
                        }
                        "end" => {
                            println!("Ending basher.");
                        }
                        _ => {
                            if let Some(captures) = START.captures(command) {
                                let area = captures.get(1).unwrap().as_str().to_string();
                                println!("Beginning area {}", area);
                            } else {
                                println!("No such command: {}", command);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        if let Some(calls) = calls {
            Ok(TopperResponse::passive("basher".to_string(), calls))
        } else {
            Ok(TopperResponse::silent())
        }
    }
}

#[cfg(test)]
mod basher_tests {
    use crate::topper::basher::*;
    #[test]
    fn eval_works() {
        let tlingor = r#""tlingor165930"     a baby tlingor. (uninjured)"#;
        assert_eq!(
            get_eval(tlingor.to_string()),
            Some((
                "tlingor165930".to_string(),
                "a baby tlingor".to_string(),
                "uninjured".to_string()
            ))
        );
    }
}
