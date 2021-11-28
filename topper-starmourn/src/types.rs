use crate::timeline::{BaseAgentState, CType};
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
pub const BALANCE_SCALE: f32 = 100.0;
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum BType {
    Balance,
    Wetwiring,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum Subsys {
    Sensory,
    Muscular,
    Internal,
    Mental,
    Wetwiring,

    SIZE,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum LandLocation {
    Known(u32),
    Stale(u32, u32),
    Unknown,
}

#[derive(Debug, Clone, Default)]
pub struct Health {
    pub subsystems: [Subsys; Subsys::SIZE as u8],
    pub health: i32,
}

pub struct BeastState;
pub struct EngineerState {
    pub parts: i8,
}
pub struct NanoseerState {
    pub nanites: i16,
    pub sanity: i8,
}
pub struct ScoundrelState;
pub struct FuryState;

#[derive(Debug, Clone)]
pub enum ClassState {
    Beast(BeastState),
    Engineer(EngineerState),
    Nanoseer(NanoseerState),
    Scoundrel(ScoundrelState),
    Fury(FuryState),
    Unknown,
}

impl Default for ClassState {
    fn default() -> ClassState {
        ClassState::Unknown
    }
}

pub type Channel = String;

#[derive(Debug, Clone)]
pub enum ChannelStatus {
    Continuous,
    Countdown(f32),
}

#[derive(Debug, Clone)]
pub enum Channeling {
    Active,
    Channeling(Channel, ChannalStatus),
    Unknown,
}

impl Default for Channeling {
    fn default() -> Channeling {
        Channeling::Unknown
    }
}

pub type Affliction = String;

#[derive(Debug, Clone)]
pub enum AfflictionStatus {
    Uncured, // Mendable
    Single(f32),
    Stacked(Vec<f32>),
    Unknown(f32),
}

#[derive(Debug, Clone, Default)]
pub struct Afflictions {
    pub status: HashMap<String, AfflictionStatus>,
}

#[derive(Debug, Clone, Default)]
pub struct Landed {
    pub balances: Balance,
    pub location: LandLocation,
    pub health: Health,
    pub class: ClassState,
    pub channel: Channeling,
}
