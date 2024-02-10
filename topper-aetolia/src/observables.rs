use crate::timeline::{simulation_slice, AetObservation, AetTimeSlice, AetTimeline};
use crate::types::BType;
use std::collections::HashMap;
use topper_core::timeline::CType;

// A list of states and their relative weights.
pub type ActiveEvent = Vec<AetObservation>;
pub struct ProbableEvent(ActiveEvent, u32);
pub type Activator = String;
pub type ActivatorFailure = String;
pub type ActivateResult = Result<Activator, ActivatorFailure>;

impl ProbableEvent {
    pub fn new(observations: ActiveEvent, weight: u32) -> Self {
        ProbableEvent(observations, weight)
    }
    pub fn certain(observations: ActiveEvent) -> Vec<Self> {
        vec![Self::new(observations, 1)]
    }
}

pub trait ActiveTransition {
    fn act(&self, timline: &AetTimeline) -> ActivateResult;
    fn simulate(&self, timline: &AetTimeline) -> Vec<ProbableEvent> {
        todo!()
    }
}

#[derive(Default)]
pub struct ActionPlan {
    who: String,
    qeb: Option<Box<dyn ActiveTransition>>,
    back_qeb: Option<Box<dyn ActiveTransition>>,
    other: HashMap<BType, Box<dyn ActiveTransition>>,
}

impl core::fmt::Debug for ActionPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.who)
    }
}

impl ActionPlan {
    pub fn new(who: &str) -> Self {
        ActionPlan {
            who: who.to_string(),
            qeb: None,
            back_qeb: None,
            other: HashMap::new(),
        }
    }

    pub fn join(
        old_qeb: Box<dyn ActiveTransition>,
        action: Box<dyn ActiveTransition>,
    ) -> Box<dyn ActiveTransition> {
        Box::new(SeparatorAction::pair(old_qeb, action))
    }

    pub fn add_to_qeb(&mut self, action: Box<dyn ActiveTransition>) {
        if self.qeb.is_some() {
            self.qeb = self
                .qeb
                .take()
                .map(|old_qeb| ActionPlan::join(old_qeb, action));
        } else {
            self.qeb = Some(action);
        }
    }

    pub fn add_to_front_of_qeb(&mut self, action: Box<dyn ActiveTransition>) {
        if self.qeb.is_some() {
            self.qeb = self
                .qeb
                .take()
                .map(|old_qeb| ActionPlan::join(action, old_qeb));
        } else {
            self.qeb = Some(action);
        }
    }

    pub fn add_to_back_of_qeb(&mut self, action: Box<dyn ActiveTransition>) {
        if self.back_qeb.is_some() {
            self.back_qeb = self
                .back_qeb
                .take()
                .map(|old_qeb| ActionPlan::join(old_qeb, action));
        } else {
            self.back_qeb = Some(action);
        }
    }

    pub fn queue_for(&mut self, bal: BType, action: Box<dyn ActiveTransition>) {
        self.other.insert(bal, action);
    }

    pub fn get_inputs(&self, timeline: &AetTimeline) -> String {
        let mut inputs = "".to_string();
        if let Some(Ok(qeb)) = self.qeb.as_ref().map(|action| action.act(&timeline)) {
            inputs = format!("qeb {}", qeb);
            if let Some(Ok(back_qeb)) = self.back_qeb.as_ref().map(|action| action.act(&timeline)) {
                inputs = format!("{};;{}", inputs, back_qeb);
            }
        } else if let Some(Ok(qeb)) = self.back_qeb.as_ref().map(|action| action.act(&timeline)) {
            inputs = format!("qeb {}", qeb);
        }
        if let Some(Ok(qs)) = self
            .other
            .get(&BType::Secondary)
            .map(|action| action.act(&timeline))
        {
            inputs = format!("{}%%qs {}", inputs, qs);
        }
        inputs
    }

    fn get_next_balance(
        &self,
        timeline: &AetTimeline,
    ) -> Option<(&Box<dyn ActiveTransition>, BType, CType)> {
        let mut next_balance = None;
        let me = timeline.state.borrow_agent(&self.who);
        if let Some(qeb) = &self.qeb {
            next_balance = Some((qeb, me.qeb_balance(), me.get_raw_balance(me.qeb_balance())));
        }
        if let Some(other) = me.next_balance(self.other.keys()) {
            if let Some((_, bal_or_eq, time)) = next_balance {
                if time > me.get_raw_balance(other) {
                    next_balance = Some((
                        self.other.get(&other).unwrap(),
                        other,
                        me.get_raw_balance(other),
                    ));
                }
            } else {
                next_balance = Some((
                    self.other.get(&other).unwrap(),
                    other,
                    me.get_raw_balance(other),
                ));
            }
        }
        next_balance
    }

    pub fn get_time_slice(&self, timeline: &AetTimeline) -> Option<AetTimeSlice> {
        if let Some((transition, balance, time)) = self.get_next_balance(timeline) {
            if let Some(ProbableEvent(observations, _)) = transition.simulate(timeline).first() {
                Some(simulation_slice(observations.to_vec(), time))
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub struct Inactivity;

impl ActiveTransition for Inactivity {
    fn act(&self, timline: &AetTimeline) -> ActivateResult {
        Ok(format!(""))
    }
}

pub struct PlainAction(String);

impl PlainAction {
    pub fn new(action: String) -> Self {
        PlainAction(action)
    }
}

impl ActiveTransition for PlainAction {
    fn act(&self, timline: &AetTimeline) -> ActivateResult {
        Ok(self.0.clone())
    }
}

pub struct Trace(String);

impl Trace {
    pub fn new(trace: String) -> Self {
        Trace(trace)
    }
}

impl ActiveTransition for Trace {
    fn act(&self, timline: &AetTimeline) -> ActivateResult {
        Ok(format!("echo {}", self.0))
    }
}

pub struct SeparatorAction(Box<dyn ActiveTransition>, Box<dyn ActiveTransition>);

impl SeparatorAction {
    pub fn pair(first: Box<dyn ActiveTransition>, second: Box<dyn ActiveTransition>) -> Self {
        SeparatorAction(first, second)
    }
}

impl ActiveTransition for SeparatorAction {
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(format!(
            "{};;{}",
            self.0.act(&timeline)?,
            self.1.act(&timeline)?
        ))
    }
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        let mut results = vec![];
        for ProbableEvent(simulate_first, weight_first) in self.0.simulate(&timeline).iter() {
            for ProbableEvent(simulate_second, weight_second) in self.1.simulate(&timeline).iter() {
                let mut observations = vec![];
                observations.append(&mut simulate_first.clone());
                observations.append(&mut simulate_second.clone());
                results.push(ProbableEvent(observations, weight_first * weight_second));
            }
        }
        results
    }
}
