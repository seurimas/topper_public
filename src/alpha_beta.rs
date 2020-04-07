use crate::classes::Class;
use crate::observables::ActionPlan;
use crate::timeline::Timeline;
use std::collections::HashMap;

pub trait ActionPlanner {
    fn get_strategies(&self) -> &'static [&'static str];
    fn get_plan(
        &self,
        timeline: &Timeline,
        actor: &String,
        target: &String,
        strategy: &str,
    ) -> ActionPlan;
}

struct DuelSimulation<M: ActionPlanner, T: ActionPlanner> {
    timeline: Timeline,
    duelists: (Duelist<M>, Duelist<T>),
}

struct Duelist<P: ActionPlanner> {
    name: String,
    action_planner: P,
}

struct SimulationIterator<'s, D: ActionPlanner> {
    timeline: &'s Timeline,
    duelist: &'s Duelist<D>,
    target: &'s String,
    index: usize,
}

impl<'s, D: ActionPlanner> SimulationIterator<'s, D> {
    pub fn new(timeline: &'s Timeline, duelist: &'s Duelist<D>, target: &'s String) -> Self {
        SimulationIterator {
            timeline,
            duelist,
            target,
            index: 0,
        }
    }
}

impl<'s, D: ActionPlanner> Iterator for SimulationIterator<'s, D> {
    type Item = Timeline;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(strategy) = self.duelist.action_planner.get_strategies().get(self.index) {
            self.index += 1;
            let action_plan = self.duelist.action_planner.get_plan(
                &self.timeline,
                &self.duelist.name,
                self.target,
                strategy,
            );
            let mut new_timeline = self.timeline.branch();
            if let Some(timeslice) = action_plan.get_time_slice(&new_timeline) {
                new_timeline.push_time_slice(timeslice);
                Some(new_timeline)
            } else {
                self.next()
            }
        } else {
            None
        }
    }
}

impl<M: ActionPlanner, T: ActionPlanner> DuelSimulation<M, T> {}
