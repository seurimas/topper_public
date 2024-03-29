use super::battle_stats::BattleStats;
use super::db::AetMudletDatabaseModule;
use super::BattleModule;
use topper_aetolia::db::AetDatabaseModule;
use topper_aetolia::timeline::AetTimeline;
use topper_aetolia::types::*;
use topper_aetolia::{classes::*, timeline::AetTimeSlice};
use topper_core_mudlet::topper::{TopperMessage, TopperModule, TopperResponse};

#[derive(Debug, Default)]
pub struct PredictionModule {
    prediction: String,
    removing: bool,
}
impl<'s> TopperModule<'s, AetTimeSlice, BattleStats> for PredictionModule {
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
        Ok(prioritize_cures(self, &timeline, me, &target, db))
    }
}

fn guess_aff(timeline: &AetTimeline, aff: FType) -> String {
    match aff {
        FType::Anorexia | FType::Hypersomnia => format!("eat kawhe"),
        // FType::Impairment => format!("chameleon {}", timeline.who_am_i()),
        // FType::Paranoia => format!("unenemy {}", timeline.who_am_i()),
        FType::Paresis => {
            format!("touch tree")
        }
        FType::Asthma => {
            let me = timeline.state.borrow_me();
            if me.balanced(BType::Pill) && me.is(FType::Aeon) {
                format!("eat decongestant;;smoke willow")
            } else {
                format!("smoke reishi")
            }
        }
        FType::Weariness => {
            if timeline.state.borrow_me().get_qeb_balance() <= 0.0 {
                format!("firstaid predict {}", aff.to_name())
            } else {
                format!("dash out")
            }
        }
        FType::Impatience => format!("meditate;;wake"),
        FType::Superstition => format!("point icewall"),
        _ => format!("firstaid predict {}", aff.to_name()),
    }
}

pub fn get_guessed_value(
    me: &AgentState,
    opponent_class: &Option<Class>,
    hidden_aff: FType,
) -> usize {
    match hidden_aff {
        FType::Impatience => 11,
        FType::Anorexia => 11,
        FType::Asthma => 10,
        FType::Faintness => 10,
        FType::Lethargy => 9,
        FType::Dizziness => 8,
        FType::Clumsiness => 8,
        FType::Hypersomnia => 7,
        FType::Stupidity => 6,
        FType::Paresis => 5,
        FType::Weariness => 4,
        FType::Recklessness => 3,
        _ => 0,
    }
}

pub fn get_best_guess(me: &AgentState, opponent_class: &Option<Class>) -> Option<(usize, FType)> {
    let mut best_guess_with_score = None;
    for hidden_aff in me.hidden_state.iter_guesses() {
        let aff_score = get_guessed_value(me, opponent_class, *hidden_aff);
        best_guess_with_score = best_guess_with_score.map_or(
            Some((aff_score, *hidden_aff)),
            |(best_score, best_guess)| {
                if best_score < aff_score {
                    Some((aff_score, *hidden_aff))
                } else {
                    Some((best_score, best_guess))
                }
            },
        );
    }
    best_guess_with_score.filter(|(score, _guess)| *score > 0)
}

pub fn get_guesses(
    prediction_module: &mut PredictionModule,
    timeline: &AetTimeline,
    me: &String,
    foe: &Option<String>,
    db: &AetMudletDatabaseModule,
) -> Option<String> {
    let mut best_guess_with_score = None;
    let mut max_guesses = 0;
    let opponent_class = foe.as_ref().map(|you| db.get_class(&you)).flatten();
    if let Some(me) = timeline.state.get_agent(me) {
        for my_state in me {
            if my_state.hidden_state.guesses() == 1
                && my_state.get_balance(BType::Tree) < 0.5
                && my_state.can_tree(true)
            {
                // Don't bother to guess if we're just going to tree it away.
                continue;
            }
            if let Some((guess_score, guess)) = get_best_guess(my_state, &opponent_class) {
                if let Some((best_guess_score, best_guess)) = best_guess_with_score {
                    if best_guess_score < guess_score {
                        best_guess_with_score = Some((guess_score, guess));
                    }
                } else {
                    best_guess_with_score = Some((guess_score, guess));
                }
            }
            if my_state.hidden_state.guesses() > max_guesses {
                max_guesses = my_state.hidden_state.guesses();
            }
        }
    }
    if best_guess_with_score.is_none() && max_guesses == 0 {
        return Some("".to_string());
    }
    best_guess_with_score.map(|(_score, guess)| guess_aff(timeline, guess))
}

pub fn prioritize_cures(
    prediction_module: &mut PredictionModule,
    timeline: &AetTimeline,
    me: &String,
    foe: &Option<String>,
    db: &AetMudletDatabaseModule,
) -> TopperResponse<BattleStats> {
    if let Some(guesses) = get_guesses(prediction_module, timeline, me, foe, db) {
        if !guesses.eq(&prediction_module.prediction) {
            prediction_module.prediction = guesses.clone();
            if guesses.len() > 0 {
                return TopperResponse::passive("predict".to_string(), guesses);
            }
        }
    }
    let me = timeline.state.borrow_me();
    if me.bard_board.iron_collar_state == IronCollarState::Locking && !me.is(FType::Perplexed) {
        if !prediction_module.removing {
            prediction_module.removing = true;
            return TopperResponse::passive(
                "defend".to_string(),
                "remove collar;;remove collar".to_string(),
            );
        }
    } else {
        prediction_module.removing = false;
    }
    TopperResponse::silent()
}
