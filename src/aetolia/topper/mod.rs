use crate::aetolia::classes::get_attack;
use crate::aetolia::timeline::{AetObservation, AetPrompt, AetTimeSlice, AetTimeline};
use crate::aetolia::types::AgentState;
use crate::timeline::BaseTimeline;
use crate::topper::{
    DatabaseModule, TelnetModule, TimelineModule, Topper, TopperCore, TopperHandler, TopperMessage,
    TopperModule, TopperRequest, TopperResponse, WebModule,
};
use battle_stats::BattleStatsModule;
use std::sync::mpsc::Sender;
pub mod battle_stats;

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

pub type AetTopper = Topper<AetObservation, AetPrompt, AgentState>;

impl AetTopper {
    pub fn new(send_lines: Sender<String>) -> Self {
        Topper {
            timeline_module: AetTimelineModule::new(),
            core_module: TopperCore::new(),
            telnet_module: TelnetModule::new(send_lines),
            battlestats_module: BattleStatsModule::new(),
            database_module: DatabaseModule::new("topper.db"),
            web_module: WebModule::new(),
        }
    }
}

impl TopperHandler for AetTopper {
    fn handle_request_or_event(
        &mut self,
        topper_msg: &TopperMessage,
    ) -> Result<TopperResponse, String> {
        let module_msg = self
            .core_module
            .handle_message(&topper_msg, ())?
            .then(self.timeline_module.handle_message(&topper_msg, ())?)
            .then(self.telnet_module.handle_message(&topper_msg, ())?)
            .then(self.battlestats_module.handle_message(
                &topper_msg,
                (
                    &self.timeline_module.timeline,
                    &self.core_module.target,
                    &self.database_module,
                ),
            )?)
            .then(self.database_module.handle_message(&topper_msg, ())?)
            .then(self.web_module.handle_message(&topper_msg, ())?);
        match topper_msg {
            TopperMessage::Request(request) => match request {
                TopperRequest::Attack(strategy) => {
                    if let Some(target) = self.get_target() {
                        Ok(module_msg.then(TopperResponse::qeb(get_attack(
                            &self.timeline_module.timeline,
                            &self.me(),
                            &target,
                            &strategy,
                            Some(&self.database_module),
                        ))))
                    } else {
                        Ok(module_msg.then(TopperResponse::error("No target.".into())))
                    }
                }
                _ => Ok(module_msg),
            },
            _ => Ok(module_msg),
        }
    }
}
