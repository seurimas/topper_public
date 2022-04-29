use battle_stats::BattleStatsModule;
use group::GroupModule;
use prediction::PredictionModule;
use serde_json::from_str;
use std::sync::mpsc::Sender;
use topper_aetolia::classes::get_attack;
use topper_aetolia::timeline::*;
use topper_aetolia::types::AgentState;
use topper_core::observations;
use topper_core::observations::{ObservationParser, BENCHMARKS};
use topper_core::timeline::BaseTimeline;
use topper_core_mudlet::topper::{
    TelnetModule, TimelineModule, Topper, TopperCore, TopperHandler, TopperMessage, TopperModule,
    TopperRequest, TopperResponse,
};
pub mod battle_stats;
pub mod db;
pub mod first_aid;
pub mod group;
pub mod prediction;
pub mod web_ui;
use crate::topper::prediction::prioritize_cures;

use self::battle_stats::BattleStats;
use self::db::AetMudletDatabaseModule;
use self::web_ui::WebModule;

pub type AetTimelineModule = TimelineModule<AetObservation, AetPrompt, AgentState>;

impl<'s> TopperModule<'s, AetTimeSlice, BattleStats> for AetTimelineModule {
    type Siblings = (&'s AetMudletDatabaseModule,);
    fn handle_message(
        &mut self,
        message: &TopperMessage<AetTimeSlice>,
        siblings: Self::Siblings,
    ) -> Result<TopperResponse<BattleStats>, String> {
        match message {
            TopperMessage::TimeSlice(timeslice) => {
                self.timeline
                    .push_time_slice(timeslice.clone(), Some(siblings.0))?;
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

impl<'s> TopperModule<'s, AetTimeSlice, BattleStats> for BattleModule {
    type Siblings = (
        &'s String,
        &'s Option<String>,
        &'s AetTimeline,
        &'s AetMudletDatabaseModule,
    );
    fn handle_message(
        &mut self,
        message: &TopperMessage<AetTimeSlice>,
        (me, target, timeline, db): Self::Siblings,
    ) -> Result<TopperResponse<BattleStats>, String> {
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
            _ => Ok(TopperResponse::silent()),
        }
    }
}

pub struct AetTopper {
    pub debug_mode: bool,
    triggers_dir: String,
    pub timeline_module: AetTimelineModule,
    pub core_module: TopperCore,
    pub telnet_module: TelnetModule,
    pub battle_module: BattleModule,
    pub prediction_module: PredictionModule,
    pub group_module: GroupModule,
    pub battlestats_module: BattleStatsModule,
    pub database_module: AetMudletDatabaseModule,
    pub web_module: WebModule,
    pub observation_parser: ObservationParser<AetObservation>,
}

impl Topper<AetObservation, AetPrompt, AgentState, AetMudletDatabaseModule> for AetTopper {
    fn get_timeline_module(&self) -> &AetTimelineModule {
        &self.timeline_module
    }
    fn get_mut_timeline_module(&mut self) -> &mut AetTimelineModule {
        &mut self.timeline_module
    }
    fn get_core_module(&self) -> &TopperCore {
        &self.core_module
    }
    fn get_database(&mut self) -> &mut AetMudletDatabaseModule {
        &mut self.database_module
    }
}

impl AetTopper {
    pub fn new(send_lines: Sender<String>, path: String, triggers_dir: String) -> Self {
        println!("DB: {:?}", std::fs::canonicalize(path.clone()).unwrap());
        let database_module = AetMudletDatabaseModule::new(path);
        AetTopper {
            debug_mode: false,
            triggers_dir: triggers_dir.clone(),
            timeline_module: AetTimelineModule::new(),
            core_module: TopperCore::new(),
            telnet_module: TelnetModule::new(send_lines),
            battle_module: BattleModule::default(),
            prediction_module: PredictionModule::default(),
            group_module: GroupModule::new(&database_module),
            battlestats_module: BattleStatsModule::new(),
            database_module,
            web_module: WebModule::new(),
            observation_parser: ObservationParser::<AetObservation>::new_from_directory(
                triggers_dir,
                aet_observation_creator,
            )
            .unwrap(),
        }
    }
}

impl TopperHandler<BattleStats> for AetTopper {
    type Message = TopperMessage<AetTimeSlice>;

    fn from_str(&self, line: &String) -> Result<TopperMessage<AetTimeSlice>, String> {
        from_str(line).map_err(|error| error.to_string())
    }

    fn handle_request_or_event(
        &mut self,
        topper_msg: &mut TopperMessage<AetTimeSlice>,
    ) -> Result<TopperResponse<BattleStats>, String> {
        match topper_msg {
            TopperMessage::TimeSlice(slice) => {
                let mut new_observations = self.observation_parser.observe(&slice);
                if self.debug_mode {
                    println!("{:?}", new_observations);
                }
                slice
                    .observations
                    .get_or_insert(Vec::new())
                    .append(&mut new_observations);
            }
            TopperMessage::Request(TopperRequest::ModuleMsg(module, command)) => {
                if "core".eq(module) && "debug".eq(command) {
                    self.debug_mode = !self.debug_mode;
                    if self.debug_mode {
                        println!("Debug mode on!");
                    } else {
                        println!("Debug mode off!");
                    }
                } else if "core".eq(module) && "reload triggers".eq(command) {
                    println!("Reloading triggers");
                    self.observation_parser =
                        ObservationParser::<AetObservation>::new_from_directory(
                            self.triggers_dir.clone(),
                            aet_observation_creator,
                        )
                        .map_err(|err| err.to_string())?;
                }
            }
            _ => {}
        }
        Ok(self
            .core_module
            .handle_message(&topper_msg, ())?
            .then(
                self.timeline_module
                    .handle_message(&topper_msg, (&self.database_module,))?,
            )
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
            .then(self.group_module.handle_message(
                &topper_msg,
                (
                    &self.me(),
                    &self.timeline_module.timeline,
                    &self.database_module,
                ),
            )?)
            .then(self.web_module.handle_message(&topper_msg, ())?)
            .then(self.battle_module.handle_message(
                &topper_msg,
                (
                    &self.me(),
                    &self.core_module.target,
                    &self.timeline_module.timeline,
                    &self.database_module,
                ),
            )?)
            .then(self.prediction_module.handle_message(
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
