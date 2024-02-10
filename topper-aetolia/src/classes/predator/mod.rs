pub mod observation_handling;
pub use observation_handling::*;
pub mod actions;
pub use actions::*;
pub mod combos;
pub use combos::*;
pub mod offense;
pub use offense::*;
pub mod behavior;
pub use behavior::*;
pub mod predicate;
pub use predicate::*;

use crate::types::KnifeStance;

pub const MAWCRUSH_FREELY_HINT: &str = "MAWCRUSH_FREELY";

#[cfg(test)]
#[path = "../tests/predator_tests.rs"]
mod predator_timeline_tests;
