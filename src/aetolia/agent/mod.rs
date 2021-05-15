pub mod general;
pub mod limbs;
pub mod shapeshifter;
pub mod syssin;
pub mod zealot;
pub use general::*;
pub use limbs::*;
pub use shapeshifter::*;
pub use syssin::*;
pub use zealot::*;
pub use crate::timeline::CType;
pub const BALANCE_SCALE: f32 = 100.0;

#[derive(Debug, Clone, Copy)]
pub enum TimedFlagState {
    Inactive,
    Active(CType),
}

impl Default for TimedFlagState {
    fn default() -> Self {
        TimedFlagState::Inactive
    }
}

impl TimedFlagState {
    pub fn wait(&mut self, duration: CType) {
        match self.clone() {
            TimedFlagState::Inactive => {}
            TimedFlagState::Active(remaining) => {
                if remaining > duration {
                    *self = TimedFlagState::Active(remaining - duration);
                } else {
                    *self = TimedFlagState::Inactive;
                }
            }
        }
    }

    pub fn active(&self) -> bool {
        match self {
            TimedFlagState::Inactive => false,
            _ => true,
        }
    }

    pub fn activate(&mut self, duration: CType) {
        *self = TimedFlagState::Active(duration);
    }

    pub fn deactivate(&mut self) {
        *self = TimedFlagState::Inactive;
    }
}