use crate::aetolia::timeline::*;
use crate::aetolia::types::*;

lazy_static! {
    pub static ref CIRCLE_AFFS: Vec<FType> = vec![
        FType::Merciful,
        FType::Masochism,
        FType::Berserking,
        FType::Recklessness,
    ];
    pub static ref SQUARE_AFFS: Vec<FType> = vec![
        FType::Dizziness,
        FType::Faintness,
        FType::Epilepsy,
        FType::Shyness,
    ];
    pub static ref TRIANGLE_AFFS: Vec<FType> = vec![
        FType::Laxity,
        FType::LoversEffect,
        FType::Peace,
        FType::Magnanimity,
    ];
}

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut AetTimelineState,
    _before: &Vec<AetObservation>,
    after: &Vec<AetObservation>,
) -> Result<(), String> {
    match combat_action.skill.as_ref() {
        "Circle" => {}
        _ => {}
    }
    Ok(())
}
