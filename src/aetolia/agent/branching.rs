use super::*;
use crate::timeline::BaseAgentState;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Branch {
    time: CType,
    strikes: usize,
    points: usize,
}

#[derive(Debug, Clone)]
pub enum BranchState {
    Single,
    Branched(Branch),
}

impl Default for BranchState {
    fn default() -> BranchState {
        BranchState::Single
    }
}
