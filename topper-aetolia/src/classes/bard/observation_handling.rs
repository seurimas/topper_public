use crate::{
    classes::remove_through, curatives::RANDOM_CURES, non_agent::AetTimelineRoomExt,
    observables::*, timeline::*, types::*,
};

const BLADES_COUNT: usize = 3;
// All values assume onyx.
const RUNEBAND_DITHER: usize = 2;
const MANABARBS_DITHER: usize = 2;
const GLOBES_DITHER: usize = 0;
const BARBS_DITHER: usize = 2;
const BLADESTORM_DITHER: usize = 2;
const ANELACE_DITHER: usize = 2;
const NULLSTONE_DITHER: usize = 1;
const BOUNDARY_DITHER: usize = 3;
const IRONCOLLAR_DITHER: usize = 2;

pub const NULLSTONE: &str = "a stone of annulment";
pub const ANELACE: &str = "a sharp anelace";

pub const GLOBE_AFFS: [FType; 3] = [FType::Dizziness, FType::Confusion, FType::Perplexed];
const RUNEBAND_AFFS: [FType; 7] = [
    FType::Stupidity,
    FType::Paranoia,
    FType::RingingEars,
    FType::Loneliness,
    FType::Exhausted,
    FType::Laxity,
    FType::Clumsiness,
];

lazy_static! {
    static ref PIERCE_ORDER: Vec<FType> = vec![
        FType::Reflection,
        FType::Shielded,
        FType::Rebounding,
        FType::Speed,
    ];
}

