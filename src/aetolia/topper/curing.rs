use super::BattleModule;
use crate::aetolia::timeline::AetTimeline;
use crate::aetolia::types::*;
use crate::topper::{DatabaseModule, TopperResponse};

#[derive(Debug, Default)]
pub struct CureModule {
    prediction: String,
}

pub fn get_guesses(
    battle_module: &mut BattleModule,
    timeline: &AetTimeline,
    me: &String,
    foe: &Option<String>,
    db: &DatabaseModule,
) -> Option<String> {
    let mut guess = None;
    let mut max_guesses = 0;
    if let Some(me) = timeline.state.get_agent(me) {
        for my_state in me {
            if !my_state.is(FType::Speed)
                && my_state.is(FType::Asthma)
                && my_state.hidden_state.guessed(FType::Asthma)
            {
                guess = Some("firstaid predict asthma".to_string());
            }
            if my_state.hidden_state.guesses() > max_guesses {
                max_guesses = my_state.hidden_state.guesses();
            }
        }
    }
    if guess.is_none() && max_guesses == 0 {
        return Some("".to_string());
    }
    guess
}

pub fn prioritize_cures(
    battle_module: &mut BattleModule,
    timeline: &AetTimeline,
    me: &String,
    foe: &Option<String>,
    db: &DatabaseModule,
) -> TopperResponse {
    if let Some(guesses) = get_guesses(battle_module, timeline, me, foe, db) {
        if !guesses.eq(&battle_module.0.prediction) {
            battle_module.0.prediction = guesses.clone();
            if guesses.len() > 0 {
                return TopperResponse::passive("cure".to_string(), guesses);
            }
        }
    }
    TopperResponse::silent()
}
