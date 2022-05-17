use crate::{observables::*, timeline::*, types::*};

pub struct RefillPipeAction {
    pub id: usize,
    pub herb: String,
    pub empty: bool,
}

impl RefillPipeAction {
    pub fn new(id: usize, herb: String, empty: bool) -> Self {
        RefillPipeAction { id, herb, empty }
    }
}

impl ActiveTransition for RefillPipeAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        ProbableEvent::certain(vec![AetObservation::FillPipe(self.herb.clone())])
    }
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        if self.empty {
            Ok(format!(
                "empty {};;outc {};;put {} in {}",
                self.id, self.herb, self.herb, self.id
            ))
        } else {
            Ok(format!(
                "outc {};;put {} in {}",
                self.herb, self.herb, self.id
            ))
        }
    }
}

pub fn get_needed_refills(who: &AgentState) -> Vec<RefillPipeAction> {
    who.pipe_state
        .get_needed_refills()
        .iter()
        .map(|(herb, id)| RefillPipeAction {
            id: *id,
            herb: herb.clone(),
            empty: true,
        })
        .collect::<Vec<RefillPipeAction>>()
}
