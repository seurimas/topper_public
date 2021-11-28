use crate::classes::Class;
use crate::db::AetDatabaseModule;
use crate::observables::ActionPlan;
use crate::timeline::{AetObservation, AetPrompt, AetTimeSlice, AetTimeline};
use std::collections::HashMap;
use topper_core::timeline::db::DatabaseModule;
use topper_core::timeline::BaseTimeline;

pub trait ActionPlanner {
    fn get_strategies(&self) -> &'static [&'static str];
    fn get_plan(
        &self,
        timeline: &AetTimeline,
        actor: &String,
        target: &String,
        strategy: &str,
        db: Option<&impl AetDatabaseModule>,
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

struct SimulationIterator<'s, AP: ActionPlanner, DB: AetDatabaseModule> {
    timeline: &'s AetTimeline,
    duelist: &'s Duelist<AP>,
    target: &'s String,
    index: usize,
    db: Option<&'s DB>,
}

impl<'s, AP: ActionPlanner, DB: AetDatabaseModule> SimulationIterator<'s, AP, DB> {
    pub fn new(
        timeline: &'s AetTimeline,
        duelist: &'s Duelist<AP>,
        target: &'s String,
        db: Option<&'s DB>,
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

impl<'s, AP: ActionPlanner, DB: AetDatabaseModule + DatabaseModule> Iterator
    for SimulationIterator<'s, AP, DB>
{
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
                <AetTimeline as BaseTimeline<AetObservation, AetPrompt, DB>>::push_time_slice(
                    &mut new_timeline,
                    timeslice,
                    None,
                );
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
