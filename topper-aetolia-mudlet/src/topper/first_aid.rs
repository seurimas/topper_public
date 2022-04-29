use super::battle_stats::BattleStats;
use super::db::AetMudletDatabaseModule;
use super::BattleModule;
use topper_aetolia::db::AetDatabaseModule;
use topper_aetolia::timeline::AetTimeline;
use topper_aetolia::types::*;
use topper_aetolia::{classes::*, curatives::*, timeline::AetTimeSlice};
use topper_core_mudlet::topper::{TopperMessage, TopperModule, TopperResponse};

#[derive(Debug, Default)]
pub struct FirstAidModule {
    in_flight: FirstAid,
}
