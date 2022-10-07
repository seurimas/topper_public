use super::*;

const AGGRO_WINDOWS: CType = (BALANCE_SCALE * 10.0) as CType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AggroState {
    timer: CType,
    latest: i32,
    oldest: i32,
}

impl Default for AggroState {
    fn default() -> Self {
        AggroState {
            timer: 0,
            latest: 0,
            oldest: 0,
        }
    }
}

impl AggroState {
    pub fn wait(&mut self, duration: CType) {
        self.timer -= duration;
        if self.timer <= 0 {
            self.oldest = self.latest;
            self.latest = 0;
            self.timer = AGGRO_WINDOWS;
        }
    }

    pub fn register_hit(&mut self) {
        self.latest += 1;
    }

    pub fn get_aggro(&self) -> i32 {
        self.latest + self.oldest
    }
}
