use topper_core::timeline::{BaseAgentState, Timeline};

pub struct TimelineModule<O, P, A, N> {
    pub timeline: Timeline<O, P, A, N>,
}

impl<O, P, A: BaseAgentState + Clone, N: Clone> TimelineModule<O, P, A, N> {
    pub fn new() -> Self {
        TimelineModule {
            timeline: Timeline::<O, P, A, N>::new(),
        }
    }
}
