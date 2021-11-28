use crate::timeline::*;
use crate::types::*;

lazy_static! {
    pub static ref SPIRITWRACK_AFFS: Vec<FType> = vec![
        FType::Anorexia,
        FType::Stupidity,
        FType::Impatience,
        FType::Vertigo,
        FType::Sensitivity,
        FType::SelfPity,
        FType::Berserking,
        FType::Migraine,
    ];
    pub static ref CHASTEN_AFFS: Vec<FType> = vec![
        FType::Anorexia,
        FType::Dementia,
        FType::Hypochondria,
        FType::Lethargy,
        FType::Loneliness,
        FType::Masochism,
        FType::Paranoia,
        FType::Recklessness,
        FType::Stupidity,
        FType::Agony,
    ];
}

const CRUSH_DAMAGE: f32 = 12.5;
const SMASH_DAMAGE: f32 = 25.0;
pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut AetTimelineState,
    _before: &Vec<AetObservation>,
    after: &Vec<AetObservation>,
) -> Result<(), String> {
    match combat_action.skill.as_ref() {
        "Aura" => {
            let observations = after.clone();
            for_agent_closure(
                agent_states,
                &combat_action.caster,
                Box::new(move |me| {
                    apply_or_infer_balance(me, (BType::Equil, 3.0), &observations);
                }),
            );
            let mut affected = if combat_action.target == "" {
                &combat_action.caster
            } else {
                &combat_action.target
            };
            for_agent_closure(
                agent_states,
                affected,
                Box::new(move |you| {
                    you.set_flag(FType::Shielded, true);
                }),
            );
        }
        "Smash" => {
            if let Ok(limb) = get_limb_damage(&combat_action.annotation) {
                attack_limb_damage(
                    agent_states,
                    &combat_action.target,
                    (limb, SMASH_DAMAGE, true),
                    after,
                );
            };
        }
        "Crush" => {
            if let Ok(limb) = get_limb_damage(&combat_action.annotation) {
                attack_limb_damage(
                    agent_states,
                    &combat_action.target,
                    (limb, CRUSH_DAMAGE, true),
                    after,
                );
            };
        }
        "Spiritwrack" => {
            if combat_action.annotation.eq("fire") {
                let observations = after.clone();
                let perspective = agent_states.get_perspective(&combat_action);
                if perspective != Perspective::Bystander {
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
                                    SPIRITWRACK_AFFS
                                        .iter()
                                        .filter(|aff| !you.is(**aff))
                                        .map(|aff| *aff)
                                        .collect(),
                                )),
                            )
                        }),
                    );
                }
            }
        }
        "Chasten" => {
            let observations = after.clone();
            let perspective = agent_states.get_perspective(&combat_action);
            if perspective != Perspective::Bystander {
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
                                CHASTEN_AFFS
                                    .iter()
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
