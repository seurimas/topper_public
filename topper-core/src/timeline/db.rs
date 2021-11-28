use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
pub trait DatabaseModule {
    fn insert_json<T: Serialize>(&self, tree: &str, key: &String, value: T);
    fn get_json<T: DeserializeOwned>(&self, tree: &str, key: &String) -> Option<T>;
}
