use crate::classes::Class;
use sled::{open, Db};
use std::path::Path;
use serde::Deserialize;
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

#[derive(Debug, Deserialize)]
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

    fn send_api_request(&self, who: String) {
        let last_bytes = self.db.open_tree("api_last").expect("Bad api_last tree").get(who.clone()).expect("Bad api get").unwrap_or((&[0; 16]).into());
        let last = u128::from_be_bytes(last_bytes.as_ref().try_into().expect("Bad length"));
        let since = time_since_epoch(last);
        if since.as_secs() > 300 {
            self.api_sender.send(ApiRequest(who));
        } else {
            println!("Ignoring request for {}, last request only {} seconds ago.", who, since.as_secs());
        }
    }

    fn drain_responses(&self) {
        while let Ok(character) = self.api_receiver.try_recv() {
            println!("Received response for {}", character.name);
            let epoch = get_epoch_time();
            self.db.open_tree("api_last").expect("Bad api_last tree").insert(character.name.clone(), &epoch.to_be_bytes()).expect("Bad api insert");
            if let Some(class) = Class::from_str(&character.class) {
                self.set_class(character.name, class);
            }
        }
    }

    pub fn get_class(&self, who: String) -> Option<Class> {
        if let Ok(tree) = self.db.open_tree("classes") {
            if let Ok(Some(value)) = tree.get(who) {
                if let [value] = value.as_ref() {
                    if let Ok(class) = Class::try_from(*value) {
                        Some(class)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn set_class(&self, who: String, class: Class) {
        if let Ok(tree) = self.db.open_tree("classes") {
            tree.insert(who.as_bytes(), &[class as u8]);
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
                self.send_api_request(who.to_string());
                Ok(TopperResponse::silent())
            }
            _ => Ok(TopperResponse::silent()),
        }
    }
}
