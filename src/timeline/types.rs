use log::warn;
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
pub type CType = i32;

#[derive(Debug, Deserialize, Clone)]
pub struct TimeSlice<O, P> {
    pub observations: Option<Vec<O>>,
    pub lines: Vec<(String, u32)>,
    pub prompt: P,
    pub time: CType,
    pub me: String,
}

pub trait BaseAgentState {
    fn get_base_state() -> Self;
    fn wait(&mut self, time: i32);
}

#[derive(Clone)]
pub struct TimelineState<A> {
    pub agent_states: HashMap<String, A>,
    pub free_hints: HashMap<String, String>,
    pub time: CType,
    pub me: String,
}

impl<A: BaseAgentState + Clone> TimelineState<A> {
    pub fn new() -> Self {
        TimelineState {
            agent_states: HashMap::new(),
            free_hints: HashMap::new(),
            time: 0,
            me: "".to_string(),
        }
    }

    pub fn add_player_hint(&mut self, name: &str, hint_type: &str, hint: String) {
        self.free_hints
            .insert(format!("{}_{}", name, hint_type), hint);
    }

    pub fn get_player_hint(&self, name: &String, hint_type: &String) -> Option<String> {
        self.free_hints
            .get(&format!("{}_{}", name, hint_type))
            .cloned()
    }

    pub fn get_agent(&mut self, name: &String) -> A {
        self.borrow_agent(name)
    }

    pub fn get_me(&mut self) -> A {
        self.get_agent(&self.me.clone())
    }

    pub fn get_my_hint(&self, hint_type: &String) -> Option<String> {
        self.get_player_hint(&self.me, hint_type)
    }

    pub fn borrow_agent(&self, name: &String) -> A {
        self.agent_states
            .get(name)
            .cloned()
            .unwrap_or_else(A::get_base_state)
    }

    pub fn borrow_me(&self) -> A {
        self.borrow_agent(&self.me.clone())
    }

    pub fn set_agent(&mut self, name: &String, state: A) {
        self.agent_states.insert(name.to_string(), state);
    }

    fn wait(&mut self, duration: CType) {
        for agent_state in self.agent_states.values_mut() {
            agent_state.wait(duration);
        }
    }

    pub fn update_time(&mut self, when: CType) -> Result<(), String> {
        if when > self.time {
            self.wait(when - self.time);
            self.time = when;
        }
        Ok(())
    }
}

pub struct Timeline<O, P, A> {
    pub slices: Vec<TimeSlice<O, P>>,
    pub state: TimelineState<A>,
}

pub trait BaseTimeline<O, P> {
    fn push_time_slice(&mut self, slice: TimeSlice<O, P>) -> Result<(), String>;
}

impl<O, P, A: BaseAgentState + Clone> Timeline<O, P, A> {
    pub fn new() -> Self {
        Timeline {
            slices: Vec::new(),
            state: TimelineState::<A>::new(),
        }
    }

    pub fn branch(&self) -> Self {
        Timeline {
            slices: Vec::new(),
            state: self.state.clone(),
        }
    }

    pub fn update_time(&mut self, when: CType) -> Result<(), String> {
        self.state.update_time(when)
    }

    pub fn who_am_i(&self) -> String {
        self.state.me.clone()
    }
}
