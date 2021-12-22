use regex::Regex;
use reqwest::Response;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use sled::{open, Db, IVec};
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, SystemTime};
use tokio;
use topper_aetolia::classes::{Class, VenomPlan};
use topper_aetolia::curatives::first_aid::{parse_priority_set, FirstAidPriorities};
use topper_aetolia::db::AetDatabaseModule;
use topper_aetolia::timeline::*;
use topper_aetolia::types::{FType, Hypnosis};
use topper_core::timeline::db::DatabaseModule;
use topper_core_mudlet::topper::{
    TopperCore, TopperMessage, TopperModule, TopperRequest, TopperResponse,
};

use super::battle_stats::BattleStats;

pub struct AetMudletDatabaseModule {
    db: Db,
    thread: JoinHandle<()>,
    api_sender: Sender<ApiRequest>,
    api_receiver: Receiver<CharacterApiResponse>,
}

struct ApiRequest(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterApiResponse {
    pub name: String,
    fullname: String,
    level: String,
    class: String,
    pub city: String,
    guild: String,
    race: String,
}

fn get_epoch_time() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Bad epoch")
        .as_millis()
}

fn from_epoch_time(time: u128) -> SystemTime {
    SystemTime::UNIX_EPOCH
        .checked_add(Duration::from_millis(time as u64))
        .expect("Bad add")
}

fn time_since_epoch(time: u128) -> Duration {
    SystemTime::now()
        .duration_since(from_epoch_time(time))
        .expect("Bad duration")
}

#[tokio::main]
pub async fn retrieve_api(who: String) -> reqwest::Result<CharacterApiResponse> {
    let api_url = format!("https://api.aetolia.com/characters/{}.json", who);
    println!("Retrieving {}", api_url);
    reqwest::get(&api_url)
        .await?
        .json::<CharacterApiResponse>()
        .await
}

impl AetMudletDatabaseModule {
    pub fn new<P: AsRef<Path>>(path: P) -> AetMudletDatabaseModule {
        let (send_request, receive_request): (Sender<ApiRequest>, Receiver<ApiRequest>) = channel();
        let (send_response, receive_response): (
            Sender<CharacterApiResponse>,
            Receiver<CharacterApiResponse>,
        ) = channel();
        AetMudletDatabaseModule {
            db: open(path).unwrap(),
            api_sender: send_request,
            api_receiver: receive_response,
            thread: thread::spawn(move || loop {
                while let Ok(ApiRequest(who)) = receive_request.try_recv() {
                    if let Ok(api_response) = retrieve_api(who) {
                        send_response.send(api_response);
                    }
                }
                thread::sleep_ms(100);
            }),
        }
    }

    fn insert(&self, tree: &str, key: &String, value: IVec) {
        self.db
            .open_tree(tree)
            .expect(format!("Bad {} tree", tree).as_ref())
            .insert(key, value)
            .expect(format!("Bad {} insert", key).as_ref());
    }

    fn get(&self, tree: &str, key: &String) -> Option<IVec> {
        self.db
            .open_tree(tree)
            .expect(format!("Bad {} tree", tree).as_ref())
            .get(key)
            .expect(format!("Bad {} get", key).as_ref())
    }
}

impl DatabaseModule for AetMudletDatabaseModule {
    fn insert_json<T: Serialize>(&self, tree: &str, key: &String, value: T) {
        let json = serde_json::to_string(&value).unwrap();
        self.insert(tree, key, json.as_bytes().into())
    }

    fn get_json<T: DeserializeOwned>(&self, tree: &str, key: &String) -> Option<T> {
        self.get(tree, key)
            .and_then(|ivec| {
                let slice: Vec<u8> = ivec.as_ref().into();
                String::from_utf8(slice).ok()
            })
            .and_then(|str_val| {
                let str_val: &str = str_val.as_ref();
                serde_json::from_str(str_val).ok()
            })
    }
}

impl AetMudletDatabaseModule {
    fn send_api_request(&self, who: &String, priority: u64) -> bool {
        let last_bytes = self.get("api_last", who).unwrap_or((&[0; 16]).into());
        let last = u128::from_be_bytes(last_bytes.as_ref().try_into().expect("Bad length"));
        let since = time_since_epoch(last);
        if since.as_secs() > priority {
            let epoch = get_epoch_time();
            self.insert("api_last", who, (&epoch.to_be_bytes()).into());
            self.api_sender.send(ApiRequest(who.to_string())).is_ok()
        } else {
            false
        }
    }

    fn drain_responses(&self) {
        while let Ok(character) = self.api_receiver.try_recv() {
            println!(
                "Received response for {} ({} from {})",
                character.name, character.class, character.city
            );
            if let Some(class) = Class::from_str(&character.class) {
                self.set_class(&character.name, class);
            }
            self.insert_json("character", &character.name.clone(), character);
        }
    }
}

