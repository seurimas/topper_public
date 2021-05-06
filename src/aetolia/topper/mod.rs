use crate::aetolia::classes::get_attack;
use crate::aetolia::timeline::{AetObservation, AetPrompt, AetTimeSlice, AetTimeline};
use crate::aetolia::types::AgentState;
use crate::timeline::BaseTimeline;
use crate::topper::{
    DatabaseModule, TelnetModule, TimelineModule, Topper, TopperCore, TopperHandler, TopperMessage,
    TopperModule, TopperRequest, TopperResponse, WebModule,
};
use battle_stats::BattleStatsModule;
use serde_json::from_str;
use std::sync::mpsc::Sender;
pub mod battle_stats;
pub mod curing;
use crate::aetolia::topper::curing::prioritize_cures;

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

#[derive(Default)]
pub struct BattleModule;

impl<'s> TopperModule<'s> for BattleModule {
    type Siblings = (
        &'s String,
        &'s Option<String>,
        &'s AetTimeline,
        &'s DatabaseModule,
    );
    fn handle_message(
        &mut self,
        message: &TopperMessage,
        (me, target, timeline, db): Self::Siblings,
    ) -> Result<TopperResponse, String> {
        match message {
            TopperMessage::Request(request) => match request {
                TopperRequest::Attack(strategy) => {
                    if let Some(target) = target {
                        Ok(TopperResponse::qeb(get_attack(
                            &timeline,
                            me,
                            &target,
                            &strategy,
                            Some(db),
                        )))
                    } else {
                        Ok(TopperResponse::error("No target.".into()))
                    }
                }
                _ => Ok(TopperResponse::silent()),
            },
            _ => Ok(prioritize_cures(&timeline, me, &target, db)),
        }
    }
}

pub type AetTopper = Topper<AetObservation, AetPrompt, AgentState, BattleModule>;

impl AetTopper {
    pub fn new(send_lines: Sender<String>, path: String) -> Self {
        println!("DB: {:?}", std::fs::canonicalize(path.clone()).unwrap());
        Topper {
            timeline_module: AetTimelineModule::new(),
            core_module: TopperCore::new(),
            telnet_module: TelnetModule::new(send_lines),
            battle_module: BattleModule::default(),
            battlestats_module: BattleStatsModule::new(),
            database_module: DatabaseModule::new(path),
            web_module: WebModule::new(),
        }
    }
}

impl TopperHandler for AetTopper {
    type Message = TopperMessage;

    fn from_str(&self, line: &String) -> Result<TopperMessage, String> {
        from_str(line).map_err(|error| error.to_string())
    }

    fn handle_request_or_event(
        &mut self,
        topper_msg: &TopperMessage,
    ) -> Result<TopperResponse, String> {
        Ok(self
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
            .then(
                self.database_module
                    .handle_message(&topper_msg, (self.timeline_module.timeline.who_am_i()))?,
            )
            .then(self.web_module.handle_message(&topper_msg, ())?)
            .then(self.battle_module.handle_message(
                &topper_msg,
                (
                    &self.me(),
                    &self.core_module.target,
                    &self.timeline_module.timeline,
                    &self.database_module,
                ),
            )?))
    }
}
