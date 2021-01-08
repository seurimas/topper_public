use crate::aetolia::classes::{get_attack, VenomPlan};
use crate::aetolia::timeline::AetTimeSlice;
use crate::aetolia::topper::battle_stats::*;
use crate::aetolia::types::{AgentState, Hypnosis};
use crate::timeline::{BaseAgentState, CType, Timeline};
pub use crate::topper::db::DatabaseModule;
pub use crate::topper::telnet::TelnetModule;
pub use crate::topper::timeline::TimelineModule;
pub use crate::topper::web_ui::WebModule;
pub mod db;
pub mod telnet;
pub mod timeline;
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
    AetEvent(AetTimeSlice),
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
    pub target: Option<String>,
}

impl TopperCore {
    pub fn new() -> Self {
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

pub struct Topper<O, P, A> {
    pub timeline_module: TimelineModule<O, P, A>,
    pub core_module: TopperCore,
    pub telnet_module: TelnetModule,
    pub battlestats_module: BattleStatsModule,
    pub database_module: DatabaseModule,
    pub web_module: WebModule,
}

pub trait TopperHandler {
    type Message;
    fn handle_request_or_event(
        &mut self,
        topper_msg: &Self::Message,
    ) -> Result<TopperResponse, String>;
    fn from_str(&self, line: &String) -> Result<Self::Message, String>;
}

impl<O, P, A: BaseAgentState + Clone> Topper<O, P, A>
where
    Topper<O, P, A>: TopperHandler,
{
    pub fn me(&self) -> String {
        self.timeline_module.timeline.who_am_i()
    }

    pub fn get_target(&self) -> Option<String> {
        self.core_module.target.clone()
    }

    pub fn get_timeline(&mut self) -> &mut Timeline<O, P, A> {
        &mut self.timeline_module.timeline
    }

    pub fn get_database(&mut self) -> &mut DatabaseModule {
        &mut self.database_module
    }

    pub fn parse_request_or_event(&mut self, line: &String) -> Result<TopperResponse, String> {
        let start = Instant::now();
        let parsed = self.from_str(line);
        let result = match parsed {
            Ok(topper_msg) => self.handle_request_or_event(&topper_msg),
            Err(error) => Err(error.to_string()),
        };
        let millis = start.elapsed().as_millis();
        info!("({}) {}", millis, line);
        if millis > 50 {
            println!("{} millis to process...", millis);
        }
        result
    }

    pub fn provide_action(&mut self) {
        loop {
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(n) => {
                    if n == 0 {
                        break;
                    }
                    let without_newline = &input[..input.len() - 1];
                    let response = &self
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
}

pub fn send_response(response: &TopperResponse) {
    println!(
        "{}",
        serde_json::to_string(response).unwrap_or("{err: \"JSON Error\"}".into())
    );
}