pub fn handle_weaving_action(
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
            for_agent(
                agent_states,
                &combat_action.caster,
                &move |me: &mut AgentState| {
                    me.assume_bard(&|bard: &mut BardClassState| {
                        bard.dithering = RUNEBAND_DITHER;
                    });
                    use_destiny_eq(me, &observations);
                },
            );
            for_agent(
                agent_states,
                &combat_action.target,
                &|me: &mut AgentState| {
                    me.bard_board.runeband_state = RunebandState::initial();
                },
            );
        }
        "Runebanded" => {
            for_agent(
                agent_states,
                &combat_action.caster,
                &|me: &mut AgentState| {
                    if let Some(aff) = me.bard_board.runebanded(&RUNEBAND_AFFS) {
                        me.set_flag(aff, true);
                    };
                },
            );
        }
        "Globes" => {
            for_agent(
                agent_states,
                &combat_action.caster,
                &move |me: &mut AgentState| {
                    me.assume_bard(&|bard: &mut BardClassState| {
                        bard.dithering = GLOBES_DITHER;
                    });
                    use_destiny_eq(me, &observations);
                },
            );
            for_agent(
                agent_states,
                &combat_action.target,
                &|me: &mut AgentState| {
                    me.bard_board.globes_state = GlobesState::initial();
                },
            );
        }
        "Globed" => {
            if combat_action.annotation.eq("final") {
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &|me: &mut AgentState| {
                        me.bard_board.globes_state = GlobesState::Floating(1);
                        if let Some(aff) = me.bard_board.globed(&GLOBE_AFFS) {
                            me.set_flag(aff, true);
                        };
                    },
                );
            } else if combat_action.annotation.eq("all") {
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &|me: &mut AgentState| {
                        while let Some(aff) = me.bard_board.globed(&GLOBE_AFFS) {
                            me.set_flag(aff, true);
                        }
                    },
                );
            } else {
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &|me: &mut AgentState| {
                        if let Some(aff) = me.bard_board.globed(&GLOBE_AFFS) {
                            me.set_flag(aff, true);
                        };
                    },
                );
            }
        }
        "Bladestorm" => {
            for_agent(
                agent_states,
                &combat_action.caster,
                &move |me: &mut AgentState| {
                    me.assume_bard(&|bard: &mut BardClassState| {
                        bard.dithering = BLADESTORM_DITHER;
                    });
                    use_destiny_eq(me, &observations);
                },
            );
            for_agent(
                agent_states,
                &combat_action.target,
                &|me: &mut AgentState| {
                    me.bard_board.blades_count = BLADES_COUNT;
                },
            );
        }
        "Barbs" => {
            for_agent(
                agent_states,
                &combat_action.caster,
                &move |me: &mut AgentState| {
                    me.assume_bard(&|bard: &mut BardClassState| {
                        bard.dithering = BLADESTORM_DITHER;
                    });
                },
            );
            for_agent(
                agent_states,
                &combat_action.target,
                &|me: &mut AgentState| {
                    me.set_flag(FType::Manabarbs, true);
                },
            );
        }
        "Bladestormed" => {
            let final_blade = combat_action.annotation.eq("final");
            for_agent(
                agent_states,
                &combat_action.caster,
                &move |me: &mut AgentState| {
                    for observation in observations.iter() {
                        if let AetObservation::Devenoms(venom) = observation {
                            apply_venom(me, &venom, false);
                        }
                    }
                    me.bard_board.runeband_state.reverse();
                    if final_blade {
                        me.bard_board.blades_count = 0;
                    } else if me.bard_board.blades_count > 0 {
                        me.bard_board.blades_count -= 1;
                    }
                },
            );
        }
        "Ironcollar" => {
            for_agent(
                agent_states,
                &combat_action.caster,
                &move |me: &mut AgentState| {
                    me.assume_bard(&|bard: &mut BardClassState| {
                        bard.dithering = IRONCOLLAR_DITHER;
                    });
                    apply_or_infer_balance(me, (BType::Equil, 3.0), &observations);
                },
            );
            for_agent(
                agent_states,
                &combat_action.target,
                &|me: &mut AgentState| {
                    me.bard_board.iron_collar_state = IronCollarState::Locking;
                },
            );
        }
        "Ironcollared" => {
            if combat_action.annotation.eq("hit") {
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &|me: &mut AgentState| {
                        me.bard_board.iron_collar_state = IronCollarState::Locked;
                    },
                );
            } else if combat_action.annotation.eq("end") || combat_action.annotation.eq("miss") {
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &|me: &mut AgentState| {
                        me.bard_board.iron_collar_state = IronCollarState::None;
                    },
                );
            }
        }
        "Nullstone" => for_agent(
            agent_states,
            &combat_action.caster,
            &|me: &mut AgentState| {
                me.assume_bard(&|bard: &mut BardClassState| {
                    bard.dithering = NULLSTONE_DITHER;
                });
                me.wield_state.weave(NULLSTONE);
            },
        ),
        "Anelace" => {
            let stabbed = combat_action.annotation.eq("stab");
            if stabbed {
                attack_afflictions(
                    agent_states,
                    &combat_action.target,
                    vec![FType::Hollow, FType::Narcolepsy],
                    &observations,
                );
            }
            for_agent(
                agent_states,
                &combat_action.caster,
                &move |me: &mut AgentState| {
                    if stabbed {
                        me.assume_bard(&|bard: &mut BardClassState| {
                            if bard.anelaces > 0 {
                                bard.anelaces -= 1;
                            }
                        });
                        me.wield_state.unweave(ANELACE);
                    } else {
                        me.assume_bard(&|bard: &mut BardClassState| {
                            bard.anelaces += 1;
                            bard.dithering = ANELACE_DITHER;
                        });
                        me.wield_state.weave(ANELACE);
                        use_destiny_eq(me, &observations);
                    }
                },
            );
        }
        "UnweavedHands" | "UnweavedBelt" => {
            let unweaved = combat_action.annotation.clone().to_ascii_lowercase();
            let in_hands = combat_action.skill.eq("UnweavedHands");
            for_agent(
                agent_states,
                &combat_action.caster,
                &|me: &mut AgentState| {
                    match unweaved.as_ref() {
                        ANELACE => {
                            me.assume_bard(&move |bard: &mut BardClassState| {
                                if bard.anelaces > 0 {
                                    bard.anelaces -= 1;
                                }
                            });
                        }
                        _ => {}
                    };
                    if in_hands {
                        me.wield_state.unweave(&unweaved);
                    }
                },
            );
        }
        "Boundary" => {
            if let Some(my_room) = agent_states.get_my_room_mut() {
                if combat_action.annotation.eq("end") {
                    my_room.remove_tag("boundary");
                } else {
                    my_room.add_tag("boundary");
                }
            }
            if !combat_action.annotation.eq("fail") && !combat_action.annotation.eq("end") {
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &|me: &mut AgentState| {
                        me.assume_bard(&|bard| {
                            bard.dithering = BOUNDARY_DITHER;
                        });
                        use_destiny_eq(me, &observations);
                    },
                );
            }
        }
        _ => {}
    }
    Ok(())
}

fn use_destiny_eq(me: &mut AgentState, observations: &Vec<AetObservation>) {
    if !me.is(FType::Destiny) {
        apply_or_infer_balance(me, (BType::Equil, 2.0), observations);
    } else {
        me.set_flag(FType::Destiny, false);
    }
}

