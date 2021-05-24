use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ZenithState {
    Inactive,
    Rising(CType),
    Active(CType),
}

impl Default for ZenithState {
    fn default() -> Self {
        ZenithState::Inactive
    }
}

impl ZenithState {
    pub fn wait(&mut self, duration: CType) {
        match self.clone() {
            ZenithState::Inactive => {}
            ZenithState::Rising(remaining) => {
                if remaining > duration {
                    *self = ZenithState::Rising(remaining - duration);
                } else {
                    self.activate();
                }
            }
            ZenithState::Active(remaining) => {
                if remaining > duration {
                    *self = ZenithState::Active(remaining - duration);
                } else {
                    self.deactivate();
                }
            }
        }
    }
    pub fn initiate(&mut self) {
        *self = ZenithState::Rising((15.0 * BALANCE_SCALE) as CType);
    }
    pub fn activate(&mut self) {
        *self = ZenithState::Active((10.0 * BALANCE_SCALE) as CType);
    }
    pub fn deactivate(&mut self) {
        *self = ZenithState::Inactive;
    }
    pub fn can_initiate(&self) -> bool {
        match self {
            ZenithState::Inactive => true,
            _ => false,
        }
    }
    pub fn active(&self) -> bool {
        match self {
            ZenithState::Active(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct ZealotClassState {
    pub zenith: ZenithState,
    pub pyromania: TimedFlagState,
}
