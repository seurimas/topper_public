use super::*;
use crate::timeline::BaseAgentState;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HiddenState {
    unknown: usize,      // Truly unknown, with no guesses. Non-branched.
    guessed: Vec<FType>, // Partially unknown, some guesses existing in this branch.
}
