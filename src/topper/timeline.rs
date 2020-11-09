use crate::timeline::aetolia::{AetObservation, AetPrompt};
use crate::timeline::*;
pub use crate::timeline::{TimeSlice, Timeline};
use crate::topper::{TopperMessage, TopperModule, TopperRequest, TopperResponse};
use crate::types::AgentState;

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

pub type AetTimelineModule = TimelineModule<AetObservation, AetPrompt, AgentState>;

impl<'s> TopperModule<'s> for AetTimelineModule {
    type Siblings = ();
    fn handle_message(
        &mut self,
        message: &TopperMessage,
        siblings: Self::Siblings,
    ) -> Result<TopperResponse, String> {
        match message {
            TopperMessage::AetEvent(timeslice) => {
                self.timeline.push_time_slice(timeslice.clone())?;
                Ok(TopperResponse::silent())
            }
            TopperMessage::Request(request) => match request {
                TopperRequest::BattleStats(when) => {
                    self.timeline.update_time(*when)?;
                    Ok(TopperResponse::silent())
                }
                TopperRequest::Hint(who, hint, value) => {
                    self.timeline
                        .state
                        .add_player_hint(&who, &hint, value.to_string());
                    Ok(TopperResponse::silent())
                }
                TopperRequest::Assume(who, aff_or_def, value) => {
                    self.timeline
                        .state
                        .set_flag_for_agent(&who, &aff_or_def, *value);
                    Ok(TopperResponse::silent())
                }
                TopperRequest::Reset(reset_type) => {
                    self.timeline.reset(reset_type.eq("full"));
                    Ok(TopperResponse::silent())
                }
                _ => Ok(TopperResponse::silent()),
            },
            _ => Ok(TopperResponse::silent()),
        }
    }
}
