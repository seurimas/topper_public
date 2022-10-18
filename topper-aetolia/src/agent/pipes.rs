use super::*;

const PIPE_PUFFS: usize = 10;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Pipe {
    pub artifact: bool,
    pub lit: CType,
    pub id: usize,
    pub puffs: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PipeState {
    UnknownFilled,
    UnknownFilledPuffs(usize),
    UnknownUnfilled,
    Known(Pipe),
}

impl Default for PipeState {
    fn default() -> Self {
        Self::UnknownFilled
    }
}

impl PipeState {
    fn wait(&mut self, time: CType) {
        match self {
            Self::Known(pipe) => {
                if !pipe.artifact {
                    pipe.lit -= time;
                }
            }
            _ => {}
        }
    }

    fn refill(&mut self) {
        match self {
            Self::Known(pipe) => {
                pipe.puffs = PIPE_PUFFS;
            }
            _ => {
                *self = Self::UnknownFilled;
            }
        }
    }

    fn puff(&mut self) -> bool {
        match self {
            Self::Known(pipe) => {
                if pipe.puffs > 0 {
                    pipe.puffs -= 1;
                    return true;
                }
                false
            }
            Self::UnknownUnfilled => {
                *self = Self::UnknownFilledPuffs(9);
                true
            }
            Self::UnknownFilledPuffs(puffs) => {
                if *puffs > 1 {
                    *self = Self::UnknownFilledPuffs(*puffs - 1);
                } else {
                    *self = Self::UnknownUnfilled
                }
                true
            }
            _ => true,
        }
    }

    fn puff_all(&mut self) {
        match self {
            Self::Known(pipe) => {
                if pipe.puffs > 0 {
                    pipe.puffs = 0;
                }
            }
            _ => {
                *self = Self::UnknownUnfilled;
            }
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Self::Known(pipe) => pipe.puffs <= 0,
            Self::UnknownFilled | Self::UnknownFilledPuffs(_) => false,
            Self::UnknownUnfilled => true,
        }
    }

    fn needs_refill(&self) -> Option<usize> {
        match self {
            Self::Known(pipe) => {
                if pipe.puffs > 0 {
                    None
                } else {
                    Some(pipe.id)
                }
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct PipesState {
    yarrow: PipeState,
    reishi: PipeState,
    willow: PipeState,
}

pub fn herb_from_string(herb_str: &String) -> &str {
    if herb_str.contains("willow") {
        "willow"
    } else if herb_str.contains("reishi") {
        "reishi"
    } else if herb_str.contains("yarrow") {
        "yarrow"
    } else {
        "empty"
    }
}

impl PipesState {
    pub fn wait(&mut self, time: CType) {
        self.yarrow.wait(time);
        self.reishi.wait(time);
        self.willow.wait(time);
    }

    pub fn puff(&mut self, herb: &str) -> bool {
        match herb {
            "yarrow" => self.yarrow.puff(),
            "willow" => self.willow.puff(),
            "reishi" => self.reishi.puff(),
            _ => false,
        }
    }

    pub fn puff_all(&mut self, herb: &str) {
        match herb {
            "yarrow" => self.yarrow.puff_all(),
            "willow" => self.willow.puff_all(),
            "reishi" => self.reishi.puff_all(),
            _ => {}
        }
    }

    pub fn refill(&mut self, herb: &str) {
        match herb {
            "yarrow" => self.yarrow.refill(),
            "willow" => self.willow.refill(),
            "reishi" => self.reishi.refill(),
            _ => {}
        }
    }

    pub fn initialize(&mut self, herb: &str, pipe: Pipe) {
        match herb {
            "yarrow" => self.yarrow = PipeState::Known(pipe),
            "willow" => self.willow = PipeState::Known(pipe),
            "reishi" => self.reishi = PipeState::Known(pipe),
            _ => {}
        }
    }

    pub fn get_needed_refills(&self) -> Vec<(String, usize)> {
        let mut refills = Vec::new();
        if let Some(id) = self.yarrow.needs_refill() {
            refills.push(("yarrow".to_string(), id));
        }
        if let Some(id) = self.willow.needs_refill() {
            refills.push(("willow".to_string(), id));
        }
        if let Some(id) = self.reishi.needs_refill() {
            refills.push(("reishi".to_string(), id));
        }
        refills
    }

    pub fn get_empties(&self) -> Vec<String> {
        let mut empties = Vec::new();
        if self.yarrow.is_empty() {
            empties.push("yarrow".to_string());
        }
        if self.willow.is_empty() {
            empties.push("willow".to_string());
        }
        if self.reishi.is_empty() {
            empties.push("reishi".to_string());
        }
        empties
    }
}
