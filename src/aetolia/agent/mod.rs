pub mod agent;
pub mod general;
pub mod limbs;
pub mod shapeshifter;
pub mod syssin;
pub mod zealot;
pub use agent::*;
pub use general::*;
pub use limbs::*;
pub use shapeshifter::*;
pub use syssin::*;
pub use zealot::*;
pub use crate::timeline::CType;
pub const BALANCE_SCALE: f32 = 100.0;