impl AetMudletDatabaseModule {
    fn set_venom_plan(&self, stack_name: &String, stack: Vec<VenomPlan>) {
        self.insert_json::<Vec<VenomPlan>>("stacks", stack_name, stack);
    }

    fn set_hypno_plan(&self, stack_name: &String, stack: Vec<Hypnosis>) {
        self.insert_json::<Vec<Hypnosis>>("hypnosis", stack_name, stack);
    }

    fn set_first_aid_priorities(
        &self,
        who: &String,
        priorities_name: &String,
        priorities: FirstAidPriorities,
    ) {
        self.insert_json::<FirstAidPriorities>(
            "first_aid",
            &format!("{}_{}", who, priorities_name),
            priorities,
        );
    }

    pub fn get_characters(&self) -> Vec<CharacterApiResponse> {
        self.db
            .open_tree("character")
            .expect(format!("Bad character tree").as_ref())
            .iter()
            .filter_map(|result| result.ok())
            .filter_map(|(_key, ivec)| {
                let slice: Vec<u8> = ivec.as_ref().into();
                String::from_utf8(slice).ok().and_then(|str_val| {
                    let str_val: &str = str_val.as_ref();
                    serde_json::from_str(str_val).ok()
                })
            })
            .collect::<Vec<CharacterApiResponse>>()
    }

    pub fn get_character(&self, who: &String) -> Option<(Option<Class>, String)> {
        if let Some(character) = self.get_json::<CharacterApiResponse>("character", who) {
            Some((Class::from_str(&character.class), character.city.clone()))
        } else {
            None
        }
    }
}
impl AetDatabaseModule for AetMudletDatabaseModule {
    fn get_class(&self, who: &String) -> Option<Class> {
        let class_id = self.get("classes", who);
        if let Some(class_id) = class_id {
            if let [class_id] = class_id.as_ref() {
                Class::try_from(*class_id).ok()
            } else {
                None
            }
        } else {
            None
        }
    }

    fn set_class(&self, who: &String, class: Class) {
        self.insert("classes", who, (&[class as u8]).into());
    }

    fn get_venom_plan(&self, stack_name: &String) -> Option<Vec<VenomPlan>> {
        self.get_json::<Vec<VenomPlan>>("stacks", stack_name)
    }

    fn get_hypno_plan(&self, stack_name: &String) -> Option<Vec<Hypnosis>> {
        self.get_json::<Vec<Hypnosis>>("hypnosis", stack_name)
    }

    fn get_first_aid_priorities(
        &self,
        who: &String,
        priorities_name: &String,
    ) -> Option<FirstAidPriorities> {
        self.get_json::<FirstAidPriorities>("first_aid", &format!("{}_{}", who, priorities_name))
    }
}

fn insert_in_stack<T: Serialize + Clone>(
    old_stack: &mut Vec<T>,
    index: usize,
    item: &T,
    unique: bool,
) {
    if let Ok(item_str) = serde_json::to_string(&item) {
        let mut old_index = None;
        for idx in 0..old_stack.len() {
            let plan_item = old_stack.get(idx).unwrap();
            if let Ok(plan_item_str) = serde_json::to_string(plan_item) {
                if item_str.eq(&plan_item_str) && unique {
                    old_index = Some(idx);
                    break;
                }
            } else {
            }
        }
        if let Some(old_index) = old_index {
            old_stack.remove(old_index);
            if index >= old_stack.len() {
                old_stack.push(item.clone());
            } else if index > old_index {
                old_stack.insert(index - 1, item.clone());
            } else {
                old_stack.insert(index, item.clone());
            }
        } else if index <= old_stack.len() {
            old_stack.insert(index, item.clone());
        } else {
            old_stack.push(item.clone());
        }
    }
}

fn update_stack<T: Serialize + Clone>(
    stack_name: &String,
    old_stack: &mut Vec<T>,
    index: usize,
    item: &Option<T>,
    unique: bool,
) {
    if let Some(item) = item {
        insert_in_stack(old_stack, index, item, unique);
    } else {
        if index < old_stack.len() {
            old_stack.remove(index);
        } else {
            println!(
                "Stack {} has only {} items, but tried to remove {}",
                stack_name,
                old_stack.len(),
                index
            );
        }
    }
}

lazy_static! {
    static ref INSPECT: Regex = Regex::new(r"^inspect (\w+) (\w+)$").unwrap();
    static ref SET_HYPNOSIS: Regex = Regex::new(r"^hypnosis (\w+) (\d+) (.*)$").unwrap();
    static ref SET_PRIORITY: Regex = Regex::new(r"^priority (\w+) (\d+) (.*)$").unwrap();
}

fn parse_inspect(message: &String) -> Option<(String, String)> {
    if let Some(captures) = INSPECT.captures(message) {
        if let (Some(tree), Some(key)) = (captures.get(1), captures.get(2)) {
            Some((tree.as_str().to_string(), key.as_str().to_string()))
        } else {
            None
        }
    } else {
        None
    }
}

