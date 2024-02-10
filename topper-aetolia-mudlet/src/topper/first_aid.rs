use super::battle_stats::BattleStats;
use super::db::AetMudletDatabaseModule;
use super::BattleModule;
use topper_aetolia::db::AetDatabaseModule;
use topper_aetolia::timeline::AetTimeline;
use topper_aetolia::types::*;
use topper_aetolia::{classes::*, curatives::*, timeline::AetTimeSlice};
use topper_core_mudlet::topper::*;

#[derive(Debug, Default)]
pub struct FirstAidModule {
    active: FirstAid,
    in_flight: FirstAid,
}

impl<'s> TopperModule<'s, AetTimeSlice, BattleStats> for FirstAidModule {
    type Siblings = (&'s mut AetTimeline, &'s AetMudletDatabaseModule);

    fn handle_message(
        &mut self,
        message: &TopperMessage<AetTimeSlice>,
        siblings: Self::Siblings,
    ) -> Result<TopperResponse<BattleStats>, String> {
        match message {
            TopperMessage::Request(TopperRequest::ModuleMsg(module, command)) => {
                if module.eq("firstaid") {
                    if command.eq("check") {
                        println!("FirstAidModule: check {:?}", self);
                    }
                }
                Ok(TopperResponse::silent())
            }
            _ => Ok(TopperResponse::silent()),
        }
    }
}
