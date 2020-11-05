use crate::classes::{get_attack, VenomPlan};
use crate::topper::battle_stats::*;
use crate::topper::db::DatabaseModule;
use crate::topper::telnet::TelnetModule;
use crate::topper::timeline::{TimeSlice, Timeline, TimelineModule};
use crate::topper::web_ui::WebModule;
use crate::types::{CType, Hypnosis};
mod battle_stats;
pub mod db;
pub mod telnet;
mod timeline;
mod web_ui;
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::io;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Deserialize)]
pub enum TopperRequest {
    Target(String),
    BattleStats(CType),
    Plan(String),
    Attack(String),
    Hint(String, String, String),
    Assume(String, String, bool),
    Reset(String),
    Api(String),
    // DB Methods
    Inspect(String, String),
    SetPriority(String, usize, Option<VenomPlan>),
    SetHypnosis(String, usize, Option<Hypnosis>),
    // Web Methods
    OpenWeb,
}

#[derive(Deserialize)]
pub enum TopperMessage {
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

pub trait TopperModule<'s> {
    type Siblings;
    fn handle_message(
        &mut self,
        message: &TopperMessage,
        siblings: Self::Siblings,
    ) -> Result<TopperResponse, String>;
}

pub struct TopperCore {
    target: Option<String>,
}

impl TopperCore {
    fn new() -> Self {
        TopperCore { target: None }
    }
}

impl<'s> TopperModule<'s> for TopperCore {
    type Siblings = ();
    fn handle_message(
        &mut self,
        message: &TopperMessage,
        siblings: Self::Siblings,
    ) -> Result<TopperResponse, String> {
        match message {
            TopperMessage::Kill => Ok(TopperResponse::die()),
            TopperMessage::Request(request) => match request {
                TopperRequest::Target(target) => {
                    self.target = Some(target.to_string());
                    Ok(TopperResponse::silent())
                }
                _ => Ok(TopperResponse::silent()),
            },
            _ => Ok(TopperResponse::silent()),
        }
    }
}

impl TopperResponse {
    pub fn then(self, next: TopperResponse) -> Self {
        TopperResponse {
            qeb: self.qeb.or(next.qeb),
            battle_stats: self.battle_stats.or(next.battle_stats),
            error: self.error.or(next.error),
            die: self.die || next.die,
        }
    }
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
    timeline_module: TimelineModule,
    core_module: TopperCore,
    telnet_module: TelnetModule,
    battlestats_module: BattleStatsModule,
    database_module: DatabaseModule,
    web_module: WebModule,
}

impl Topper {
    pub fn new(send_lines: Sender<String>) -> Self {
        Topper {
            timeline_module: TimelineModule::new(),
            core_module: TopperCore::new(),
            telnet_module: TelnetModule::new(send_lines),
            battlestats_module: BattleStatsModule::new(),
            database_module: DatabaseModule::new("topper.db"),
            web_module: WebModule::new(),
        }
    }

    pub fn me(&self) -> String {
        self.timeline_module.timeline.who_am_i()
    }

    pub fn get_target(&self) -> Option<String> {
        self.core_module.target.clone()
    }

    pub fn get_timeline(&mut self) -> &mut Timeline {
        &mut self.timeline_module.timeline
    }

    pub fn get_database(&mut self) -> &mut DatabaseModule {
        &mut self.database_module
    }

    pub fn parse_request_or_event(&mut self, line: &String) -> Result<TopperResponse, String> {
        let start = Instant::now();
        let parsed = from_str(line);
        let result = match parsed {
            Ok(topper_msg) => {
                let module_msg = self
                    .core_module
                    .handle_message(&topper_msg, ())?
                    .then(self.timeline_module.handle_message(&topper_msg, ())?)
                    .then(self.telnet_module.handle_message(&topper_msg, ())?)
                    .then(self.battlestats_module.handle_message(
                        &topper_msg,
                        (
                            &self.timeline_module.timeline,
                            &self.core_module.target,
                            &self.database_module,
                        ),
                    )?)
                    .then(self.database_module.handle_message(&topper_msg, ())?)
                    .then(self.web_module.handle_message(&topper_msg, ())?);
                match topper_msg {
                    TopperMessage::Request(request) => match request {
                        TopperRequest::Attack(strategy) => {
                            if let Some(target) = self.get_target() {
                                Ok(module_msg.then(TopperResponse::qeb(get_attack(
                                    &self.timeline_module.timeline,
                                    &self.me(),
                                    &target,
                                    &strategy,
                                    Some(&self.database_module),
                                ))))
                            } else {
                                Ok(module_msg.then(TopperResponse::error("No target.".into())))
                            }
                        }
                        _ => Ok(module_msg),
                    },
                    _ => Ok(module_msg),
                }
            }
            Err(error) => Err(error.to_string()),
        };
        let millis = start.elapsed().as_millis();
        info!("({}) {}", millis, line);
        if millis > 50 {
            println!("{} millis to process...", millis);
        }
        result
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
