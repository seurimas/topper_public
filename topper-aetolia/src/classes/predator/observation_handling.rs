use crate::classes::remove_through;
use crate::curatives::RANDOM_CURES;
use crate::db::AetDatabaseModule;
use crate::timeline::*;
use crate::types::*;

use super::MAWCRUSH_FREELY_HINT;

lazy_static! {
    static ref RAZE_ORDER: Vec<FType> = vec![
        FType::Reflection,
        FType::Shielded,
        FType::Rebounding,
        FType::Speed,
    ];
}

pub const PUMMEL_DAMAGE: f32 = 20.0;
pub const LATERAL_DAMAGE: f32 = 6.0;
pub const LOWHOOK_DAMAGE: f32 = 5.5;
pub const JAB_DAMAGE: f32 = 5.5;
pub const SPINSLASH_DAMAGE: f32 = 4.0;
pub const GOUGE_DAMAGE: f32 = 6.5;
pub const FLASHKICK_DAMAGE: f32 = 5.0;

pub fn use_up_intoxicated(
    agent_states: &mut AetTimelineState,
    target: &String,
    after: &Vec<AetObservation>,
) {
    if attack_hit(after) {
        agent_states.for_agent(target, &move |you: &mut AgentState| {
            you.predator_board.intoxicate_used();
        });
    }
}

