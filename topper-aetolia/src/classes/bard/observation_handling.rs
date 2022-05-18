use crate::{curatives::RANDOM_CURES, observables::*, timeline::*, types::*};

const BLADES_COUNT: usize = 3;
// All values assume onyx.
const RUNEBAND_DITHER: usize = 2;
const GLOBES_DITHER: usize = 0;
const BARBS_DITHER: usize = 2;
const BLADESTORM_DITHER: usize = 2;

const GLOBE_AFFS: [FType; 3] = [FType::Dizziness, FType::Confusion, FType::Perplexed];
const RUNEBAND_AFFS: [FType; 7] = [
    FType::Stupidity,
    FType::Paranoia,
    FType::RingingEars,
    FType::Loneliness,
    FType::Exhausted,
    FType::Laxity,
    FType::Clumsiness,
];

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut AetTimelineState,
    _before: &Vec<AetObservation>,
    after: &Vec<AetObservation>,
) -> Result<(), String> {
    let observations = after.clone();
    let first_person = combat_action.caster.eq(&agent_states.me);
    let hints = agent_states.get_player_hint(&combat_action.caster, &"CALLED_VENOMS".to_string());
    match combat_action.skill.as_ref() {
        "Runeband" => {
            for_agent_closure(
                agent_states,
                &combat_action.caster,
                Box::new(move |me| {
                    me.assume_bard(|bard| {
                        bard.dithering = RUNEBAND_DITHER;
                    });
                    if !me.is(FType::Destiny) {
                        apply_or_infer_balance(me, (BType::Equil, 2.0), &observations);
                    } else {
                        me.set_flag(FType::Destiny, false);
                    }
                }),
            );
            for_agent(agent_states, &combat_action.target, |me| {
                me.bard_board.runeband_state = RunebandState::initial();
            });
        }
        "Runebanded" => {
            for_agent(agent_states, &combat_action.caster, |me| {
                if let Some(aff) = me.bard_board.runebanded(&RUNEBAND_AFFS) {
                    me.set_flag(aff, true);
                };
            });
        }
        "Globes" => {
            for_agent_closure(
                agent_states,
                &combat_action.caster,
                Box::new(move |me| {
                    me.assume_bard(|bard| {
                        bard.dithering = RUNEBAND_DITHER;
                    });
                    if !me.is(FType::Destiny) {
                        apply_or_infer_balance(me, (BType::Equil, 2.0), &observations);
                    } else {
                        me.set_flag(FType::Destiny, false);
                    }
                }),
            );
            for_agent(agent_states, &combat_action.target, |me| {
                me.bard_board.globes_state = GlobesState::initial();
            });
        }
        "Globed" => {
            if combat_action.annotation.eq("final") {
                for_agent(agent_states, &combat_action.caster, |me| {
                    me.bard_board.globes_state = GlobesState::Floating(1);
                    if let Some(aff) = me.bard_board.globed(&GLOBE_AFFS) {
                        me.set_flag(aff, true);
                    };
                });
            } else if combat_action.annotation.eq("all") {
                for_agent(agent_states, &combat_action.caster, |me| {
                    while let Some(aff) = me.bard_board.globed(&GLOBE_AFFS) {
                        me.set_flag(aff, true);
                    }
                });
            } else {
                for_agent(agent_states, &combat_action.caster, |me| {
                    if let Some(aff) = me.bard_board.globed(&GLOBE_AFFS) {
                        me.set_flag(aff, true);
                    };
                });
            }
        }
        "Crackshot" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Dizziness, FType::Perplexed, FType::Stun],
                after,
            );
            for_agent_closure(
                agent_states,
                &combat_action.target,
                Box::new(move |me| {
                    apply_or_infer_balance(me, (BType::Balance, 2.8), &observations);
                }),
            );
        }
        "Hiltblow" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Vomiting, FType::Misery],
                after,
            );
            for_agent_closure(
                agent_states,
                &combat_action.target,
                Box::new(move |me| {
                    apply_or_infer_balance(me, (BType::Balance, 2.8), &observations);
                }),
            );
        }
        "Tempo" | "Harry" | "Bravado" => {
            for_agent_closure(
                agent_states,
                &combat_action.caster,
                Box::new(move |me| {
                    apply_or_infer_balance(me, (BType::Balance, 2.65), &observations);
                }),
            );
            apply_weapon_hits(
                agent_states,
                &combat_action.caster,
                &combat_action.target,
                after,
                first_person,
                &hints,
            );
            if combat_action.skill.eq("Tempo") {
                for_agent(agent_states, &combat_action.caster, |me| {
                    me.assume_bard(|bard| {
                        bard.on_tempo();
                    });
                });
            } else if combat_action.skill.eq("Bravado") {
                let observations = after.clone();
                let perspective = agent_states.get_perspective(&combat_action);
                for_agent_closure(
                    agent_states,
                    &combat_action.caster,
                    Box::new(move |me| {
                        apply_or_strike_random_cure(
                            me,
                            &observations,
                            perspective,
                            (1, RANDOM_CURES.to_vec()),
                        );
                        apply_or_infer_balance(me, (BType::ClassCure1, 15.0), &observations);
                    }),
                );
            }
        }
        "Bladestorm" => {
            for_agent_closure(
                agent_states,
                &combat_action.caster,
                Box::new(move |me| {
                    me.assume_bard(|bard| {
                        bard.dithering = BLADESTORM_DITHER;
                    });
                    if !me.is(FType::Destiny) {
                        apply_or_infer_balance(me, (BType::Equil, 2.0), &observations);
                    } else {
                        me.set_flag(FType::Destiny, false);
                    }
                }),
            );
            for_agent(agent_states, &combat_action.target, |me| {
                me.bard_board.blades_count = BLADES_COUNT;
            });
        }
        "Bladestormed" => {
            let final_blade = combat_action.annotation.eq("final");
            for_agent_closure(
                agent_states,
                &combat_action.caster,
                Box::new(move |me| {
                    for observation in observations.iter() {
                        if let AetObservation::Devenoms(venom) = observation {
                            apply_venom(me, &venom, false);
                        }
                    }
                    if final_blade {
                        me.bard_board.blades_count = 0;
                    } else if me.bard_board.blades_count > 0 {
                        me.bard_board.blades_count -= 1;
                    }
                }),
            );
        }
        "TempoEnd" => {
            for_agent(agent_states, &combat_action.caster, |me| {
                me.assume_bard(|bard| {
                    bard.off_tempo();
                });
            });
        }
        "HalfbeatStart" => {
            for_agent(agent_states, &combat_action.caster, |me| {
                me.assume_bard(|bard| {
                    bard.half_beat_pickup();
                });
            });
        }
        "HalfbeatEnd" => {
            for_agent(agent_states, &combat_action.caster, |me| {
                me.assume_bard(|bard| {
                    bard.half_beat_slowdown();
                });
            });
        }
        "Needle" => {
            for_agent_closure(
                agent_states,
                &combat_action.caster,
                Box::new(move |me| {
                    apply_or_infer_balance(me, (BType::Balance, 1.0), &observations);
                }),
            );
            let venom = combat_action.annotation.clone();
            if venom.eq("dodge") {
                for_agent(agent_states, &combat_action.target, |me| {
                    me.dodge_state.register_dodge();
                });
            } else {
                for_agent_closure(
                    agent_states,
                    &combat_action.target,
                    Box::new(move |me| {
                        me.bard_board.needle_with(&venom);
                    }),
                );
            }
        }
        "Needled" => {
            for_agent_closure(
                agent_states,
                &combat_action.caster,
                Box::new(move |me| {
                    if let Some(venom) = me.bard_board.needled() {
                        apply_venom(me, &venom, false);
                    }
                }),
            );
        }
        _ => {}
    }
    Ok(())
}
