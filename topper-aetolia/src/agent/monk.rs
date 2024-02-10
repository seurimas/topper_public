use super::*;
use serde::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
pub enum MonkStance {
    #[default]
    None,
    Horse,
    Eagle,
    Cat,
    Bear,
    Rat,
    Scorpion,
    Cobra,
    Phoenix,
    Tiger,
    Wolf,
    Dragon,
}

impl MonkStance {
    pub fn from_name(name: &str) -> Self {
        match name {
            "horse" => MonkStance::Horse,
            "eagle" => MonkStance::Eagle,
            "cat" => MonkStance::Cat,
            "bear" => MonkStance::Bear,
            "rat" => MonkStance::Rat,
            "scorpion" => MonkStance::Scorpion,
            "cobra" => MonkStance::Cobra,
            "phoenix" => MonkStance::Phoenix,
            "tiger" => MonkStance::Tiger,
            "wolf" => MonkStance::Wolf,
            "dragon" => MonkStance::Dragon,
            _ => MonkStance::None,
        }
    }

    pub fn to_name(&self) -> &'static str {
        match self {
            MonkStance::None => "none",
            MonkStance::Horse => "horse",
            MonkStance::Eagle => "eagle",
            MonkStance::Cat => "cat",
            MonkStance::Bear => "bear",
            MonkStance::Rat => "rat",
            MonkStance::Scorpion => "scorpion",
            MonkStance::Cobra => "cobra",
            MonkStance::Phoenix => "phoenix",
            MonkStance::Tiger => "tiger",
            MonkStance::Wolf => "wolf",
            MonkStance::Dragon => "dragon",
        }
    }

    pub fn param_str(&self) -> &'static str {
        match self {
            MonkStance::None => "drs", // Just go into dragon, dunno how we'd get here.
            MonkStance::Horse => "hrs",
            MonkStance::Eagle => "egs",
            MonkStance::Cat => "cts",
            MonkStance::Bear => "brs",
            MonkStance::Rat => "rts",
            MonkStance::Scorpion => "scs",
            MonkStance::Cobra => "cbs",
            MonkStance::Phoenix => "phs",
            MonkStance::Tiger => "tgs",
            MonkStance::Wolf => "wfs",
            MonkStance::Dragon => "drs",
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct MonkClassState {
    pub stance: MonkStance,
    pub kai: CType,
}