pub fn sitara_strike(
    agent_states: &mut AetTimelineState,
    target: &String,
    after: &Vec<AetObservation>,
    count: u32,
) {
    if attack_hit(after) {
        agent_states.for_agent(target, &move |you: &mut AgentState| {
            you.predator_board.sitara_hit(count);
        });
    }
}

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut AetTimelineState,
    _before: &Vec<AetObservation>,
    after: &Vec<AetObservation>,
    db: Option<&impl AetDatabaseModule>,
) -> Result<(), String> {
    let first_person = combat_action.caster.eq(&agent_states.me);
    let hints = agent_states.get_player_hint(&combat_action.caster, &"CALLED_VENOMS".to_string());
    match combat_action.skill.as_ref() {
        // Knifeplay non-combo attacks.
        "Bloodscourge" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Bloodscourge],
                after,
            );
            apply_weapon_hits(
                agent_states,
                &combat_action.caster,
                &combat_action.target,
                after,
                first_person,
                &hints,
            );
            use_up_intoxicated(agent_states, &combat_action.target, after);
            sitara_strike(agent_states, &combat_action.target, after, 1);
        }
        "Fleshbane" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Fleshbane],
                after,
            );
            apply_weapon_hits(
                agent_states,
                &combat_action.caster,
                &combat_action.target,
                after,
                first_person,
                &hints,
            );
            use_up_intoxicated(agent_states, &combat_action.target, after);
            sitara_strike(agent_states, &combat_action.target, after, 1);
        }
        "Fleshbaned" => {
            if combat_action.annotation.eq_ignore_ascii_case("end") {
                for_agent(agent_states, &combat_action.target, &|me| {
                    me.predator_board.fleshbane_end();
                });
            } else {
                let count = match combat_action.annotation.as_str() {
                    "single" => 1,
                    "two" => 2,
                    "three" => 3,
                    "four" => 4,
                    "five" => 5,
                    "six" => 6,
                    "seven" => 7,
                    "eight" => 8,
                    "nine" => 9,
                    "ten" => 10,
                    "eleven" => 11,
                    "twelve" => 12,
                    "thirteen" => 13,
                    "fourteen" => 14,
                    "fifteen" => 15,
                    // Can we get more than 15? Sure.
                    // Will we? Probably not.
                    _ => 15,
                };
                for_agent(agent_states, &combat_action.target, &|me| {
                    me.limb_damage.fleshbaned_count = count;
                    me.predator_board.fleshbane_triggered();
                });
            }
        }
        "Bloodscourged" => {
            let venom = combat_action.annotation.clone();
            for_agent(agent_states, &combat_action.caster, &|me| {
                if venom.eq_ignore_ascii_case("end") {
                    me.predator_board.bloodscourge_end();
                } else {
                    me.predator_board.bloodscourge_hit();
                    apply_venom(me, &venom, false);
                }
            });
        }
        "Cirisosis" => {
            let venom = combat_action.annotation.clone();
            for_agent(agent_states, &combat_action.caster, &|me| {
                if venom.eq_ignore_ascii_case("end") {
                    me.predator_board.cirisosis_lost();
                } else {
                    me.predator_board.cirisosis_shock();
                }
            });
        }
        // Knifeplay combo attacks.
        "Jab" => {
            let limb = LType::from_name(&combat_action.annotation);
            attack_limb_damage(
                agent_states,
                &combat_action.target,
                (limb, JAB_DAMAGE, true),
                after,
            );
            sitara_strike(agent_states, &combat_action.target, after, 1);
            let mut parried = attack_parried(after);
            if agent_states
                .borrow_agent(&combat_action.target)
                .is(FType::TorsoBroken)
                && parried
            {
                toggle_mawcrush_freely(db, true);
            }
        }
        "Lowhook" => {
            let limb = LType::from_name(&combat_action.annotation);
            attack_limb_damage(
                agent_states,
                &combat_action.target,
                (limb, LOWHOOK_DAMAGE, true),
                after,
            );
            sitara_strike(agent_states, &combat_action.target, after, 1);
            let mut parried = attack_parried(after);
            if agent_states
                .borrow_agent(&combat_action.target)
                .is(FType::TorsoBroken)
                && parried
            {
                toggle_mawcrush_freely(db, true);
            }
        }
        "Spinslash" => {
            let limb = LType::from_name(&combat_action.annotation);
            attack_limb_damage(
                agent_states,
                &combat_action.target,
                (limb, SPINSLASH_DAMAGE, true),
                after,
            );
            sitara_strike(agent_states, &combat_action.target, after, 2);
        }
        "Pinprick" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Epilepsy],
                after,
            );
        }
        "Lateral" => {
            attack_limb_damage(
                agent_states,
                &combat_action.target,
                (LType::TorsoDamage, LATERAL_DAMAGE, true),
                after,
            );
            sitara_strike(agent_states, &combat_action.target, after, 1);
            let mut parried = attack_parried(after);
            if agent_states
                .borrow_agent(&combat_action.target)
                .is(FType::TorsoBroken)
            {
                toggle_mawcrush_freely(db, !parried);
            }
        }
        "Vertical" | "Crescentcut" | "Butterfly" => {
            apply_weapon_hits(
                agent_states,
                &combat_action.caster,
                &combat_action.target,
                after,
                first_person,
                &hints,
            );
            use_up_intoxicated(agent_states, &combat_action.target, after);
            sitara_strike(agent_states, &combat_action.target, after, 1);
        }
        "Freefall" => {
            apply_weapon_hits(
                agent_states,
                &combat_action.caster,
                &combat_action.target,
                after,
                first_person,
                &hints,
            );
            use_up_intoxicated(agent_states, &combat_action.target, after);
            sitara_strike(agent_states, &combat_action.target, after, 3);
        }
        "Trip" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Fallen],
                after,
            );
        }
        "Feint" => {
            let limb = LType::from_name(&combat_action.annotation);
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.assume_predator(&|class_state| {
                    class_state.feint();
                });
            });
            for_agent(agent_states, &combat_action.target, &move |target| {
                target.set_parrying(limb);
            });
        }
        "Flashkick" => {
            attack_limb_damage(
                agent_states,
                &combat_action.target,
                (LType::HeadDamage, FLASHKICK_DAMAGE, true),
                after,
            );
            let mut parried = attack_parried(after);
            if agent_states
                .borrow_agent(&combat_action.target)
                .is(FType::TorsoBroken)
                && parried
            {
                toggle_mawcrush_freely(db, true);
            }
        }
        "Flashkicked" => {
            let aff = FType::from_name(&combat_action.annotation);
            if let Some(aff) = aff {
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.toggle_flag(aff, true);
                });
            }
        }
        "Veinrip" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Veinrip],
                after,
            );
            sitara_strike(agent_states, &combat_action.target, after, 1);
            let mut parried = attack_parried(after);
            if agent_states
                .borrow_agent(&combat_action.target)
                .is(FType::TorsoBroken)
                && parried
            {
                toggle_mawcrush_freely(db, true);
            }
        }
        "Veinripped" => {
            if combat_action.annotation.eq_ignore_ascii_case("hit") {
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.predator_board.veinrip.reset();
                });
            } else {
                attack_afflictions(
                    agent_states,
                    &combat_action.caster,
                    vec![FType::Weariness, FType::Dizziness],
                    after,
                );
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.predator_board.veinrip.expire();
                });
            }
        }
        "Raze" => {
            let annotation = combat_action.annotation.clone();
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
                        &RAZE_ORDER.to_vec(),
                    )
                },
            );
        }
        "Swipe" => {
            let annotation = combat_action.annotation.clone();
            for_agent(
                agent_states,
                &combat_action.target,
                &move |me: &mut AgentState| {
                    me.set_flag(FType::Density, false);
                },
            );
        }
        "Gouge" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Disfigurement],
                after,
            );
            attack_limb_damage(
                agent_states,
                &combat_action.target,
                (LType::TorsoDamage, GOUGE_DAMAGE, true),
                after,
            );
            let mut parried = attack_parried(after);
            if agent_states
                .borrow_agent(&combat_action.target)
                .is(FType::TorsoBroken)
                && parried
            {
                toggle_mawcrush_freely(db, true);
            }
        }
        "Tidalslash" => {
            if combat_action.annotation.eq_ignore_ascii_case("full") {
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.assume_predator(&|class| {
                        class.tidalslash_full();
                    });
                });
            } else {
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.assume_predator(&|class| {
                        class.use_tidalslash();
                    });
                });
            }
        }
        // Predation attacks
        "Dartshot" | "Twinshot" => {
            apply_weapon_hits(
                agent_states,
                &combat_action.caster,
                &combat_action.target,
                after,
                first_person,
                &hints,
            );
            let mut check_cirisosis = false;
            for observation in after {
                if check_cirisosis {
                    if matches!(observation, AetObservation::DiscernedCure(_, _)) {
                        for_agent(agent_states, &combat_action.target, &|me| {
                            me.predator_board.cirisosis_start();
                        });
                    } else {
                        break;
                    }
                }
                if let AetObservation::Devenoms(devenomed) = observation {
                    if devenomed == "cirisosis" {
                        check_cirisosis = true;
                    }
                }
            }
        }
        "Pheromones" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::LoversEffect],
                after,
            );
        }
        "Mindnumb" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Impairment],
                after,
            );
        }
        "Ferocity" => {
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
                    apply_or_infer_balance(me, (BType::ClassCure1, 20.0), &observations);
                },
            );
        }
        "Arouse" => {
            let observations = after.clone();
            let perspective = agent_states.get_perspective(&combat_action);
            for_agent(
                agent_states,
                &combat_action.caster,
                &move |me: &mut AgentState| {
                    let arouse_time = me
                        .check_if_predator(&|predator| predator.get_arouse_time())
                        .unwrap_or(90.0);
                    apply_or_infer_balance(me, (BType::ClassCure2, arouse_time), &observations);
                },
            );
        }
        "Pindown" => {
            if combat_action.annotation.eq_ignore_ascii_case("fail") {
                for_agent(agent_states, &combat_action.target, &|me| {
                    me.observe_not_prone();
                });
            } else {
                attack_afflictions(
                    agent_states,
                    &combat_action.target,
                    vec![FType::WritheDartpinned],
                    after,
                );
            }
        }
        // Beastmastery
        "Beastcalled" => {
            let beast = combat_action.annotation.clone();
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.assume_predator(&|class_state| {
                    if beast.contains("spider") {
                        class_state.get_spider();
                    } else if beast.contains("orgyuk") {
                        class_state.get_orgyuk();
                    } else if beast.contains("orel") {
                        class_state.get_orel();
                    }
                });
            });
        }
        "Negate" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Negated],
                after,
            );
        }
        "Acid" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Acid],
                after,
            );
        }
        "Web" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::WritheWeb],
                after,
            );
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.assume_predator(&|class_state| {
                    class_state.webbed();
                });
            });
        }
        "Intoxicate" => {
            let target = combat_action.target.clone();
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.assume_predator(&|class_state| {
                    class_state.intoxicate(target.clone());
                });
            });
        }
        "Intoxicated" => {
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.predator_board.intoxicate();
            });
        }
        "Pummel" => {
            let limb = LType::from_name(&combat_action.annotation);
            attack_limb_damage(
                agent_states,
                &combat_action.target,
                (limb, PUMMEL_DAMAGE, true),
                after,
            );
        }
        "Mawrcrush" => {
            if combat_action.annotation.eq_ignore_ascii_case("fail") {
                for_agent(agent_states, &combat_action.target, &|me| {
                    me.observe_flag(FType::TorsoBroken, false);
                });
            } else {
                let mut parried = attack_parried(after);
                toggle_mawcrush_freely(db, !parried);
            }
        }
        "Rake" => {
            let who = combat_action.target.clone();
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.assume_predator(&|class_state| {
                    class_state.rake_start(who.clone());
                });
            });
        }
        "Raked" => {
            let who = combat_action.caster.clone();
            agent_states.agent_states.iter_mut().for_each(|(id, me)| {
                for agent in me {
                    if let ClassState::Predator(ref mut predator) = agent.class_state {
                        predator.rake(&who);
                    }
                }
            });
        }
        _ => {}
    }
    Ok(())
}

fn toggle_mawcrush_freely(db: Option<&impl AetDatabaseModule>, value: bool) {
    if let Some(db) = db {
        db.insert_hint(&MAWCRUSH_FREELY_HINT.to_string(), &value.to_string());
    }
}
