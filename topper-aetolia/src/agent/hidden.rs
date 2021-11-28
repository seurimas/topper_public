use super::*;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::collections::hash_set::Iter;
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};
use topper_core::timeline::BaseAgentState;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct HiddenState {
    unknown: usize,          // Truly unknown, with no guesses. Non-branched.
    guessed: HashSet<FType>, // Partially unknown, some guesses existing in this branch.
}

impl Hash for HiddenState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.unknown.hash(state);
        let mut affs = self.guessed.iter().collect::<Vec<_>>();
        affs.sort();
        for flag in affs {
            flag.hash(state);
        }
    }
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
        !self.guessed.insert(flag)
    }
    pub fn unhide(&mut self, flag: FType) {
        self.guessed.remove(&flag);
    }
    pub fn guesses(&self) -> usize {
        self.guessed.len()
    }
    pub fn is_guessed(&self, flag: FType) -> bool {
        self.guessed.contains(&flag)
    }
    pub fn iter_guesses(&self) -> Iter<FType> {
        self.guessed.iter()
    }
}
