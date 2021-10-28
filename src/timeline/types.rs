use crate::{aetolia::agent::BALANCE_SCALE, topper::db::DatabaseModule};
use log::warn;
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
pub type CType = i32;

#[derive(Debug, Deserialize, Clone)]
pub struct TimeSlice<O, P> {
    pub observations: Option<Vec<O>>,
    pub gmcp: Vec<(String, Value)>,
    pub lines: Vec<(String, u32)>,
    pub prompt: P,
    pub time: CType,
    pub me: String,
}

pub trait BaseAgentState {
    fn get_base_state() -> Self;
    fn wait(&mut self, time: i32);
    fn branch(&mut self, time: i32);
}

pub type AgentStates<A> = HashMap<String, Vec<A>>;

#[derive(Clone)]
pub struct TimelineState<A> {
    pub agent_states: AgentStates<A>,
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

    pub fn is_hint_time_fresh(&self, name: &String, hint_type: &String, freshness: f32) -> bool {
        self.get_player_hint(name, hint_type)
            .and_then(|time| time.parse::<i32>().ok())
            .map(|time| self.time - time)
            .map(|staleness| (staleness as f32) * BALANCE_SCALE <= freshness)
            .unwrap_or(false)
    }

    pub fn get_agent(&self, name: &String) -> Option<&Vec<A>> {
        self.agent_states.get(name)
    }

    fn get_mut_agent(&mut self, name: &String) -> &mut Vec<A> {
        if let Some(agent) = self.agent_states.get(name) {
            self.agent_states.get_mut(name).unwrap()
        } else {
            self.agent_states
                .insert(name.to_string(), vec![A::get_base_state()]);
            self.get_mut_agent(name)
        }
    }

    pub fn get_my_hint(&self, hint_type: &String) -> Option<String> {
        self.get_player_hint(&self.me, hint_type)
    }

    pub fn borrow_agent(&self, name: &String) -> A {
        self.agent_states
            .get(name)
            .and_then(|state| state.first())
            .cloned()
            .unwrap_or_else(A::get_base_state)
    }

    pub fn borrow_me(&self) -> A {
        self.borrow_agent(&self.me.clone())
    }

    pub fn for_agent(&mut self, who: &String, act: fn(&mut A)) {
        for you in self.get_mut_agent(who) {
            act(you);
        }
    }
    pub fn for_agent_closure(&mut self, who: &String, act: Box<dyn Fn(&mut A)>) {
        for you in self.get_mut_agent(who) {
            act(you);
        }
    }

    pub fn for_agent_uncertain(&mut self, who: &String, act: fn(&mut A) -> Option<Vec<A>>) {
        let mut branches = Vec::new();
        let mut unbranched = Vec::new();
        for (i, mut you) in self.get_mut_agent(who).iter_mut().enumerate() {
            if let Some(mut new_branches) = act(you) {
                branches.append(&mut new_branches);
            } else {
                unbranched.push(i);
            }
        }
        if branches.len() > 0 {
            branches.extend(self.get_mut_agent(who).drain(..).enumerate().filter_map(
                |(i, agent)| {
                    if unbranched.contains(&i) {
                        Some(agent)
                    } else {
                        None
                    }
                },
            ));
            self.agent_states.insert(who.clone(), branches);
        }
    }
    pub fn for_agent_uncertain_closure(
        &mut self,
        who: &String,
        act: Box<dyn Fn(&mut A) -> Option<Vec<A>>>,
    ) {
        let mut branches = Vec::new();
        let mut unbranched = Vec::new();
        for (i, mut you) in self.get_mut_agent(who).iter_mut().enumerate() {
            if let Some(mut new_branches) = act(you) {
                branches.append(&mut new_branches);
            } else {
                unbranched.push(i);
            }
        }
        if branches.len() > 0 {
            branches.extend(self.get_mut_agent(who).drain(..).enumerate().filter_map(
                |(i, agent)| {
                    if unbranched.contains(&i) {
                        Some(agent)
                    } else {
                        None
                    }
                },
            ));
            branches.iter_mut().for_each(|branch| {
                branch.branch(self.time);
            });
            self.agent_states.insert(who.clone(), branches);
        }
    }

    fn wait(&mut self, duration: CType) {
        for agent_state in self.agent_states.values_mut() {
            for agent_state in agent_state.iter_mut() {
                agent_state.wait(duration);
            }
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
    fn push_time_slice(
        &mut self,
        slice: TimeSlice<O, P>,
        db: Option<&DatabaseModule>,
    ) -> Result<(), String>;
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
