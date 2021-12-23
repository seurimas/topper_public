use std::sync::Arc;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
pub trait DatabaseModule {
    fn insert(&self, tree: &str, key: &String, value: &[u8]);
    fn insert_json<T: Serialize>(&self, tree: &str, key: &String, value: T);

    fn get(&self, tree: &str, key: &String) -> Option<Arc<[u8]>>;
    fn get_json<T: DeserializeOwned>(&self, tree: &str, key: &String) -> Option<T>;
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
