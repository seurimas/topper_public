use super::*;
use crate::timeline::BaseAgentState;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct HiddenState {
    unknown: usize,      // Truly unknown, with no guesses. Non-branched.
    guessed: Vec<FType>, // Partially unknown, some guesses existing in this branch.
}

impl HiddenState {
    pub fn found_out(&mut self) -> bool {
        if self.unknown > 0 {
            self.unknown = self.unknown - 1;
            true
        } else {
            false
        }
    }
    pub fn add_guess(&mut self, flag: FType) -> bool {
        if self.guessed.contains(&flag) {
            false
        } else {
            self.guessed.push(flag);
            true
        }
    }
    pub fn unhide(&mut self, flag: FType) {
        self.guessed.retain(|aff| *aff != flag);
    }
    pub fn guesses(&self) -> usize {
        self.guessed.len()
    }
    pub fn guessed(&self, flag: FType) -> bool {
        self.guessed.contains(&flag)
    }
}
