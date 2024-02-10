use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use topper_core::combinatorics::combinations;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RelapseState {
    Inactive,
    Active(Vec<(CType, String)>),
}

pub enum RelapseResult {
    Concrete(Vec<String>, usize),
    Uncertain(usize, Vec<(CType, String)>, usize),
    None,
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

    pub fn drop_relapse(&mut self, time: CType, venom: &String) {
        match self {
            RelapseState::Active(relapses) => {
                relapses.retain(|(r_time, r_venom)| *r_time != time || !r_venom.eq(venom));
            }
            RelapseState::Inactive => {}
        }
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
                let mut expired = 0;
                for (time, venom) in relapses.iter() {
                    if RelapseState::is_venom_ripe(*time) {
                        possible.push(venom.to_string());
                    } else if !RelapseState::is_venom_alive(*time) {
                        expired += 1;
                    }
                }
                if possible.len() == relapse_count {
                    relapses.retain(|(time, _venom)| {
                        !RelapseState::is_venom_ripe(*time) && RelapseState::is_venom_alive(*time)
                    });
                    RelapseResult::Concrete(possible, expired)
                } else if possible.len() > 0 {
                    relapses.retain(|(time, _venom)| RelapseState::is_venom_alive(*time));
                    RelapseResult::Uncertain(relapse_count, relapses.clone(), expired)
                } else {
                    RelapseResult::None
                }
            }
            RelapseState::Inactive => RelapseResult::None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub enum Hypnosis {
    Aff(FType),
    Action(String),
    Ebbing,
    Bulimia,
    Eradicate,
    Trigger(String),
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub enum HypnoState {
    #[default]
    Empty,
    Hypnotized(Vec<Hypnosis>),
    Firing(Vec<Hypnosis>, CType),
    Sealed(Vec<Hypnosis>, CType),
}

impl HypnoState {
    pub fn wait(&mut self, duration: CType) {
        match self {
            HypnoState::Firing(_, time) => {
                *time -= duration;
            }
            _ => {}
        }
    }
    pub fn suggestion_count(&self) -> usize {
        match self {
            HypnoState::Empty => 0,
            HypnoState::Hypnotized(suggestions) => suggestions.len(),
            HypnoState::Firing(suggestions, _) => suggestions.len(),
            HypnoState::Sealed(suggestions, _) => suggestions.len(),
        }
    }

    pub fn fire(&mut self) -> Option<Hypnosis> {
        match self {
            HypnoState::Firing(suggestions, timer) => {
                if suggestions.len() > 0 {
                    let top = suggestions.get(0).cloned();
                    *timer = 600;
                    suggestions.remove(0);
                    top
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn lose_suggestion(&mut self, active: bool) -> Option<Hypnosis> {
        match self {
            HypnoState::Hypnotized(suggestions) => {
                if suggestions.len() > 0 {
                    let top = suggestions.get(0).cloned();
                    suggestions.remove(0);
                    top
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn push_suggestion(&mut self, suggestion: Hypnosis) {
        match self {
            HypnoState::Empty => {
                *self = HypnoState::Hypnotized(vec![suggestion]);
            }
            HypnoState::Hypnotized(suggestions) => {
                suggestions.push(suggestion);
            }
            HypnoState::Firing(suggestions, _) => {
                *self = HypnoState::Hypnotized(vec![suggestion]);
            }
            HypnoState::Sealed(suggestions, _) => {
                suggestions.push(suggestion);
                *self = HypnoState::Hypnotized(suggestions.clone());
            }
        }
    }

    pub fn get_next_hypno_aff(&self) -> Option<FType> {
        match self {
            HypnoState::Firing(suggestions, _) => {
                suggestions.get(0).and_then(|suggestion| match suggestion {
                    Hypnosis::Aff(aff) => Some(*aff),
                    _ => None,
                })
            }
            _ => None,
        }
    }

    pub fn activate(&mut self) {
        match self {
            HypnoState::Sealed(suggestions, time) => {
                *self = HypnoState::Firing(suggestions.clone(), *time);
            }
            _ => {}
        }
    }

    pub fn is_hypnotized(&self) -> bool {
        match self {
            HypnoState::Hypnotized(_) => true,
            _ => false,
        }
    }

    pub fn is_sealed(&self) -> bool {
        match self {
            HypnoState::Sealed(_, _) => true,
            _ => false,
        }
    }

    pub fn is_firing(&self) -> bool {
        match self {
            HypnoState::Firing(_, _) => true,
            _ => false,
        }
    }

    pub fn hypnotize(&mut self) {
        match self {
            HypnoState::Empty | HypnoState::Firing(_, _) => {
                *self = HypnoState::Hypnotized(Vec::new());
            }
            HypnoState::Hypnotized(_) => {}
            HypnoState::Sealed(suggestions, _) => {
                *self = HypnoState::Hypnotized(suggestions.clone());
            }
        }
    }

    pub fn desway(&mut self) {
        *self = HypnoState::Empty;
    }

    pub fn seal(&mut self, length: f32) {
        match self {
            HypnoState::Empty | HypnoState::Firing(_, _) => {
                *self = HypnoState::Sealed(Vec::new(), (length * BALANCE_SCALE as f32) as CType);
            }
            HypnoState::Hypnotized(suggestions) => {
                *self = HypnoState::Sealed(
                    suggestions.clone(),
                    (length * BALANCE_SCALE as f32) as CType,
                );
            }
            HypnoState::Sealed(_, _) => {}
        }
    }

    pub fn get_suggestion(&self, idx: usize) -> Option<&Hypnosis> {
        match self {
            HypnoState::Hypnotized(suggestions) => suggestions.get(idx),
            HypnoState::Firing(suggestions, _) => suggestions.get(idx),
            HypnoState::Sealed(suggestions, _) => suggestions.get(idx),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct InfiltratorClassState {
    pub finesse: u32,
}

impl Default for InfiltratorClassState {
    fn default() -> Self {
        InfiltratorClassState { finesse: 0 }
    }
}
