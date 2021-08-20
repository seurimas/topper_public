use crate::aetolia::timeline::*;
use crate::aetolia::types::*;

lazy_static! {
    pub static ref SUN_AFFS: Vec<FType> = vec![
        FType::Paresis,
        FType::Clumsiness,
        FType::Asthma,
        FType::Vomiting,
        FType::Lethargy,
        FType::Sensitivity,
        FType::Superstition,
        FType::Hypersomnia,
    ];
    pub static ref MOON_AFFS: Vec<FType> = vec![
        FType::Stupidity,
        FType::Confusion,
        FType::Recklessness,
        FType::Impatience,
        FType::Epilepsy,
        FType::Berserking,
        FType::Weariness,
        FType::Anorexia,
    ];
}

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut AetTimelineState,
    _before: &Vec<AetObservation>,
    after: &Vec<AetObservation>,
) -> Result<(), String> {
    match combat_action.skill.as_ref() {
        "Sun" | "Moon" => {
            if !combat_action.annotation.eq("dodge") {
                let observations = after.clone();
                let perspective = agent_states.get_perspective(&combat_action);
                let sun = combat_action.skill.eq("Sun");
                for_agent_uncertain_closure(
                    agent_states,
                    &combat_action.target,
                    Box::new(move |you| {
                        apply_or_infer_random_afflictions(
                            you,
                            &observations,
                            perspective,
                            Some((
                                1,
                                (if sun {
                                    SUN_AFFS.iter()
                                } else {
                                    MOON_AFFS.iter()
                                })
                                .filter(|aff| !you.is(**aff))
                                .map(|aff| *aff)
                                .collect(),
                            )),
                        )
                    }),
                );
            }
        }
        _ => {}
    }
    Ok(())
}
