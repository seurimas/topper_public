use super::*;

const AGGRO_WINDOWS: CType = (BALANCE_SCALE * 10.0) as CType;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct AggroTimeState {
    count: i32,
    min_health: i32,
    attackers: Vec<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct AggroState {
    timer: CType,
    latest: AggroTimeState,
    oldest: AggroTimeState,
}

impl AggroState {
    pub fn wait(&mut self, duration: CType) {
        self.timer -= duration;
        if self.timer <= 0 {
            core::mem::swap(&mut self.oldest, &mut self.latest);
            self.latest = AggroTimeState::default();
            self.timer = AGGRO_WINDOWS;
        }
    }

    pub fn register_hit(&mut self, attacker: Option<&String>) {
        self.latest.count += 1;
        if let Some(attacker) = attacker {
            if !self.latest.attackers.contains(attacker) {
                self.latest.attackers.push(attacker.clone());
            }
        }
    }

    pub fn get_aggro_count(&self) -> i32 {
        self.latest.count + self.oldest.count
    }

    pub fn register_health(&mut self, health: i32) {
        self.latest.min_health = self.latest.min_health.min(health);
    }

    pub fn get_aggro_health(&self) -> i32 {
        self.latest.min_health.min(self.oldest.min_health)
    }

    pub fn get_aggro_attackers(&self) -> Vec<String> {
        let mut attackers = self.latest.attackers.clone();
        attackers.retain(|attacker| !self.oldest.attackers.contains(attacker));
        attackers.extend(self.oldest.attackers.clone());
        attackers
    }
}
