use crate::classes::Class;
use crate::observables::ActionPlan;
use crate::timeline::aetolia::AetTimeline;
use crate::timeline::BaseTimeline;
use crate::topper::db::DatabaseModule;
use std::collections::HashMap;

pub trait ActionPlanner {
    fn get_strategies(&self) -> &'static [&'static str];
    fn get_plan(
        &self,
        timeline: &AetTimeline,
        actor: &String,
        target: &String,
        strategy: &str,
        db: Option<&DatabaseModule>,
    ) -> ActionPlan;
}

struct DuelSimulation<M: ActionPlanner, T: ActionPlanner> {
    timeline: AetTimeline,
    duelists: (Duelist<M>, Duelist<T>),
}

struct Duelist<P: ActionPlanner> {
    name: String,
    action_planner: P,
}

struct SimulationIterator<'s, D: ActionPlanner> {
    timeline: &'s AetTimeline,
    duelist: &'s Duelist<D>,
    target: &'s String,
    index: usize,
    db: Option<&'s DatabaseModule>,
}

impl<'s, D: ActionPlanner> SimulationIterator<'s, D> {
    pub fn new(
        timeline: &'s AetTimeline,
        duelist: &'s Duelist<D>,
        target: &'s String,
        db: Option<&'s DatabaseModule>,
    ) -> Self {
        SimulationIterator {
            timeline,
            duelist,
            target,
            index: 0,
            db,
        }
    }
}

impl<'s, D: ActionPlanner> Iterator for SimulationIterator<'s, D> {
    type Item = AetTimeline;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(strategy) = self.duelist.action_planner.get_strategies().get(self.index) {
            self.index += 1;
            let action_plan = self.duelist.action_planner.get_plan(
                &self.timeline,
                &self.duelist.name,
                self.target,
                strategy,
                self.db,
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
