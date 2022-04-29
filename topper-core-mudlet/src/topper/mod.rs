pub use crate::topper::telnet::TelnetModule;
pub use crate::topper::timeline::TimelineModule;
use topper_core::timeline::db::DatabaseModule;
use topper_core::timeline::{BaseAgentState, CType, Timeline};
pub mod telnet;
pub mod timeline;
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::collections::HashMap;
use std::io;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::{Duration, Instant};
use topper_core::observations::ObservationParser;

#[derive(Deserialize, Debug)]
pub enum TopperRequest {
    Target(String),
    BattleStats(CType),
    Plan(String),
    Attack(String),
    Hint(String, String, String),
    Assume(String, String, bool),
    Reset(String),
    Api(String),
    ModuleMsg(String, String),
}

#[derive(Deserialize, Debug)]
pub enum TopperMessage<TS> {
    Kill,
    TimeSlice(TS),
    Request(TopperRequest),
    Target(String),
}

#[derive(Serialize)]
pub struct TopperResponse<BS> {
    pub qeb: Option<String>,
    pub battle_stats: Option<BS>,
    pub error: Option<String>,
    pub passive: HashMap<String, String>,
    pub die: bool,
}

pub trait TopperModule<'s, TS, BS> {
    type Siblings;
    fn handle_message(
        &mut self,
        message: &TopperMessage<TS>,
        siblings: Self::Siblings,
    ) -> Result<TopperResponse<BS>, String>;
}

pub struct TopperCore {
    pub target: Option<String>,
}

impl TopperCore {
    pub fn new() -> Self {
        TopperCore { target: None }
    }
}

impl<'s, TS, BS> TopperModule<'s, TS, BS> for TopperCore {
    type Siblings = ();
    fn handle_message(
        &mut self,
        message: &TopperMessage<TS>,
        siblings: Self::Siblings,
    ) -> Result<TopperResponse<BS>, String> {
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

impl<'s, S, TS, BS> TopperModule<'s, TS, BS> for Option<S>
where
    S: TopperModule<'s, TS, BS>,
{
    type Siblings = S::Siblings;
    fn handle_message(
        &mut self,
        message: &TopperMessage<TS>,
        siblings: Self::Siblings,
    ) -> Result<TopperResponse<BS>, String> {
        if let Some(module) = self {
            module.handle_message(message, siblings)
        } else {
            Ok(TopperResponse::silent())
        }
    }
}

impl<BS> TopperResponse<BS> {
    pub fn then(self, next: TopperResponse<BS>) -> Self {
        let mut passive = self.passive;
        passive.extend(next.passive);
        TopperResponse {
            qeb: self.qeb.or(next.qeb),
            battle_stats: self.battle_stats.or(next.battle_stats),
            error: self.error.or(next.error),
            passive,
            die: self.die || next.die,
        }
    }
    pub fn battle_stats(battle_stats: BS) -> Self {
        TopperResponse {
            qeb: None,
            battle_stats: Some(battle_stats),
            error: None,
            passive: HashMap::new(),
            die: false,
        }
    }
    pub fn silent() -> Self {
        TopperResponse {
            qeb: None,
            battle_stats: None,
            error: None,
            passive: HashMap::new(),
            die: false,
        }
    }
    pub fn error(message: String) -> TopperResponse<BS> {
        TopperResponse {
            qeb: None,
            battle_stats: None,
            error: Some(message),
            passive: HashMap::new(),
            die: false,
        }
    }
    pub fn qeb(action: String) -> TopperResponse<BS> {
        TopperResponse {
            qeb: Some(action),
            battle_stats: None,
            error: None,
            passive: HashMap::new(),
            die: false,
        }
    }
    pub fn passive(name: String, value: String) -> TopperResponse<BS> {
        let mut passive = HashMap::new();
        passive.insert(name, value);
        TopperResponse {
            qeb: None,
            battle_stats: None,
            error: None,
            passive,
            die: false,
        }
    }
    pub fn die() -> TopperResponse<BS> {
        TopperResponse {
            qeb: None,
            battle_stats: None,
            error: None,
            passive: HashMap::new(),
            die: true,
        }
    }
}

pub trait Topper<O, P, A: BaseAgentState + Clone, DB: DatabaseModule> {
    fn get_timeline_module(&self) -> &TimelineModule<O, P, A>;
    fn get_core_module(&self) -> &TopperCore;
    fn get_database(&mut self) -> &mut DB;
    fn get_mut_timeline_module(&mut self) -> &mut TimelineModule<O, P, A>;

    fn me(&self) -> String {
        self.get_timeline_module().timeline.who_am_i()
    }

    fn get_target(&self) -> Option<String> {
        self.get_core_module().target.clone()
    }

    fn get_timeline(&mut self) -> &mut Timeline<O, P, A> {
        &mut self.get_mut_timeline_module().timeline
    }

    fn provide_action<BS: Serialize>(&mut self)
    where
        Self: TopperHandler<BS>,
    {
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
                    serde_json::to_string(&TopperResponse::<BS>::error(error.to_string())).unwrap()
                ),
            }
            thread::yield_now();
        }
    }
}

pub trait TopperHandler<BS> {
    type Message;
    fn handle_request_or_event(
        &mut self,
        topper_msg: &mut Self::Message,
    ) -> Result<TopperResponse<BS>, String>;
    fn from_str(&self, line: &String) -> Result<Self::Message, String>;

    fn parse_request_or_event(&mut self, line: &String) -> Result<TopperResponse<BS>, String> {
        let start = Instant::now();
        let parsed = self.from_str(line);
        let result = match parsed {
            Ok(mut topper_msg) => self.handle_request_or_event(&mut topper_msg),
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

pub fn send_response<BS: Serialize>(response: &TopperResponse<BS>) {
    println!(
        "{}",
        serde_json::to_string(response).unwrap_or("{err: \"JSON Error\"}".into())
    );
}
