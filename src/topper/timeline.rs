use crate::timeline::*;
pub use crate::timeline::{TimeSlice, Timeline};
use crate::topper::{TopperMessage, TopperModule, TopperRequest, TopperResponse};

pub struct TimelineModule {
    pub timeline: Timeline,
}

impl TimelineModule {
    pub fn new() -> Self {
        TimelineModule {
            timeline: Timeline::new(),
        }
    }
}

impl TopperModule for TimelineModule {
    fn handle_message(&mut self, message: &TopperMessage) -> Result<TopperResponse, String> {
        match message {
            TopperMessage::Event(timeslice) => {
                self.timeline.push_time_slice(timeslice.clone())?;
                Ok(TopperResponse::silent())
            }
            TopperMessage::Request(request) => match request {
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
                TopperRequest::Reset => {
                    self.timeline.reset();
                    Ok(TopperResponse::silent())
                }
                _ => Ok(TopperResponse::silent()),
            },
            _ => Ok(TopperResponse::silent()),
        }
    }
}