fn parse_set_hypnosis(message: &String) -> Option<(String, usize, Option<Hypnosis>)> {
    if let Some(captures) = SET_HYPNOSIS.captures(message) {
        if let (Some(stack), Some(index), Some(plan_str)) =
            (captures.get(1), captures.get(2), captures.get(3))
        {
            let plan: Option<Hypnosis> = from_str(plan_str.as_str()).unwrap_or(None);
            Some((
                stack.as_str().to_string(),
                index.as_str().parse::<usize>().unwrap(),
                plan,
            ))
        } else {
            None
        }
    } else {
        None
    }
}

fn parse_set_priority(message: &String) -> Option<(String, usize, Option<VenomPlan>)> {
    if let Some(captures) = SET_PRIORITY.captures(message) {
        if let (Some(stack), Some(index), Some(plan_str)) =
            (captures.get(1), captures.get(2), captures.get(3))
        {
            let plan: Option<VenomPlan> = from_str(plan_str.as_str()).unwrap_or(None);
            Some((
                stack.as_str().to_string(),
                index.as_str().parse::<usize>().unwrap(),
                plan,
            ))
        } else {
            None
        }
    } else {
        None
    }
}

impl<'s> TopperModule<'s, AetTimeSlice, BattleStats> for AetMudletDatabaseModule {
    type Siblings = (String);
    fn handle_message(
        &mut self,
        message: &TopperMessage<AetTimeSlice>,
        siblings: Self::Siblings,
    ) -> Result<TopperResponse<BattleStats>, String> {
        self.drain_responses();
        match message {
            TopperMessage::Request(TopperRequest::Api(who)) => {
                self.send_api_request(who, 300);
                Ok(TopperResponse::silent())
            }
            TopperMessage::Request(TopperRequest::ModuleMsg(module, message)) => {
                match module.as_ref() {
                    "db" => {
                        if let Some((stack, index, hypno)) = parse_set_hypnosis(message) {
                            let mut old_stack = self.get_hypno_plan(&stack).unwrap_or_default();
                            update_stack(&stack, &mut old_stack, index, &hypno, false);
                            if let Ok(stack_str) = to_string_pretty(&old_stack) {
                                println!("{}", stack_str);
                            }
                            self.set_hypno_plan(&stack, old_stack);
                            Ok(TopperResponse::silent())
                        } else if let Some((stack, index, venom_plan)) = parse_set_priority(message)
                        {
                            let mut old_stack = self.get_venom_plan(&stack).unwrap_or_default();
                            update_stack(&stack, &mut old_stack, index, &venom_plan, true);
                            if let Ok(stack_str) = to_string_pretty(&old_stack) {
                                println!("{}", stack_str);
                            }
                            self.set_venom_plan(&stack, old_stack);
                            Ok(TopperResponse::silent())
                        } else if let Some((tree, key)) = parse_inspect(message) {
                            match tree.as_ref() {
                                "stacks" => {
                                    if let Some(plan) = self.get_venom_plan(&key) {
                                        if let Ok(plan_str) = to_string_pretty(&plan) {
                                            println!("{}", plan_str);
                                        }
                                    } else {
                                        println!("Strategy {} not found", key);
                                    }
                                }
                                "hypnosis" => {
                                    if let Some(plan) = self.get_hypno_plan(&key) {
                                        if let Ok(plan_str) = to_string_pretty(&plan) {
                                            println!("{}", plan_str);
                                        }
                                    } else {
                                        println!("Hypno strategy {} not found", key);
                                    }
                                }
                                "character" => {
                                    if let Some(character) =
                                        self.get_json::<CharacterApiResponse>("character", &key)
                                    {
                                        if let Ok(character_str) = to_string_pretty(&character) {
                                            println!("{}", character_str);
                                        }
                                    } else {
                                        println!("Character {} not found", key);
                                    }
                                }
                                _ => {}
                            }
                            Ok(TopperResponse::silent())
                        } else {
                            Ok(TopperResponse::silent())
                        }
                    }
                    _ => Ok(TopperResponse::silent()),
                }
            }
            TopperMessage::TimeSlice(event) => {
                for observation in event.observations.iter().flatten() {
                    match observation {
                        topper_aetolia::timeline::AetObservation::CombatAction(
                            topper_aetolia::timeline::CombatAction { caster, target, .. },
                        ) => {
                            // if !caster.eq("") && !caster.find(" ").is_some() {
                            //     self.send_api_request(caster, 3600 * 2);
                            // }
                            // if !target.eq("") && !target.find(" ").is_some() {
                            //     self.send_api_request(target, 3600 * 2);
                            // }
                        }
                        _ => {}
                    }
                }
                if let Some((name, priorities)) = parse_priority_set(&event.lines) {
                    self.set_first_aid_priorities(&siblings, &name, priorities);
                }
                Ok(TopperResponse::silent())
            }
            _ => Ok(TopperResponse::silent()),
        }
    }
}
