use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard};
use topper_core::timeline::db::DummyDatabaseModule;

use crate::{db::AetDatabaseModule, timeline::AetTimeline, types::*};

// A very impressive type, if I do say so myself. We are a mutable reference to a possible mutable reference vtable object...
lazy_static! {
    pub static ref DEFENSE_DATABASE: Arc<Mutex<Option<Arc<RwLock<dyn AetDatabaseModule + Sync + Send>>>>> =
        Arc::new(Mutex::new(None));
}

#[macro_export]
macro_rules! with_defense_db {
    ($db:ident, $body:block) => {
        match DEFENSE_DATABASE.as_ref().try_lock() {
            Ok(outer_guard) => {
                let option = outer_guard.as_ref();
                if let Some(inner_mutex) = option {
                    match inner_mutex.as_ref().read() {
                        Ok($db) => $body,
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    };
}
