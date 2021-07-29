use super::*;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct SentinelClassState {
    pub alacrity: u32,
    pub spike: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Resin {
    Pyrolum,
    Corsin,
    Trientia,
    Harimel,
    Glauxe,
    Badulem,
    Lysirine,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct ResinState {
    pub hot: Option<Resin>,
    pub cold: Option<Resin>,
    pub burning: bool,
    pub burning_time: CType,
    pub ticks_left: u8,
}

impl ResinState {
    pub fn wait(&mut self, duration: CType) {
        if self.burning {
            self.burning_time += duration;
        }
        if self.burning_time > 1500 {
            // Clear state.
            self.hot_burn();
        }
    }
    pub fn clear(&mut self) {
        self.burning = false;
        self.burning_time = 0;
        self.ticks_left = 0;
        self.hot = None;
        self.cold = None;
    }
    pub fn apply(&mut self, layer: Resin) {
        self.hot = self.cold.clone();
        self.cold = Some(layer);
        self.burning = false;
        self.burning_time = 0;
        self.ticks_left = 0;
    }
    pub fn ignite(&mut self) {
        self.burning = true;
        self.burning_time = 0;
        self.ticks_left = match self.cold {
            Some(Resin::Pyrolum) => 12,
            Some(Resin::Corsin) => 8,
            Some(Resin::Trientia) => 9,
            Some(Resin::Harimel) => 14,
            Some(Resin::Glauxe) => 10,
            Some(Resin::Badulem) => 8,
            Some(Resin::Lysirine) => 6,
            None => 0,
        }
    }
    pub fn cold_burn(&mut self) {
        self.burning_time = 0;
        if (self.ticks_left > 0) {
            self.ticks_left -= 1;
        }
        if (self.ticks_left == 0) {
            self.cold = None;
        }
    }
    pub fn hot_burn(&mut self) {
        self.clear();
    }
}
