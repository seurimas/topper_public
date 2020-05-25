use crate::classes::Class;
use sled::{open, Db, IVec};
use std::path::Path;
use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use reqwest::Response;
use std::convert::{TryInto, TryFrom};
use std::thread::JoinHandle;
use std::thread;
use std::sync::mpsc::{Receiver, Sender, channel};
use crate::topper::{TopperRequest, TopperMessage, TopperModule, TopperResponse};
use tokio;
use std::time::{Duration, SystemTime};

pub struct DatabaseModule {
    db: Db,
    thread: JoinHandle<()>,
    api_sender: Sender<ApiRequest>,
    api_receiver: Receiver<CharacterApiResponse>,
}

struct ApiRequest(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterApiResponse {
    name: String,
    fullname: String,
    level: String,
    class: String,
    city: String,
    guild: String,
    race: String,
}

fn get_epoch_time() -> u128 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("Bad epoch").as_millis()
}

fn from_epoch_time(time: u128) -> SystemTime {
    SystemTime::UNIX_EPOCH.checked_add(Duration::from_millis(time as u64)).expect("Bad add")
}

fn time_since_epoch(time: u128) -> Duration {
    SystemTime::now().duration_since(from_epoch_time(time)).expect("Bad duration")
}

#[tokio::main]
pub async fn retrieve_api(who: String) -> reqwest::Result<CharacterApiResponse> {
    let api_url = format!("https://api.aetolia.com/characters/{}.json", who);
    println!("Retrieving {}", api_url);
    reqwest::get(&api_url).await?.json::<CharacterApiResponse>().await
}


impl DatabaseModule {
    pub fn new<P: AsRef<Path>>(path: P) -> DatabaseModule {
        let (send_request, receive_request): (Sender<ApiRequest>, Receiver<ApiRequest>) = channel();
        let (send_response, receive_response): (Sender<CharacterApiResponse>, Receiver<CharacterApiResponse>) = channel();
        DatabaseModule {
            db: open(path).unwrap(),
            api_sender: send_request,
            api_receiver: receive_response,
            thread: thread::spawn(move || {
                loop {
                    while let Ok(ApiRequest(who)) = receive_request.try_recv() {
                        if let Ok(api_response) = retrieve_api(who) {
                            send_response.send(api_response);
                        }
                    }
                    thread::sleep_ms(100);
                }
            }),
        }
    }

    fn insert(&self, tree: &str, key: String, value: IVec) {
        self.db
            .open_tree(tree)
            .expect(format!("Bad {} tree", tree).as_ref())
            .insert(key.clone(), value)
            .expect(format!("Bad {} insert", key).as_ref());
    }

    fn insert_json<T: Serialize>(&self, tree: &str, key: String, value: T) {
        let json = serde_json::to_string(&value).unwrap();
        self.insert(tree, key, json.as_bytes().into())
    }

    fn get(&self, tree: &str, key: String) -> Option<IVec> {
        self.db
            .open_tree(tree)
            .expect(format!("Bad {} tree", tree).as_ref())
            .get(key.clone())
            .expect(format!("Bad {} get", key).as_ref())
    }

    fn get_json<T: DeserializeOwned>(&self, tree: &str, key: String) -> Option<T> {
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

    fn send_api_request(&self, who: String, priority: u64) -> bool {
        let last_bytes = self.get("api_last", who.clone()).unwrap_or((&[0; 16]).into());
        let last = u128::from_be_bytes(last_bytes.as_ref().try_into().expect("Bad length"));
        let since = time_since_epoch(last);
        if since.as_secs() > priority {
            let epoch = get_epoch_time();
            self.insert("api_last", who.clone(), (&epoch.to_be_bytes()).into());
            self.api_sender.send(ApiRequest(who)).is_ok()
        } else {
            false
        }
    }

    fn drain_responses(&self) {
        while let Ok(character) = self.api_receiver.try_recv() {
            println!("Received response for {} ({} from {})", character.name, character.class, character.city);
            if let Some(class) = Class::from_str(&character.class) {
                self.set_class(character.name.clone(), class);
            }
            self.insert_json("character", character.name.clone(), character);
        }
    }

    pub fn get_class(&self, who: String) -> Option<Class> {
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

    pub fn set_class(&self, who: String, class: Class) {
        self.insert("classes", who, (&[class as u8]).into());
    }

    pub fn get_character(&self, who: String) -> Option<(Option<Class>, String)> {
        if let Some(character) = self.get_json::<CharacterApiResponse>("character", who) {
            Some((Class::from_str(&character.class), character.city.clone()))
        } else {
            None
        }
    }
}

impl<'s> TopperModule<'s> for DatabaseModule {
    type Siblings = ();
    fn handle_message(
        &mut self,
        message: &TopperMessage,
        siblings: Self::Siblings,
    ) -> Result<TopperResponse, String> {
        self.drain_responses();
        match message {
            TopperMessage::Request(TopperRequest::Api(who)) => {
                self.send_api_request(who.to_string(), 300);
                Ok(TopperResponse::silent())
            }
            TopperMessage::Event(event) => {
                for observation in event.observations.iter() {
                    match observation {
                        crate::timeline::Observation::CombatAction(crate::timeline::CombatAction { caster, target, .. }) => {
                            if !caster.eq("") {
                                self.send_api_request(caster.to_string(), 3600 * 2);
                            }
                            if !target.eq("") {
                                self.send_api_request(target.to_string(), 3600 * 2);
                            }
                        }
                        _ => {
                        }
                    }
                }
                Ok(TopperResponse::silent())
            }
            _ => Ok(TopperResponse::silent()),
        }
    }
}
