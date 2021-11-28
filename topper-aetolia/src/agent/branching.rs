use super::*;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;
use topper_core::timeline::BaseAgentState;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Branch {
    time: CType,
    strikes: usize,
    points: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BranchState {
    Single,
    Branched(Branch),
}

impl BranchState {
    pub fn branch(&mut self, time: CType) {
        match self {
            BranchState::Single => {
                *self = BranchState::Branched(Branch {
                    time,
                    strikes: 0,
                    points: 0,
                });
            }
            BranchState::Branched(Branch {
                strikes, points, ..
            }) => {
                *self = BranchState::Branched(Branch {
                    time,
                    strikes: *strikes,
                    points: *points,
                })
            }
        }
    }
    pub fn strike(&mut self) {
        match self {
            BranchState::Single => {}
            BranchState::Branched(Branch { strikes, .. }) => {
                println!("Striking!");
                *strikes = *strikes + 1;
            }
        }
    }
    pub fn strike_aff(&mut self, flag: FType, expected: bool) {
        match self {
            BranchState::Single => {}
            BranchState::Branched(Branch { strikes, .. }) => {
                println!("Striking {:?} {}!", flag, expected);
                *strikes = *strikes + 1;
            }
        }
    }
    pub fn strikes(&self) -> usize {
        match self {
            BranchState::Single => 0,
            BranchState::Branched(Branch { strikes, .. }) => *strikes,
        }
    }
}

impl Default for BranchState {
    fn default() -> BranchState {
        BranchState::Single
    }
}
