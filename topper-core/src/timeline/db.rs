use std::sync::Arc;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub const HINT_TREE: &str = "HINTS";

pub trait DatabaseModule {
    fn insert(&self, tree: &str, key: &String, value: &[u8]);
    fn insert_json<T: Serialize>(&self, tree: &str, key: &String, value: T);

    fn get(&self, tree: &str, key: &String) -> Option<Arc<[u8]>>;
    fn get_json<T: DeserializeOwned>(&self, tree: &str, key: &String) -> Option<T>;

    fn insert_hint(&self, key: &String, value: &String) {
        self.insert(HINT_TREE, key, value.as_bytes());
    }
    fn get_hint(&self, key: &String) -> Option<String> {
        self.get(HINT_TREE, key).and_then(|bytes| {
            std::str::from_utf8(&bytes)
                .map(|str_ref| str_ref.to_string())
                .ok()
        })
    }
}

pub struct DummyDatabaseModule;

impl DatabaseModule for DummyDatabaseModule {
    fn insert_json<T: Serialize>(&self, tree: &str, key: &String, value: T) {
        panic!("Dummy called");
    }
    fn get_json<T: DeserializeOwned>(&self, tree: &str, key: &String) -> Option<T> {
        panic!("Dummy called");
    }

    fn insert(&self, tree: &str, key: &String, value: &[u8]) {
        panic!("Dummy called");
    }

    fn get(&self, tree: &str, key: &String) -> Option<Arc<[u8]>> {
        panic!("Dummy called");
    }
}