pub fn handle_performance_action(
    combat_action: &CombatAction,
    agent_states: &mut AetTimelineState,
    _before: &Vec<AetObservation>,
    after: &Vec<AetObservation>,
) -> Result<(), String> {
    let observations = after.clone();
    let first_person = combat_action.caster.eq(&agent_states.me);
    let hints = agent_states.get_player_hint(&combat_action.caster, &"CALLED_VENOMS".to_string());
    match combat_action.skill.as_ref() {
        "Pierce" => {
            let annotation = combat_action.annotation.clone();
            for_agent(
                agent_states,
                &combat_action.caster,
                &move |me: &mut AgentState| {
                    apply_or_infer_balance(me, (BType::Balance, 2.0), &observations);
                },
            );
            for_agent(
                agent_states,
                &combat_action.target,
                &move |me: &mut AgentState| {
                    remove_through(
                        me,
                        match annotation.as_ref() {
                            "reflection" => FType::Reflection,
                            "shield" => FType::Shielded,
                            "rebounding" => FType::Rebounding,
                            "speed" => FType::Speed,
                            _ => FType::Speed,
                        },
                        &PIERCE_ORDER.to_vec(),
                    )
                },
            );
        }
        "Crackshot" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Dizziness, FType::Perplexed, FType::Stun],
                after,
            );
            for_agent(
                agent_states,
                &combat_action.caster,
                &move |me: &mut AgentState| {
                    apply_or_infer_balance(me, (BType::Balance, 2.8), &observations);
                },
            );
        }
        "Ridicule" => {
            if combat_action.annotation.eq("hard") {
                attack_afflictions(
                    agent_states,
                    &combat_action.target,
                    vec![FType::Magnanimity],
                    after,
                );
            } else {
                attack_afflictions(
                    agent_states,
                    &combat_action.target,
                    vec![FType::SelfLoathing],
                    after,
                );
            }
            for_agent(
                agent_states,
                &combat_action.caster,
                &move |me: &mut AgentState| {
                    apply_or_infer_balance(me, (BType::Balance, 2.8), &observations);
                },
            );
        }
        "Quip" => {
            if combat_action.annotation.eq("angry") {
                attack_afflictions(
                    agent_states,
                    &combat_action.target,
                    vec![FType::Hatred],
                    after,
                );
            } else {
                attack_afflictions(
                    agent_states,
                    &combat_action.target,
                    vec![FType::Berserking],
                    after,
                );
            }
            for_agent(
                agent_states,
                &combat_action.caster,
                &move |me: &mut AgentState| {
                    apply_or_infer_balance(me, (BType::Balance, 2.8), &observations);
                },
            );
        }
        "Sock" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Dizziness],
                after,
            );
            for_agent(
                agent_states,
                &combat_action.caster,
                &move |me: &mut AgentState| {
                    apply_or_infer_balance(me, (BType::Balance, 2.8), &observations);
                },
            );
        }
        "Hiltblow" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Clumsiness, FType::Misery],
                after,
            );
            for_agent(
                agent_states,
                &combat_action.target,
                &move |me: &mut AgentState| {
                    apply_or_infer_balance(me, (BType::Balance, 2.8), &observations);
                },
            );
        }
        "Tempo" | "Harry" | "Bravado" => {
            for_agent(
                agent_states,
                &combat_action.caster,
                &move |me: &mut AgentState| {
                    apply_or_infer_balance(me, (BType::Balance, 2.65), &observations);
                },
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
                let annotation = combat_action.annotation.clone();
                for_agent(
                    agent_states,
                    &combat_action.target,
                    &move |me: &mut AgentState| {
                        if annotation.eq("two") {
                            me.observe_flag(FType::Paresis, true);
                        } else if annotation.eq("three") {
                            me.observe_flag(FType::Shyness, true);
                        } else if annotation.eq("four") {
                            me.observe_flag(FType::Besilence, true);
                        }
                    },
                );
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &|me: &mut AgentState| {
                        me.assume_bard(&|bard: &mut BardClassState| {
                            bard.on_tempo();
                        });
                    },
                );
            } else if combat_action.skill.eq("Bravado") {
                let observations = after.clone();
                let perspective = agent_states.get_perspective(&combat_action);
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &move |me: &mut AgentState| {
                        apply_or_strike_random_cure(
                            me,
                            &observations,
                            perspective,
                            (1, RANDOM_CURES.to_vec()),
                        );
                        apply_or_infer_balance(me, (BType::ClassCure1, 15.0), &observations);
                    },
                );
            }
        }
        "TempoEnd" => {
            for_agent(
                agent_states,
                &combat_action.caster,
                &|me: &mut AgentState| {
                    me.assume_bard(&|bard: &mut BardClassState| {
                        bard.off_tempo();
                    });
                },
            );
        }
        "Needle" => {
            for_agent(
                agent_states,
                &combat_action.caster,
                &move |me: &mut AgentState| {
                    apply_or_infer_balance(me, (BType::Balance, 1.0), &observations);
                },
            );
            let venom = combat_action.annotation.clone();
            if venom.eq("dodge") {
                for_agent(
                    agent_states,
                    &combat_action.target,
                    &|me: &mut AgentState| {
                        me.dodge_state.register_dodge();
                    },
                );
            } else {
                for_agent(
                    agent_states,
                    &combat_action.target,
                    &move |me: &mut AgentState| {
                        me.bard_board.needle_with(&venom);
                    },
                );
            }
        }
        "Needled" => {
            for_agent(
                agent_states,
                &combat_action.caster,
                &move |me: &mut AgentState| {
                    if let Some(venom) = me.bard_board.needled() {
                        apply_venom(me, &venom, false);
                    }
                },
            );
        }
        _ => {}
    }
    Ok(())
}

