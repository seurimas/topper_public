use crate::{
    classes::remove_through, curatives::RANDOM_CURES, non_agent::AetTimelineRoomExt,
    observables::*, timeline::*, types::*,
};

const BLADES_COUNT: usize = 3;
// All values assume onyx.
const RUNEBAND_DITHER: usize = 2;
const MANABARBS_DITHER: usize = 2;
const PATCHWORK_DITHER: usize = 2;
const HOROLOGE_DITHER: usize = 2;
const GLOBES_DITHER: usize = 0;
const TEARING_DITHER: usize = 2;
const BARBS_DITHER: usize = 2;
const BLADESTORM_DITHER: usize = 2;
const ANELACE_DITHER: usize = 2;
const HEADSTITCH_DITHER: usize = 1;
const NULLSTONE_DITHER: usize = 1;
const BOUNDARY_DITHER: usize = 3;
const THURIBLE_DITHER: usize = 4;
const HEARTCAGE_DITHER: usize = 5;
const IRONCOLLAR_DITHER: usize = 2;
const IMPETUS_DITHER: usize = 3;
const SWINDLE_DITHER: usize = 4;

pub const NULLSTONE: &str = "a stone of annulment";
pub const HOROLOGE: &str = "a faded hourglass";
pub const THURIBLE: &str = "a golden thurible";
pub const ANELACE: &str = "a sharp anelace";

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
                    if let Some(aff) = me.bard_board.runebanded() {
                        me.set_flag(aff, true);
                    };
                },
            );
        }
        "Patchwork" => {
            for_agent(
                agent_states,
                &combat_action.caster,
                &|me: &mut AgentState| {
                    me.assume_bard(&|bard: &mut BardClassState| {
                        bard.dithering = PATCHWORK_DITHER;
                    });
                },
            );
        }
        "Headstitch" => {
            for_agent(
                agent_states,
                &combat_action.caster,
                &|me: &mut AgentState| {
                    me.assume_bard(&|bard: &mut BardClassState| {
                        bard.dithering = HEADSTITCH_DITHER;
                    });
                },
            );
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Besilence, FType::Deadening],
                after,
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
                        if let Some(aff) = me.bard_board.globed() {
                            me.set_flag(aff, true);
                        };
                    },
                );
            } else if combat_action.annotation.eq("all") {
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &|me: &mut AgentState| {
                        while let Some(aff) = me.bard_board.globed() {
                            me.set_flag(aff, true);
                        }
                    },
                );
            } else {
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &|me: &mut AgentState| {
                        if let Some(aff) = me.bard_board.globed() {
                            me.set_flag(aff, true);
                        };
                    },
                );
            }
        }
        "Deglobed" => {
            for_agent(
                agent_states,
                &combat_action.caster,
                &|me: &mut AgentState| {
                    me.assume_bard(&|mut me| {
                        if me.dithering > 0 {
                            me.dithering -= 1;
                        }
                    });
                },
            );
            for_agent(
                agent_states,
                &combat_action.target,
                &|me: &mut AgentState| {
                    me.bard_board.globed();
                },
            );
        }
        "Deruneband" => {
            for_agent(
                agent_states,
                &combat_action.target,
                &|me: &mut AgentState| {
                    me.bard_board.runeband_state = RunebandState::Inactive;
                },
            );
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
                    me.bard_board.runeband_state.reverse();
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
                    me.set_balance(BType::Manabarbs, 8.);
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
                    me.bard_board.runeband_state.reverse();
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
        "Thurible" => for_agent(
            agent_states,
            &combat_action.caster,
            &|me: &mut AgentState| {
                me.assume_bard(&|bard: &mut BardClassState| {
                    bard.dithering = THURIBLE_DITHER;
                });
                me.wield_state.weave(THURIBLE);
            },
        ),
        "Impetus" => for_agent(
            agent_states,
            &combat_action.caster,
            &|me: &mut AgentState| {
                me.assume_bard(&|bard: &mut BardClassState| {
                    bard.dithering = IMPETUS_DITHER;
                    bard.begin_impetus();
                });
            },
        ),
        "Swindle" => for_agent(
            agent_states,
            &combat_action.caster,
            &|me: &mut AgentState| {
                me.assume_bard(&|bard: &mut BardClassState| {
                    bard.dithering = SWINDLE_DITHER;
                });
            },
        ),
        "Tearing" => for_agent(
            agent_states,
            &combat_action.caster,
            &|me: &mut AgentState| {
                me.assume_bard(&|bard: &mut BardClassState| {
                    bard.dithering = TEARING_DITHER;
                });
            },
        ),
        "Heartcage" => match combat_action.annotation.as_ref() {
            "no_collar" => for_agent(
                agent_states,
                &combat_action.caster,
                &|me: &mut AgentState| {
                    me.bard_board.iron_collar_state = IronCollarState::None;
                },
            ),
            "no_boundary" => {
                if let Some(mut my_room) = agent_states.get_my_room_mut() {
                    my_room.remove_tag("boundary");
                }
            }
            "end" => {}
            "form" => {}
            _ => for_agent(
                agent_states,
                &combat_action.caster,
                &|me: &mut AgentState| {
                    me.assume_bard(&|bard: &mut BardClassState| {
                        bard.dithering = HEARTCAGE_DITHER;
                    });
                },
            ),
        },
        "Horologe" => for_agent(
            agent_states,
            &combat_action.caster,
            &|me: &mut AgentState| {
                me.assume_bard(&|bard: &mut BardClassState| {
                    bard.dithering = HOROLOGE_DITHER;
                });
                me.wield_state.weave(HOROLOGE);
            },
        ),
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
        "Anelace" => match combat_action.annotation.as_ref() {
            "stab" => {
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &move |me: &mut AgentState| {
                        apply_or_infer_balance(me, (BType::Balance, 2.0), &observations);
                    },
                );
            }
            "hit" => {
                attack_afflictions(
                    agent_states,
                    &combat_action.caster,
                    vec![FType::Hollow, FType::Narcolepsy],
                    &observations,
                );
            }
            "two" => {
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &move |me: &mut AgentState| {
                        me.assume_bard(&|bard: &mut BardClassState| {
                            bard.anelaces += 1;
                        })
                    },
                );
            }
            "" => {
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &move |me: &mut AgentState| {
                        me.assume_bard(&|bard: &mut BardClassState| {
                            bard.anelaces += 1;
                            bard.dithering = ANELACE_DITHER;
                        });
                        me.wield_state.weave(ANELACE);
                        use_destiny_eq(me, &observations);
                    },
                );
            }
            _ => {
                println!("Odd Anelace CombatAction: {:?}", combat_action)
            } // should be no other cases!
        },
        "UnweavedHands" | "UnweavedBelt" | "UnweavedGround" => {
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
                        me.wield_state.unweave(|item_name| {
                            if item_name.starts_with("anelace") && unweaved == ANELACE {
                                true
                            } else if item_name.starts_with("hourglass") && unweaved == HOROLOGE {
                                true
                            } else if item_name.starts_with("thurible") && unweaved == THURIBLE {
                                true
                            } else if item_name.starts_with("nullstone") && unweaved == NULLSTONE {
                                true
                            } else {
                                item_name.eq_ignore_ascii_case(&unweaved)
                            }
                        });
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
                    &combat_action.caster,
                    vec![FType::Berserking],
                    after,
                );
            } else {
                attack_afflictions(
                    agent_states,
                    &combat_action.target,
                    vec![FType::Hatred],
                    after,
                );
                let after = observations.clone();
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &move |me: &mut AgentState| {
                        apply_or_infer_balance(me, (BType::Balance, 2.8), &after);
                    },
                );
                for_agent(
                    agent_states,
                    &combat_action.target,
                    &move |me: &mut AgentState| {
                        if !me.bard_board.dumbness_known()
                            && me.bard_board.emotion_state.primary != Some(Emotion::Anger)
                        {
                            let mut dumbness = false;
                            for observation in &observations {
                                match observation {
                                    AetObservation::Proc(combat_action) => {
                                        if combat_action.skill == "Quip"
                                            && combat_action.annotation == "angry"
                                        {
                                            dumbness = true;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            me.bard_board.observe_dumbness(dumbness);
                        }
                    },
                );
            }
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
        "Tempo" | "Harry" | "Bravado" | "Rhythm" => {
            if !combat_action.skill.eq("Rhythm") {
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &move |me: &mut AgentState| {
                        apply_or_infer_balance(me, (BType::Balance, 2.65), &observations);
                    },
                );
            }
            apply_weapon_hits(
                agent_states,
                &combat_action.caster,
                &combat_action.target,
                after,
                first_person,
                &hints,
            );
            if combat_action.skill.eq("Tempo") || combat_action.skill.eq("Rhythm") {
                let annotation = combat_action.annotation.clone();
                for_agent(
                    agent_states,
                    &combat_action.target,
                    &move |me: &mut AgentState| {
                        if annotation.eq("one") {
                            me.observe_flag(FType::Paresis, true);
                        } else if annotation.eq("two") {
                            me.observe_flag(FType::Shyness, true);
                        } else if annotation.eq("three") {
                            me.observe_flag(FType::Besilence, true);
                        }
                    },
                );
                let annotation = combat_action.annotation.clone();
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &|me: &mut AgentState| {
                        me.assume_bard(&|bard: &mut BardClassState| {
                            bard.on_tempo(if annotation.eq("one") {
                                2
                            } else if annotation.eq("two") {
                                3
                            } else if annotation.eq("three") {
                                4
                            } else {
                                1
                            });
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
        ("Induce", emotion) => {
            for_agent(
                agent_states,
                &combat_action.target,
                &|me: &mut AgentState| {
                    if let Some(emotion) = Emotion::try_from_name(emotion) {
                        me.bard_board.induce(emotion);
                    }
                },
            );
        }
        ("Induced", _) => {
            for_agent(
                agent_states,
                &combat_action.caster,
                &|me: &mut AgentState| {
                    if let Some(emotion) = me.bard_board.emotion_state.primary {
                        me.set_flag(emotion.get_aff(), true);
                    }
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
        ("Discordanced", aff_name) => {
            if let Some(aff) = FType::from_name(&aff_name.to_string()) {
                attack_afflictions(agent_states, &combat_action.caster, vec![aff], after);
            }
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
