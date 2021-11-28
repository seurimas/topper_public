use topper_core::timeline::{BaseAgentState, Timeline};

pub struct TimelineModule<O, P, A> {
    pub timeline: Timeline<O, P, A>,
}

impl<O, P, A: BaseAgentState + Clone> TimelineModule<O, P, A> {
    pub fn new() -> Self {
        TimelineModule {
            timeline: Timeline::<O, P, A>::new(),
        }
    }
}
