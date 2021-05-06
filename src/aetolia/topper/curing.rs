use crate::aetolia::timeline::AetTimeline;
use crate::topper::{DatabaseModule, TopperResponse};

pub fn prioritize_cures(
    timeline: &AetTimeline,
    me: &String,
    foe: &Option<String>,
    db: &DatabaseModule,
) -> TopperResponse {
    TopperResponse::silent()
}
