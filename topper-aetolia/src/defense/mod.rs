pub mod behavior;
pub mod parry;
pub mod pipes;
use std::sync::{Arc, Mutex, RwLock};

pub use behavior::*;
pub use parry::*;
pub use pipes::*;
use topper_core::timeline::db::DummyDatabaseModule;

use crate::{db::AetDatabaseModule, timeline::AetTimeline, types::*};

// A very impressive type, if I do say so myself. We are a mutable reference to a possible mutable reference vtable object...
lazy_static! {
    pub static ref DEFENSE_DATABASE: Arc<Mutex<Option<Arc<RwLock<dyn AetDatabaseModule + Sync + Send>>>>> =
        Arc::new(Mutex::new(None));
}

pub fn should_regenerate(timeline: &AetTimeline, me: &String) -> bool {
    let me = timeline.state.borrow_agent(me);
    if me.balanced(BType::Regenerate) {
        false
    } else if let Some((_limb, damage, regenerating)) = me.get_restoring() {
        !regenerating && damage > 4000
    } else {
        false
    }
}

pub fn needs_restore(timeline: &AetTimeline, me: &String) -> bool {
    let me = timeline.state.borrow_agent(me);
    me.restore_count() > 0
        && me.restore_count() < 3
        && me.is(FType::Fallen)
        && me.get_balance(BType::Salve) > 2.5
}
