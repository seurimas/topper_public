pub mod behavior;
pub mod db;
pub mod dodge;
pub mod parry;
pub mod pipes;
use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard};

pub use behavior::*;
pub use db::*;
pub use dodge::*;
pub use parry::*;
pub use pipes::*;
use topper_core::timeline::db::DummyDatabaseModule;

use crate::{db::AetDatabaseModule, timeline::AetTimeline, types::*};

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
