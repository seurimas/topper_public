use super::*;
use crate::combinatorics::combinations;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum RelapseState {
    Inactive,
    Active(Vec<(CType, String)>),
}

pub enum RelapseResult {
    Concrete(Vec<String>),
    Uncertain(usize, Vec<(CType, String)>),
    None,
}

impl RelapseResult {
    fn branch(&self, me: &AgentState) -> Vec<AgentState> {
        vec![]
    }
}

impl Default for RelapseState {
    fn default() -> Self {
        RelapseState::Inactive
    }
}

impl RelapseState {
    pub fn wait(&mut self, duration: CType) {
        match self {
            RelapseState::Active(relapses) => {
                for relapse in relapses.iter_mut() {
                    relapse.0 += duration;
                }
            }
            RelapseState::Inactive => {}
        }
    }

    pub fn push(&mut self, venom: String) {
        match self {
            RelapseState::Active(relapses) => {
                relapses.push((0 as CType, venom));
            }
            RelapseState::Inactive => {
                *self = RelapseState::Active(vec![(0 as CType, venom)]);
            }
        }
    }

    fn is_venom_ripe(time: CType) -> bool {
        time > (1.9 * BALANCE_SCALE as f32) as CType && time < (7.1 * BALANCE_SCALE as f32) as CType
    }

    fn is_venom_alive(time: CType) -> bool {
        time < (7.1 * BALANCE_SCALE as f32) as CType
    }

    pub fn stalest(&self, venoms: Vec<String>) -> Option<String> {
        match self {
            RelapseState::Active(relapses) => {
                let mut ages = HashMap::new();
                for venom in venoms.iter() {
                    ages.insert(venom, BALANCE_SCALE as CType * 10);
                }
                for (time, venom) in relapses.iter() {
                    if ages.contains_key(venom) {
                        ages.insert(venom, *time);
                    }
                }
                ages.iter()
                    .max_by_key(|(venom, age)| *age)
                    .map(|(venom, age)| venom.to_string())
            }
            _ => venoms.get(0).cloned(),
        }
    }

    pub fn get_relapses(&mut self, relapse_count: usize) -> RelapseResult {
        match self {
            RelapseState::Active(relapses) => {
                let mut possible = Vec::new();
                for (time, venom) in relapses.iter() {
                    if RelapseState::is_venom_ripe(*time) {
                        possible.push(venom.to_string());
                    }
                }
                if possible.len() == relapse_count {
                    relapses.retain(|(time, _venom)| {
                        !RelapseState::is_venom_ripe(*time) && RelapseState::is_venom_alive(*time)
                    });
                    RelapseResult::Concrete(possible)
                } else if possible.len() > 0 {
                    relapses.retain(|(time, _venom)| RelapseState::is_venom_alive(*time));
                    RelapseResult::Uncertain(relapse_count, relapses.clone())
                } else {
                    RelapseResult::None
                }
            }
            RelapseState::Inactive => RelapseResult::None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Hypnosis {
    Aff(FType),
    Action(String),
    Bulimia,
    Eradicate,
    Trigger(String),
}

#[derive(Debug, Clone, Default)]
pub struct HypnoState {
    pub hypnotized: bool,
    pub active: bool,
    pub sealed: Option<f32>,
    pub hypnosis_stack: Vec<Hypnosis>,
}

impl HypnoState {
    pub fn suggestion_count(&self) -> usize {
        self.hypnosis_stack.len()
    }

    pub fn fire(&mut self) -> Option<Hypnosis> {
        if self.hypnosis_stack.len() <= 1 {
            self.active = false;
        } else if !self.active {
            self.activate();
        }
        if self.hypnosis_stack.len() > 0 {
            let top = self.hypnosis_stack.get(0).cloned();
            self.hypnosis_stack.remove(0);
            top
        } else {
            self.desway();
            None
        }
    }

    pub fn pop_suggestion(&mut self, active: bool) -> Option<Hypnosis> {
        if self.hypnosis_stack.len() > 0 {
            if active {
                if self.hypnosis_stack.len() == 1 {
                    self.active = false;
                } else if !self.active {
                    self.active = true;
                }
            }
            self.hypnosis_stack.pop()
        } else {
            None
        }
    }

    pub fn push_suggestion(&mut self, suggestion: Hypnosis) {
        self.hypnosis_stack.push(suggestion);
        self.active = false;
        self.hypnotized = true;
        self.sealed = None;
    }

    pub fn get_next_hypno_aff(&self) -> Option<FType> {
        if !self.active {
            return None;
        }
        if let Some(Hypnosis::Aff(aff)) = self.hypnosis_stack.get(0) {
            Some(*aff)
        } else {
            None
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
        self.sealed = None;
        self.hypnosis_stack = self
            .hypnosis_stack
            .iter()
            .filter(|item| match item {
                Hypnosis::Trigger(_) => false,
                _ => true,
            })
            .cloned()
            .collect();
    }

    pub fn hypnotize(&mut self) {
        self.hypnotized = true;
        self.active = false;
        self.sealed = None;
    }

    pub fn desway(&mut self) {
        self.hypnotized = false;
        self.active = false;
        self.sealed = None;
        self.hypnosis_stack = Vec::new();
    }

    pub fn seal(&mut self, length: f32) {
        self.sealed = Some(length);
        self.hypnotized = false;
        self.active = false;
    }
}