pub fn handle_songcalling_action(
    combat_action: &CombatAction,
    agent_states: &mut AetTimelineState,
    _before: &Vec<AetObservation>,
    after: &Vec<AetObservation>,
) -> Result<(), String> {
    let observations = after.clone();
    let first_person = combat_action.caster.eq(&agent_states.me);
    let hints = agent_states.get_player_hint(&combat_action.caster, &"CALLED_VENOMS".to_string());
    match (
        combat_action.skill.as_ref(),
        combat_action.annotation.as_ref(),
    ) {
        ("HalfbeatStart", _) => {
            for_agent(
                agent_states,
                &combat_action.caster,
                &|me: &mut AgentState| {
                    me.assume_bard(&|bard: &mut BardClassState| {
                        bard.half_beat_pickup();
                    });
                },
            );
        }
        ("HalfbeatEnd", _) => {
            for_agent(
                agent_states,
                &combat_action.caster,
                &|me: &mut AgentState| {
                    me.assume_bard(&|bard: &mut BardClassState| {
                        bard.half_beat_slowdown();
                    });
                },
            );
        }
        ("Remembrance", "end") => {
            for_agent(
                agent_states,
                &combat_action.caster,
                &|me: &mut AgentState| {
                    me.assume_bard(&|bard: &mut BardClassState| {
                        bard.dithering = 0;
                        bard.end_song(Song::Remembrance);
                    });
                },
            );
        }
        ("AudienceSong", "end") => {
            for_agent(
                agent_states,
                &combat_action.caster,
                &move |me: &mut AgentState| {
                    me.assume_bard(&|bard: &mut BardClassState| {
                        for song in [
                            Song::Fate,
                            Song::Mythics,
                            Song::Hero,
                            Song::Doom,
                            Song::Sorrow,
                            Song::Unheard,
                            Song::Fascination,
                        ] {
                            bard.end_song(song);
                        }
                    });
                },
            );
        }
        ("Awakening", "hit") => {
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.bard_board.awaken();
            });
        }
        ("Sorrow", "hit") => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Squelched, FType::Migraine],
                after,
            );
        }
        ("Decadence", "hit") => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Addiction],
                after,
            );
        }
        ("Charity", "hit") => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Generosity],
                after,
            );
        }
        (song_name, "end") => {
            if let Some(song) = song_name.parse().ok() {
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &move |me: &mut AgentState| {
                        me.assume_bard(&|bard: &mut BardClassState| {
                            bard.end_song(song);
                        });
                        if song == Song::Destiny {
                            me.set_flag(FType::Destiny, true);
                        }
                    },
                );
            } else {
                println!("No such song found: {}", song_name);
            }
        }
        (song_name, "") => {
            if let Some(song) = song_name.parse().ok() {
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &move |me: &mut AgentState| {
                        me.assume_bard(&|bard: &mut BardClassState| {
                            bard.start_song(song, false);
                        });
                    },
                );
            }
        }
        (song_name, "play") => {
            if let Some(song) = song_name.parse().ok() {
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &move |me: &mut AgentState| {
                        me.assume_bard(&|bard: &mut BardClassState| {
                            bard.start_song(song, true);
                        });
                    },
                );
            }
        }
        _ => {}
    }
    Ok(())
}

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut AetTimelineState,
    before: &Vec<AetObservation>,
    after: &Vec<AetObservation>,
) -> Result<(), String> {
    let observations = after.clone();
    let first_person = combat_action.caster.eq(&agent_states.me);
    let hints = agent_states.get_player_hint(&combat_action.caster, &"CALLED_VENOMS".to_string());
    match combat_action.category.as_ref() {
        "Weaving" => handle_weaving_action(combat_action, agent_states, before, after),
        "Performance" => handle_performance_action(combat_action, agent_states, before, after),
        "Songcalling" => handle_songcalling_action(combat_action, agent_states, before, after),
        _ => Err(format!("Bad category: {}", combat_action.category)),
    }
}
