use crate::aetolia::classes::{
    get_needed_parry, get_preferred_parry as get_parry, is_affected_by, Class,
};
use crate::aetolia::curatives::{MENTAL_AFFLICTIONS, NORMAL_SALVE_AFFS, SOOTHING_SKIN_ORDER};
use crate::aetolia::timeline::*;
use crate::aetolia::topper::*;
use crate::aetolia::types::*;
use crate::topper::db::DatabaseModule;

pub fn get_preferred_parry(
    timeline: &AetTimeline,
    me: &String,
    target: &String,
    strategy: &String,
) -> Result<LType, String> {
    Ok(LType::TorsoDamage)
}

const PUMMEL_DAMAGE: f32 = 9.5;
const WANEKICK_DAMAGE: f32 = 9.0;
const CLAWTWIST_DAMAGE: f32 = 8.5;
const SUNKICK_DAMAGE: f32 = 6.0;
const RISEKICK_DAMAGE: f32 = 5.0;
const HEELRUSH_ONE_DAMAGE: f32 = 5.5;
const HEELRUSH_TWO_DAMAGE: f32 = 8.0;
const HEELRUSH_THREE_DAMAGE: f32 = 11.0;
const HEELRUSH_DAMAGE: f32 = HEELRUSH_ONE_DAMAGE + HEELRUSH_TWO_DAMAGE + HEELRUSH_THREE_DAMAGE;
const DIREBLOW_WEAK_DAMAGE: f32 = 10.0;
const DIREBLOW_STRONG_DAMAGE: f32 = 20.0;
const SWAGGER_LIMIT: u8 = 3;

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut AetTimelineState,
    _before: &Vec<AetObservation>,
    after: &Vec<AetObservation>,
) -> Result<(), String> {
    match combat_action.skill.as_ref() {
        "Welts" => {
            let limb = match combat_action.annotation.as_ref() {
                "head" => LType::HeadDamage,
                "torso" => LType::TorsoDamage,
                "left arm" => LType::LeftArmDamage,
                "right arm" => LType::RightArmDamage,
                "left leg" => LType::LeftLegDamage,
                "right leg" => LType::RightLegDamage,
                _ => LType::SIZE, // I don't want to panic
            };
            for_agent_closure(
                agent_states,
                &combat_action.caster,
                Box::new(move |you| {
                    you.limb_damage.welt(limb);
                }),
            );
        }
        "Hellcat" => {
            for_agent(agent_states, &combat_action.caster, |you| {
                if you.is(FType::Ablaze) {
                    you.tick_flag_up(FType::Ablaze);
                }
            });
        }
        "WeltHit" => {
            let limb = match combat_action.annotation.as_ref() {
                "head" => LType::HeadDamage,
                "torso" => LType::TorsoDamage,
                "left arm" => LType::LeftArmDamage,
                "right arm" => LType::RightArmDamage,
                "left leg" => LType::LeftLegDamage,
                "right leg" => LType::RightLegDamage,
                _ => LType::SIZE, // I don't want to panic
            };
            attack_limb_damage(
                agent_states,
                &combat_action.caster,
                (limb, 6.5, true),
                after,
            );
        }
        "Dislocated" => {
            let (limb, dislocation) = match combat_action.annotation.as_ref() {
                "left arm" => (LType::LeftArmDamage, FType::LeftArmDislocated),
                "right arm" => (LType::RightArmDamage, FType::RightArmDislocated),
                "left leg" => (LType::LeftLegDamage, FType::LeftLegDislocated),
                "right leg" => (LType::RightLegDamage, FType::RightLegDislocated),
                _ => (LType::SIZE, FType::SIZE), // I don't want to panic
            };
            for_agent_closure(
                agent_states,
                &combat_action.caster,
                Box::new(move |you| {
                    let limb_state = you.get_limb_state(limb);
                    let damage_change = 33.34 - limb_state.damage;
                    you.limb_damage.set_limb_damaged(limb, true);
                    you.toggle_flag(dislocation, false);
                }),
            );
        }
        "InfernalSeal" => {
            for_agent(agent_states, &combat_action.caster, |you| {
                you.observe_flag(FType::Ablaze, true);
                you.toggle_flag(FType::InfernalSeal, true);
            });
        }
        "Zenith" => {
            for_agent(agent_states, &combat_action.caster, |you| {
                you.assume_zealot(|zealot| zealot.zenith.initiate());
            });
        }
        "Pyromania" => match combat_action.annotation.as_ref() {
            "" => {
                for_agent(agent_states, &combat_action.caster, |you| {
                    you.assume_zealot(|zealot| zealot.pyromania.activate(2000));
                });
            }
            "hit" => {
                for_agent(agent_states, &combat_action.caster, |me| {
                    if me.is(FType::Ablaze) {
                        me.tick_flag_up(FType::Ablaze);
                    }
                });
            }
            "fall" => {
                for_agent(agent_states, &combat_action.caster, |me| {
                    me.toggle_flag(FType::Fallen, true);
                });
            }
            "shield" => {
                for_agent(agent_states, &combat_action.caster, |me| {
                    me.toggle_flag(FType::Shielded, false);
                });
            }
            _ => {}
        },
        "Heelrush" => {
            apply_combo_balance(
                agent_states,
                &combat_action.caster,
                (BType::Balance, 3.5),
                after,
            );

            if let Ok(limb) = get_limb_damage(&combat_action.annotation) {
                attack_limb_damage(
                    agent_states,
                    &combat_action.target,
                    (limb, HEELRUSH_ONE_DAMAGE, true),
                    after,
                );
                for_agent_closure(
                    agent_states,
                    &combat_action.caster,
                    Box::new(move |me| {
                        me.set_channel(ChannelState::Heelrush(limb, 325));
                    }),
                );
            }
        }
        "Heelrush Two" => {
            if let Ok(limb) = get_limb_damage(&combat_action.annotation) {
                attack_limb_damage(
                    agent_states,
                    &combat_action.target,
                    (limb, HEELRUSH_TWO_DAMAGE, true),
                    after,
                );
            }
        }
        "Heelrush Three" => {
            if let Ok(limb) = get_limb_damage(&combat_action.annotation) {
                attack_limb_damage(
                    agent_states,
                    &combat_action.target,
                    (limb, HEELRUSH_THREE_DAMAGE, true),
                    after,
                );
            }
        }
        "Direblow Weak" => {
            attack_limb_damage(
                agent_states,
                &combat_action.target,
                (LType::TorsoDamage, DIREBLOW_WEAK_DAMAGE, true),
                after,
            );
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Lightwound],
                after,
            );
        }
        "Direblow Strong" => {
            attack_limb_damage(
                agent_states,
                &combat_action.target,
                (LType::TorsoDamage, DIREBLOW_STRONG_DAMAGE, true),
                after,
            );
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Lightwound, FType::Deepwound],
                after,
            );
        }
        "Risekick" => {
            apply_combo_balance(
                agent_states,
                &combat_action.caster,
                (BType::Balance, 3.65),
                after,
            );
            attack_limb_damage(
                agent_states,
                &combat_action.target,
                (LType::TorsoDamage, RISEKICK_DAMAGE, true),
                after,
            );
            for_agent(agent_states, &combat_action.caster, |me| {
                me.toggle_flag(FType::Fallen, false);
            });
        }
        "Pummel" => {
            apply_combo_balance(
                agent_states,
                &combat_action.caster,
                (BType::Balance, 3.65),
                after,
            );

            if let Ok(limb) = get_limb_damage(&combat_action.annotation) {
                attack_limb_damage(
                    agent_states,
                    &combat_action.target,
                    (limb, PUMMEL_DAMAGE, true),
                    after,
                );
            }
        }
        "Wanekick" => {
            apply_combo_balance(
                agent_states,
                &combat_action.caster,
                (BType::Balance, 3.5),
                after,
            );

            if let Ok(limb) = get_limb_damage(&combat_action.annotation) {
                attack_limb_damage(
                    agent_states,
                    &combat_action.target,
                    (limb, WANEKICK_DAMAGE, true),
                    after,
                );
            }
        }
        "Clawtwist" => {
            apply_combo_balance(
                agent_states,
                &combat_action.caster,
                (BType::Balance, 3.5),
                after,
            );

            attack_limb_damage(
                agent_states,
                &combat_action.target,
                (LType::TorsoDamage, CLAWTWIST_DAMAGE, true),
                after,
            );
        }
        "Sunkick" => {
            apply_combo_balance(
                agent_states,
                &combat_action.caster,
                (BType::Balance, 3.5),
                after,
            );

            attack_limb_damage(
                agent_states,
                &combat_action.target,
                (LType::HeadDamage, SUNKICK_DAMAGE, true),
                after,
            );
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Dizziness, FType::Stupidity],
                after,
            );
        }
        "Palmforce" => {
            apply_combo_balance(
                agent_states,
                &combat_action.caster,
                (BType::Balance, 3.5),
                after,
            );
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Fallen],
                after,
            );
        }
        "Twinpress" => {
            apply_combo_balance(
                agent_states,
                &combat_action.caster,
                (BType::Balance, 3.25),
                after,
            );

            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::MuscleSpasms, FType::Stiffness],
                after,
            );
        }
        "Dislocate" => {
            apply_combo_balance(
                agent_states,
                &combat_action.caster,
                (BType::Balance, 3.5),
                after,
            );

            let aff = match combat_action.annotation.as_ref() {
                "left arm" => Some(FType::LeftArmDislocated),
                "right arm" => Some(FType::RightArmDislocated),
                "left leg" => Some(FType::LeftLegDislocated),
                "right leg" => Some(FType::RightLegDislocated),
                _ => None,
            };
            if let Some(aff) = aff {
                attack_afflictions(agent_states, &combat_action.target, vec![aff], after);
            }
        }
        "Anklepin" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::SoreAnkle],
                after,
            );
            for_agent(agent_states, &combat_action.target, |you| {
                you.limb_damage.dewelt(LType::LeftLegDamage);
                you.limb_damage.dewelt(LType::RightLegDamage);
            });
        }
        "Wristlash" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::SoreWrist],
                after,
            );
            for_agent(agent_states, &combat_action.target, |you| {
                you.limb_damage.dewelt(LType::LeftArmDamage);
                you.limb_damage.dewelt(LType::RightArmDamage);
            });
        }
        "Descent" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Backstrain],
                after,
            );
            for_agent(agent_states, &combat_action.target, |you| {
                you.limb_damage.dewelt(LType::TorsoDamage);
            });
        }
        "Uprise" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Whiplash],
                after,
            );
            for_agent(agent_states, &combat_action.target, |you| {
                you.limb_damage.dewelt(LType::HeadDamage);
            });
        }
        "Jawcrack" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Stuttering, FType::BlurryVision],
                after,
            );
        }
        "Rejection" => {
            for_agent(agent_states, &combat_action.caster, |me| {
                me.set_flag(FType::Rebounding, true);
            });
        }
        "Pendulum" => {
            let observations = after.clone();
            for_agent_closure(
                agent_states,
                &combat_action.caster,
                Box::new(move |me| {
                    apply_or_infer_balance(me, (BType::Equil, 3.0), &observations);
                    me.set_balance(BType::Pendulum, 10.0);
                }),
            );
            let annotation = combat_action.annotation.clone();
            let observations = after.clone();
            for_agent_closure(
                agent_states,
                &combat_action.target,
                Box::new(move |you| {
                    you.rotate_limbs(annotation == "anti-clockwise");
                }),
            );
        }
        "Whipburst" => {
            for_agent(agent_states, &combat_action.target, |you| {
                if you.is(FType::Ablaze) {
                    you.tick_flag_up(FType::Ablaze);
                }
            });
        }
        "Quicken" => {
            let observations = after.clone();
            for_agent_closure(
                agent_states,
                &combat_action.caster,
                Box::new(move |me| {
                    apply_or_infer_balance(me, (BType::Equil, 3.0), &observations);
                }),
            );
            for_agent(agent_states, &combat_action.target, |you| {
                you.tick_flag_up(FType::Ablaze);
                you.tick_flag_up(FType::Ablaze);
                you.tick_flag_up(FType::Ablaze);
            });
        }
        "Infernal" => {
            if combat_action.annotation.eq("failure") {
                for_agent(agent_states, &combat_action.caster, |you| {
                    you.limb_damage.set_limb_damaged(LType::TorsoDamage, false);
                });
            } else {
                let observations = after.clone();
                for_agent_closure(
                    agent_states,
                    &combat_action.caster,
                    Box::new(move |me| {
                        apply_or_infer_balance(me, (BType::Equil, 2.0), &observations);
                    }),
                );
                let observations = after.clone();
                for_agent(agent_states, &combat_action.target, |you| {
                    you.set_flag(FType::InfernalSeal, true);
                });
            }
        }
        "InfernalShroud" => {
            for_agent(agent_states, &combat_action.caster, |you| {
                you.set_flag(FType::Shielded, false);
            });
        }
        "Scorch" => {
            let observations = after.clone();
            for_agent_closure(
                agent_states,
                &combat_action.caster,
                Box::new(move |me| {
                    apply_or_infer_balance(me, (BType::Equil, 2.0), &observations);
                }),
            );
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Ablaze],
                after,
            );
        }
        "Heatspear" => {
            if combat_action.annotation.eq("failure") {
                for_agent(agent_states, &combat_action.caster, |you| {
                    you.observe_flag(FType::Ablaze, false);
                });
            } else {
                let observations = after.clone();
                for_agent_closure(
                    agent_states,
                    &combat_action.caster,
                    Box::new(move |me| {
                        apply_or_infer_balance(me, (BType::Equil, 3.0), &observations);
                    }),
                );
                attack_afflictions(
                    agent_states,
                    &combat_action.target,
                    vec![FType::Ablaze, FType::Heatspear],
                    after,
                );
            }
        }
        "Firefist" => {
            for_agent(agent_states, &combat_action.caster, |me| {
                me.set_balance(BType::Firefist, 80.0);
            });
        }
        "Wrath" => {
            for_agent(agent_states, &combat_action.caster, |me| {
                me.set_balance(BType::Wrath, 30.0);
            });
        }
        "Dull" => {
            for_agent(agent_states, &combat_action.target, |me| {
                me.set_flag(FType::Indifference, true);
            });
        }
        "Immolation" => {
            if combat_action.annotation.eq("failure") {
                for_agent(agent_states, &combat_action.target, |me| {
                    me.observe_flag(FType::Ablaze, false);
                });
            }
        }
        "Recover" => {
            for_agent(agent_states, &combat_action.caster, |me| {
                me.set_balance(BType::ClassCure1, 20.0);
            });
        }
        "Hackles" => {
            for_agent(agent_states, &combat_action.caster, |me| {
                me.set_balance(BType::Secondary, 6.5);
            });
        }
        "Disable" => {
            for_agent(agent_states, &combat_action.caster, |me| {
                me.set_balance(BType::Disable, 90.0);
            });
            for_agent(agent_states, &combat_action.target, |me| {
                me.set_balance(BType::Disabled, 12.0);
            });
        }
        _ => {}
    }
    Ok(())
}

#[derive(Debug)]
pub enum ComboType {
    ComboAny,
    ComboFirst,
    ComboSecond,
    Hackles,
    ZenithEq,
    AnyEq,
    Full,
    Free,
}

pub type ZealotPriority = (
    &'static str,
    fn(&AgentState, &AgentState, Option<(&DatabaseModule, &String)>, &String) -> (ComboType, f32),
);

fn value_pendulum(
    me: &AgentState,
    you: &AgentState,
    target_limbs: &LimbsState,
    counter: bool,
) -> f32 {
    if me.get_balance(BType::Pendulum) < me.get_qeb_balance() {
        if let (Some(timer), Some(limb)) =
            (you.limb_damage.restore_timer, you.limb_damage.restoring)
        {
            let timer = timer as f32 / BALANCE_SCALE - me.get_qeb_balance();
            if timer > 0.0 {
                if !you.get_limb_state(limb).broken && timer < 1.0 {
                    println!("No pendulum, timer at {}", timer);
                    return 0.0;
                }
                let mut after_rotate_state = you.limb_damage.clone();
                after_rotate_state.rotate(counter);
                after_rotate_state.complete_restore(None);
                let mut after_base_state = you.limb_damage.clone();
                after_base_state.complete_restore(None);
                let rotated_legs = after_rotate_state
                    .get_limbs_damage(vec![LType::LeftLegDamage, LType::RightLegDamage]);
                let unrotated_legs = after_base_state
                    .get_limbs_damage(vec![LType::LeftLegDamage, LType::RightLegDamage]);
                let change =
                    after_rotate_state.get_total_damage() - after_base_state.get_total_damage();
                if change > 20.0 {
                    println!(
                        "Pendulum valued at {} ({}, {})",
                        rotated_legs - unrotated_legs + change,
                        rotated_legs - unrotated_legs,
                        change,
                    );
                    (rotated_legs - unrotated_legs) + change
                } else {
                    0.0
                }
            } else {
                0.0
            }
        } else {
            0.0
        }
    } else {
        0.0
    }
}

fn psi_percent(me: &AgentState) -> f32 {
    me.get_stat(SType::SP) as f32 * 100.0 / 3000.0
}

fn can_kick(me: &AgentState) -> bool {
    !me.is(FType::Paralysis)
        && !me.limb_damage.broken(LType::LeftLegDamage)
        && !me.limb_damage.broken(LType::RightLegDamage)
        && (!me.limb_damage.broken(LType::LeftArmDamage)
            || !me.limb_damage.broken(LType::RightArmDamage))
}

fn can_punch(me: &AgentState) -> bool {
    me.get_count(FType::SappedStrength) < SWAGGER_LIMIT && !me.is(FType::Paralysis)
}

fn db_class(db: Option<(&DatabaseModule, &String)>) -> Option<Class> {
    db.and_then(|(db, who)| db.get_class(who))
}

fn value_disable(
    disable: Option<&str>,
    me: &AgentState,
    you: &AgentState,
    db: Option<(&DatabaseModule, &String)>,
    _strategy: &String,
) -> f32 {
    if !me.balanced(BType::Disable) {
        0.0
    } else if (Some("tarot aeon") == disable || disable.is_none())
        && !me.is(FType::Speed)
        && (me.is(FType::Asthma) || me.is(FType::Clumsiness) || me.is(FType::Weariness))
        && Some(Class::Indorani) == db_class(db)
    {
        1.0
    } else {
        0.0
    }
}

fn value_swagger(
    me: &AgentState,
    you: &AgentState,
    db: Option<(&DatabaseModule, &String)>,
    _strategy: &String,
) -> f32 {
    let sapped = me.get_count(FType::SappedStrength);
    if me.is(FType::Swagger) {
        0.0
    } else if sapped < SWAGGER_LIMIT {
        let class = db_class(db);
        if (Some(Class::Luminary) == class)
            && (!me.is(FType::Paresis)
                || (me.is(FType::Paresis)
                    && !me.balanced(BType::Tree)
                    && me.get_balance(BType::ParesisParalysis) > 2.0))
        {
            0.0
        } else if sapped == SWAGGER_LIMIT - 1 {
            if let Some(locked) = me.lock_duration() {
                if me.balanced(BType::Tree) {
                    1.0
                } else {
                    0.0
                }
            } else if !me.is(FType::Firefist)
                && !me.is(FType::Zenith)
                && !you.is(FType::Heatspear)
                && !you.restore_count() > 1
            {
                1.0
            } else {
                0.0
            }
        } else {
            1.0
        }
    } else {
        0.0
    }
}

fn value_heelrush(
    limb: LType,
    me: &AgentState,
    you: &AgentState,
    db: Option<(&DatabaseModule, &String)>,
    strategy: &String,
) -> f32 {
    let limb_state = you.get_limb_state(limb);
    if you.get_restoring().is_some()
        && limb_state.hits_to_break(HEELRUSH_DAMAGE) == 1
        && !limb_state.damaged
        && !limb_state.is_restoring
        && !limb_state.is_parried
        && !me.is(FType::Paresis)
        && can_kick(me)
        && !me.is(FType::Zenith)
    {
        value_limb(limb, me, you, db, strategy) * 1.5
    } else {
        0.0
    }
}

fn value_limb(
    limb: LType,
    me: &AgentState,
    you: &AgentState,
    db: Option<(&DatabaseModule, &String)>,
    strategy: &String,
) -> f32 {
    use rand::distributions::{Distribution, Uniform};
    let limb_state = you.get_limb_state(limb);
    if strategy.eq_ignore_ascii_case("bedazzle") {
        let between = Uniform::new(5.0, 30.0);
        return between.sample(&mut rand::thread_rng());
    } else if strategy.eq_ignore_ascii_case("class") {
        let impulse = you.limb_damage.get_total_damage();
        if impulse > 60.0 || you.restore_count() > 0 {
            let damage = match limb {
                LType::HeadDamage => SUNKICK_DAMAGE,
                LType::TorsoDamage => CLAWTWIST_DAMAGE,
                LType::LeftArmDamage => PUMMEL_DAMAGE,
                LType::RightArmDamage => PUMMEL_DAMAGE,
                LType::LeftLegDamage => WANEKICK_DAMAGE,
                LType::RightLegDamage => WANEKICK_DAMAGE,
                _ => 1.0,
            };
            if limb_state.hits_to_break(damage) > 0 {
                return 20.0 - limb_state.hits_to_break(damage) as f32;
            } else {
                return 0.0;
            }
        } else if limb_state.damage <= impulse {
            let between = Uniform::new(1.0, 5.0);
            return impulse - limb_state.damage + between.sample(&mut rand::thread_rng());
        } else {
            return 0.0;
        }
    }
    match limb {
        LType::TorsoDamage => {
            if you.is(FType::Fallen) {
                30.0
            } else {
                10.0
            }
        }
        LType::HeadDamage => {
            if you.is(FType::Indifference) {
                20.0
            } else {
                9.0
            }
        }
        LType::LeftArmDamage => {
            if Some(LType::RightLegDamage) == you.limb_damage.restoring {
                30.0
            } else if you.limb_damage.get_damage(LType::LeftLegDamage) > 2500 {
                20.0
            } else {
                5.0
            }
        }
        LType::RightArmDamage => {
            if Some(LType::LeftLegDamage) == you.limb_damage.restoring {
                30.0
            } else if you.limb_damage.get_damage(LType::RightLegDamage) > 2500 {
                20.0
            } else {
                5.0
            }
        }
        LType::LeftLegDamage => 8.0,
        LType::RightLegDamage => 8.0,
        _ => 0.0,
    }
}

lazy_static! {
    static ref MAIN_STACK: Vec<ZealotPriority> = vec![
        ("wrath", |me, you, _db, _strategy| {
            (
                ComboType::Free,
                if me.balanced(BType::Wrath) && you.limb_damage.get_total_damage() > 60.0 { 1.0 } else { 0.0 },
            )
        }),
        ("light pipes;;cinder", |me, you, _db, _strategy| {
            (ComboType::Free, 1.0)
        }),
        ("psi torrent", |me, you, db, strategy| {
            (
                ComboType::Free,
                if value_disable(None, me, you, db, strategy) > 0.0 && me.get_stat(SType::SP) < 1000
                {
                    2.0
                } else {
                    0.0
                },
            )
        }),
        ("psi disable {} tarot aeon", |me, you, db, strategy| {
            (
                ComboType::Free,
                value_disable(Some("tarot aeon"), me, you, db, strategy),
            )
        }),
        ("psi dull {}", |me, you, db, _strategy| {
            (
                ComboType::Full,
                if you.get_restore_time_left() - me.get_qeb_balance() > 2.0
                    && !you.is(FType::Indifference)
                    && !you.is(FType::Stuttering)
                    && !you.is(FType::BlurryVision)
                    && (!you.can_focus(false) || you.is_prone())
                    && me.get_stat(SType::SP) > 200
                    && !me.is(FType::Zenith)
                    && psi_percent(me) < 50.0
                {
                    40.0
                } else {
                    0.0
                },
            )
        }),
        ("swagger", |me, you, db, strategy| {
            (ComboType::Free, value_swagger(me, you, db, strategy))
        }),
        ("enact firefist", |me, you, _db, _strategy| {
            let target_limbs = you.get_limbs_state();
            (
                ComboType::Free,
                if me.is(FType::Firefist) || !me.balanced(BType::Firefist) {
                    0.0
                } else if me.is(FType::Zenith) && can_punch(me) {
                    1.0
                } else {
                    0.0
                },
            )
        }),
        ("enact immolation {}", |me, you, _db, _strategy| {
            (
                ComboType::Full,
                if you.get_count(FType::Ablaze) > 12 && !you.is(FType::Shielded) {
                    1000.0
                } else {
                    0.0
                },
            )
        }),
        ("touch hammer {}", |me, you, _db, _strategy| {
            (
                ComboType::Full,
                if you.is(FType::Shielded) { 100.0 } else { 0.0 },
            )
        }),
        ("respiration hold;;firstaid halt for channel on", |me, you, db, strategy| {
            (
                ComboType::Full,
                if me.get_count(FType::SappedStrength) >= SWAGGER_LIMIT {
                    100.0
                } else if value_swagger(me, you, db, strategy) > 0.0
                    && me.get_count(FType::SappedStrength) == SWAGGER_LIMIT - 1
                {
                    100.0
                } else {
                    0.0
                },
            )
        }),
        ("psi recover", |me, you, _db, _strategy| {
            (
                ComboType::Full,
                if me.balanced(BType::ClassCure1) {
                    me.affs_count(&MENTAL_AFFLICTIONS.to_vec()) as f32 * 60.0
                } else {
                    0.0
                },
            )
        }),
        ("enact pendulum {}", |me, you, _db, _strategy| {
            let target_limbs = you.get_limbs_state();
            (
                ComboType::Full,
                value_pendulum(me, you, &target_limbs, false),
            )
        }),
        ("enact pendulum {} reverse", |me, you, _db, _strategy| {
            let target_limbs = you.get_limbs_state();
            (
                ComboType::Full,
                value_pendulum(me, you, &target_limbs, true),
            )
        }),
        ("enact zenith", |me, you, _db, _strategy| {
            (
                ComboType::Full,
                if let ClassState::Zealot(ZealotClassState {zenith, .. }) = me.class_state {
                    if zenith.can_initiate() {
                        1000.0
                    } else {
                        0.0
                    }
                } else {
                    // We're not even marked as Zealot yet! GET ZENITH ON!
                    1000.0
                },
            )
        }),
        ("enact scorch {}", |me, you, _db, _strategy| {
            (
                ComboType::ZenithEq,
                if !you.is(FType::Ablaze) { 40.0 } else { 0.0 },
            )
        }),
        ("enact heatspear {}", |me, you, _db, _strategy| {
            let target_limbs = you.get_limbs_state();
            (
                if !you.is(FType::Heatspear)
                    && you.get_count(FType::Ablaze) >= 6
                    && !me.is(FType::Zenith)
                {
                    ComboType::Full
                } else {
                    ComboType::ZenithEq
                },
                if you.is(FType::Ablaze) && !you.is(FType::Heatspear) {
                    if target_limbs.torso.damage > 33.3 || you.get_count(FType::Ablaze) >= 6 {
                        200.0
                    } else {
                        60.0
                    }
                } else {
                    0.0
                },
            )
        }),
        ("enact pyromania", |me, you, _db, _strategy| {
            let target_limbs = you.get_limbs_state();
            (
                ComboType::ZenithEq,
                if let ClassState::Zealot(ZealotClassState { pyromania, .. }) = me.class_state {
                    if !pyromania.active() {
                        50.0
                    } else {
                        0.0
                    }
                } else {
                    50.0
                },
            )
        }),
        ("enact quicken {}", |me, you, _db, _strategy| {
            let target_limbs = you.get_limbs_state();
            (
                if you.is(FType::Heatspear)
                    && you.get_count(FType::Ablaze) >= 6
                    && !me.is(FType::Zenith)
                    && you.get_curing() != Some(FType::Heatspear)
                {
                    ComboType::Full
                } else {
                    ComboType::ZenithEq
                },
                if you.is(FType::Ablaze) && you.is(FType::Heatspear) {
                    100.0
                } else {
                    0.0
                },
            )
        }),
        ("enact infernal {}", |me, you, _db, _strategy| {
            let target_limbs = you.get_limbs_state();
            (
                if let ClassState::Zealot(ZealotClassState { zenith, .. }) = me.class_state {
                    if zenith.active() {
                        ComboType::ZenithEq
                    } else {
                        ComboType::Full
                    }
                } else {
                    ComboType::Full
                },
                if target_limbs.torso.damaged && !you.is(FType::InfernalSeal) {
                    1000.0
                } else {
                    0.0
                },
            )
        }),
        ("risekick", |me, you, _db, _strategy| {
            (
                ComboType::ComboFirst,
                if me.is(FType::Fallen) && can_kick(me) {
                    100.0
                } else {
                    0.0
                },
            )
        }),
        ("twinpress", |me, you, _db, _strategy| {
            let target_limbs = you.get_limbs_state();
            let spasms_value = if you.is(FType::MuscleSpasms) {
                0.0
            } else {
                (target_limbs.restores_to_zeroes() as f32 * 7.0) + (you.affs_count(&NORMAL_SALVE_AFFS) as f32 * 3.0)
            };
            let stiffness_value = if you.is(FType::Stiffness) {
                0.0
            } else {
                you.affs_count(&SOOTHING_SKIN_ORDER) as f32 * 5.0
            };
            (
                ComboType::ComboAny,
                if can_punch(me) {
                    spasms_value + stiffness_value
                } else {
                    0.0
                },
            )
        }),
        ("palmforce", |me, you, _db, _strategy| {
            let target_limbs = you.get_limbs_state();
            (
                ComboType::ComboAny,
                if !you.is(FType::Fallen)
                    && (target_limbs.left_leg.broken || target_limbs.right_leg.broken)
                    && target_limbs.restores_to_zeroes() >= 1
                    && can_punch(me)
                {
                    (30 * target_limbs.restores_to_zeroes()) as f32
                } else {
                    0.0
                },
            )
        }),
        ("clawtwist", |me, you, db, strategy| {
            let target_limbs = you.get_limbs_state();
            (
                ComboType::ComboAny,
                if !target_limbs.torso.damaged
                    && !target_limbs.torso.is_restoring
                    && !target_limbs.torso.is_parried
                    && can_punch(me)
                {
                    value_limb(LType::TorsoDamage, me, you, db, strategy)
                } else if me.is(FType::Zenith) || you.is(FType::Heatspear) {
                    15.0
                } else if target_limbs.torso.damage > 33.3
                    && you.is(FType::Heatspear)
                    && !target_limbs.torso.is_parried
                {
                    40.0
                } else {
                    2.0
                },
            )
        }),
        ("sunkick", |me, you, db, strategy| {
            let target_limb = you.get_limb_state(LType::HeadDamage);
            (
                ComboType::ComboAny,
                if me.is(FType::Zenith) || you.is(FType::Heatspear) {
                    0.0
                } else if !target_limb.is_restoring && !target_limb.is_parried && can_kick(me) {
                    value_limb(LType::TorsoDamage, me, you, db, strategy)
                } else {
                    0.0
                },
            )
        }),
        ("clawtwist", |me, you, db, _strategy| {
            let target_limb = you.get_limb_state(LType::TorsoDamage);
            (
                ComboType::ComboAny,
                if !target_limb.damaged
                    && target_limb.hits_to_break(PUMMEL_DAMAGE) == 2
                    && (!you.is(FType::InfernalSeal) || you.is(FType::Heatspear))
                    && !target_limb.is_parried
                {
                    40.0
                } else {
                    2.0
                },
            )
        }),
        ("dislocate left arm", |me, you, _db, _strategy| {
            let target_limbs = you.get_limbs_state();
            let mut restores = target_limbs.restores_to_zeroes();
            if you.is(FType::Heatspear) {
                restores = restores + 1;
            }
            (
                ComboType::ComboAny,
                if restores > 1
                    && !target_limbs.left_arm.is_parried
                    && !you.is(FType::LeftArmDislocated)
                    && can_punch(me)
                {
                    30.0 - target_limbs.left_arm.damage
                } else {
                    0.0
                },
            )
        }),
        ("dislocate right arm", |me, you, _db, _strategy| {
            let target_limbs = you.get_limbs_state();
            let mut restores = target_limbs.restores_to_zeroes();
            if you.is(FType::Heatspear) {
                restores = restores + 1;
            }
            (
                ComboType::ComboAny,
                if restores > 1
                    && !target_limbs.right_arm.is_parried
                    && !you.is(FType::RightArmDislocated)
                    && can_punch(me)
                {
                    30.0 - target_limbs.right_arm.damage
                } else {
                    0.0
                },
            )
        }),
        ("dislocate left leg", |me, you, _db, _strategy| {
            let target_limbs = you.get_limbs_state();
            let mut restores = target_limbs.restores_to_zeroes();
            if you.is(FType::Heatspear) {
                restores = restores + 1;
            }
            (
                ComboType::ComboAny,
                if restores > 1
                    && !target_limbs.left_leg.is_parried
                    && !you.is(FType::LeftLegDislocated)
                    && can_punch(me)
                {
                    30.0 - target_limbs.left_leg.damage
                } else {
                    0.0
                },
            )
        }),
        ("dislocate right leg", |me, you, _db, _strategy| {
            let target_limbs = you.get_limbs_state();
            let mut restores = target_limbs.restores_to_zeroes();
            if you.is(FType::Heatspear) {
                restores = restores + 1;
            }
            (
                ComboType::ComboAny,
                if restores > 1
                    && !target_limbs.right_leg.is_parried
                    && !you.is(FType::RightLegDislocated)
                    && can_punch(me)
                {
                    30.0 - target_limbs.right_leg.damage
                } else {
                    0.0
                },
            )
        }),
        ("heelrush head;;firstaid halt for channel off", |me, you, db, strategy| (
            ComboType::ComboSecond,
            value_heelrush(LType::HeadDamage, me, you, db, strategy),
            )),
        ("heelrush torso;;firstaid halt for channel off", |me, you, db, strategy| (
            ComboType::ComboSecond,
            value_heelrush(LType::TorsoDamage, me, you, db, strategy),
            )),
        ("heelrush left arm;;firstaid halt for channel off", |me, you, db, strategy| (
            ComboType::ComboSecond,
            value_heelrush(LType::LeftArmDamage, me, you, db, strategy),
            )),
        ("heelrush right arm;;firstaid halt for channel off", |me, you, db, strategy| (
            ComboType::ComboSecond,
            value_heelrush(LType::RightArmDamage, me, you, db, strategy),
            )),
        ("heelrush left leg;;firstaid halt for channel off", |me, you, db, strategy| (
            ComboType::ComboSecond,
            value_heelrush(LType::LeftLegDamage, me, you, db, strategy),
            )),
        ("heelrush right leg;;firstaid halt for channel off", |me, you, db, strategy| (
            ComboType::ComboSecond,
            value_heelrush(LType::RightLegDamage, me, you, db, strategy),
            )),
        ("pummel left", |me, you, db, strategy| {
            let target_limb = you.get_limb_state(LType::LeftArmDamage);
            (
                ComboType::ComboAny,
                if !target_limb.damaged
                    && !target_limb.is_restoring
                    && !target_limb.is_parried
                    && !target_limb.is_dislocated
                    && can_punch(me)
                {
                    if me.is(FType::Firefist) && !you.is(FType::Ablaze) {
                        15.0
                    } else {
                        value_limb(LType::LeftArmDamage, me, you, db, strategy)
                    }
                } else {
                    0.0
                },
            )
        }),
        ("pummel right", |me, you, db, strategy| {
            let target_limb = you.get_limb_state(LType::RightArmDamage);
            (
                ComboType::ComboAny,
                if !target_limb.damaged
                    && !target_limb.is_restoring
                    && !target_limb.is_parried
                    && !target_limb.is_dislocated
                    && can_punch(me)
                {
                    if me.is(FType::Firefist) && !you.is(FType::Ablaze) {
                        15.0
                    } else {
                        value_limb(LType::RightArmDamage, me, you, db, strategy)
                    }
                } else {
                    0.0
                },
            )
        }),
        ("wanekick left", |me, you, db, strategy| {
            let target_limb = you.get_limb_state(LType::LeftLegDamage);
            (
                ComboType::ComboAny,
                if !target_limb.damaged
                    && !target_limb.is_restoring
                    && !target_limb.is_parried
                    && !target_limb.is_dislocated
                    && can_kick(me)
                {
                    value_limb(LType::LeftLegDamage, me, you, db, strategy)
                } else {
                    0.0
                },
            )
        }),
        ("wanekick right", |me, you, db, strategy| {
            let target_limb = you.get_limb_state(LType::RightLegDamage);
            (
                ComboType::ComboAny,
                if !target_limb.damaged
                    && !target_limb.is_restoring
                    && !target_limb.is_parried
                    && !target_limb.is_dislocated
                    && can_kick(me)
                {
                    value_limb(LType::RightLegDamage, me, you, db, strategy)
                } else {
                    0.0
                },
            )
        }),
        ("pummel left", |me, you, _db, _strategy| {
            let target_limb = you.get_limb_state(LType::LeftArmDamage);
            (
                ComboType::ComboAny,
                if !target_limb.damaged
                    && target_limb.hits_to_break(PUMMEL_DAMAGE) == 2
                    && !target_limb.is_restoring
                    && !target_limb.is_parried
                    && !target_limb.is_dislocated
                    && can_punch(me)
                {
                    if me.is(FType::Firefist) && !you.is(FType::Ablaze) {
                        15.0
                    } else {
                        9.0
                    }
                } else {
                    0.0
                },
            )
        }),
        ("pummel right", |me, you, _db, _strategy| {
            let target_limb = you.get_limb_state(LType::RightArmDamage);
            (
                ComboType::ComboAny,
                if !target_limb.damaged
                    && target_limb.hits_to_break(PUMMEL_DAMAGE) == 2
                    && !target_limb.is_restoring
                    && !target_limb.is_parried
                    && !target_limb.is_dislocated
                    && can_punch(me)
                {
                    if me.is(FType::Firefist) && !you.is(FType::Ablaze) {
                        15.0
                    } else {
                        9.0
                    }
                } else {
                    0.0
                },
            )
        }),
        ("wanekick left", |me, you, _db, _strategy| {
            let target_limb = you.get_limb_state(LType::LeftLegDamage);
            (
                ComboType::ComboAny,
                if !target_limb.damaged
                    && target_limb.hits_to_break(WANEKICK_DAMAGE) == 2
                    && !target_limb.is_restoring
                    && !target_limb.is_parried
                    && !target_limb.is_dislocated
                    && can_kick(me)
                {
                    10.0
                } else {
                    0.0
                },
            )
        }),
        ("wanekick right", |me, you, _db, _strategy| {
            let target_limb = you.get_limb_state(LType::RightLegDamage);
            (
                ComboType::ComboAny,
                if !target_limb.damaged
                    && target_limb.hits_to_break(WANEKICK_DAMAGE) == 2
                    && !target_limb.is_restoring
                    && !target_limb.is_parried
                    && !target_limb.is_dislocated
                    && can_kick(me)
                {
                    10.0
                } else {
                    0.0
                },
            )
        }),
        ("jawcrack", |me, you, db, _strategy| {
            (
                ComboType::ComboFirst,
                if me.get_balance(BType::Secondary) < 3.0 {
                    0.0
                } else if let Some(class) = db_class(db) {
                    if is_affected_by(&class, FType::Clumsiness) && !you.is(FType::BlurryVision) {
                        25.0
                    } else {
                        0.0
                    }
                } else {
                    0.0
                },
            )
        }),
        ("uprise", |me, you, _db, _strategy| {
            let target_limbs = you.get_limbs_state();
            (
                ComboType::ComboFirst,
                if me.get_balance(BType::Secondary) - me.get_qeb_balance() < 3.0 {
                    0.0
                } else if target_limbs.head.welt {
                    50.0
                } else {
                    0.0
                },
            )
        }),
        ("wristlash", |me, you, _db, _strategy| {
            let target_limbs = you.get_limbs_state();
            (
                ComboType::ComboFirst,
                if me.get_balance(BType::Secondary) - me.get_qeb_balance() < 3.0 {
                    0.0
                } else if target_limbs.left_arm.welt | target_limbs.right_arm.welt {
                    50.0
                } else {
                    0.0
                },
            )
        }),
        ("anklepin", |me, you, _db, strategy| {
            let target_limbs = you.get_limbs_state();
            (
                ComboType::ComboFirst,
                if me.get_balance(BType::Secondary) - me.get_qeb_balance() < 3.0 {
                    0.0
                } else if target_limbs.left_leg.welt | target_limbs.right_leg.welt {
                    50.0
                } else if !you.is(FType::SoreAnkle) {
                    if strategy.eq("bedazzle") {
                        20.0
                    } else {
                        0.0
                    }
                } else {
                    0.0
                },
            )
        }),
        ("descent", |me, you, _db, _strategy| {
            let target_limbs = you.get_limbs_state();
            (
                ComboType::ComboFirst,
                if me.get_balance(BType::Secondary) - me.get_qeb_balance() < 3.0 {
                    0.0
                } else if target_limbs.torso.welt {
                    50.0
                } else if !you.is(FType::Backstrain) && you.is(FType::Fallen) {
                    40.0
                } else {
                    0.0
                },
            )
        }),
        ("hackles {} jawcrack", |me, you, db, _strategy| {
            (
                ComboType::Hackles,
                if you.is(FType::Indifference) || me.get_balance(BType::Secondary) > 3.0 || you.is(FType::Shielded) {
                    0.0
                } else if you.limb_damage.restore_timer.is_some()
                    && !you.is(FType::BlurryVision)
                    && !you.is(FType::Stuttering)
                {
                    35.0
                } else if let Some(class) = db_class(db) {
                    if is_affected_by(&class, FType::Clumsiness) {
                        25.0
                    } else {
                        0.0
                    }
                } else {
                    0.0
                },
            )
        }),
        ("hackles {} uprise", |me, you, _db, _strategy| {
            let target_limbs = you.get_limbs_state();
            (
                ComboType::Hackles,
                if me.get_balance(BType::Secondary) > 3.0 || you.is(FType::Shielded) {
                    0.0
                } else if target_limbs.head.welt {
                    50.0
                } else if you.limb_damage.restore_timer.is_some() && !you.is(FType::Whiplash) {
                    15.0
                } else {
                    0.0
                },
            )
        }),
        ("hackles {} wristlash", |me, you, _db, _strategy| {
            let target_limbs = you.get_limbs_state();
            (
                ComboType::Hackles,
                if me.get_balance(BType::Secondary) > 3.0 || you.is(FType::Shielded) {
                    0.0
                } else if target_limbs.left_arm.welt | target_limbs.right_arm.welt {
                    50.0
                } else if you.limb_damage.restore_timer.is_some() && !you.is(FType::SoreWrist) {
                    20.0
                } else {
                    0.0
                },
            )
        }),
        ("hackles {} anklepin", |me, you, _db, _strategy| {
            let target_limbs = you.get_limbs_state();
            (
                ComboType::Hackles,
                if me.get_balance(BType::Secondary) > 3.0 || you.is(FType::Shielded) {
                    0.0
                } else if target_limbs.left_leg.welt | target_limbs.right_leg.welt {
                    50.0
                } else if you.limb_damage.restore_timer.is_some()
                    && !you.is(FType::SoreAnkle)
                    && target_limbs.torso.is_parried
                {
                    10.0
                } else {
                    0.0
                },
            )
        }),
        ("hackles {} descent", |me, you, _db, _strategy| {
            let target_limbs = you.get_limbs_state();
            (
                ComboType::Hackles,
                if me.get_balance(BType::Secondary) > 3.0 || you.is(FType::Shielded) {
                    0.0
                } else if target_limbs.torso.welt {
                    50.0
                } else if you.limb_damage.restore_timer.is_some()
                    && !you.is(FType::Backstrain)
                    && you.is(FType::Fallen)
                {
                    40.0
                } else {
                    0.0
                },
            )
        }),
        ("hackles {} whipburst", |me, you, _db, _strategy| {
            let target_limbs = you.get_limbs_state();
            (
                ComboType::Hackles,
                if me.get_balance(BType::Secondary) > 3.0 || you.is(FType::Shielded) {
                    0.0
                } else if target_limbs.torso.broken && you.is(FType::Heatspear) {
                    40.0
                } else if you.is(FType::Heatspear) {
                    15.0 + you.get_count(FType::Ablaze) as f32 * 2.0
                } else {
                    0.0
                },
            )
        }),
    ];
}

fn check_config(timeline: &AetTimeline, value: &String) -> bool {
    timeline
        .state
        .get_my_hint(value)
        .unwrap_or("false".to_string())
        .eq(&"true")
}

pub fn get_balance_attack(
    timeline: &AetTimeline,
    target: &String,
    strategy: &String,
    db: Option<&DatabaseModule>,
) -> String {
    if strategy == "damage" {
        let you = timeline.state.borrow_agent(target);
        if you.parrying == Some(LType::HeadDamage) {
            return format!("qeb flow {} clawtwist clawtwist", target);
        } else {
            return format!(
                "qeb hackles {} wristlash;;flow {} sunkick pummel left;;psi shock {}",
                target, target, target
            );
        }
    }
    let me = timeline.state.borrow_me();
    let mut you = timeline.state.borrow_agent(target);
    if !me.is(FType::Wrath) && check_config(timeline, &"PREDICT_PARRY".to_string()) {
        if let Ok(new_parry) =
            get_parry(timeline, target, &timeline.who_am_i(), &"".to_string(), db)
        {
            you.set_parrying(new_parry);
        }
    }
    let mut actions = Vec::new();
    {
        let stack = MAIN_STACK.to_vec();
        let db = db.map(|db| (db, target));
        for (action, checker) in stack.iter() {
            actions.push((*action, checker(&me, &you, db, &strategy)));
        }
    }
    actions.sort_by(|(_action, (_type, a)), (__action, (__type, b))| {
        b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal)
    });
    let mut combo: (Option<String>, Option<String>, Option<String>) = (None, None, None);
    let mut hackles: Option<String> = None;
    let mut eq: Option<String> = None;
    let mut free_act: Option<String> = None;
    let mut uses_balance = false;
    for (action, (combo_type, value)) in actions.iter() {
        if *value <= 0.0 {
            break;
        }
        match combo_type {
            ComboType::ComboAny => match (&combo, uses_balance) {
                ((None, None, None), false) => {
                    combo = (None, Some(action.to_string()), None);
                    uses_balance = true;
                }
                ((Some(first), None, None), _) => {
                    combo = (Some(first.to_string()), Some(action.to_string()), None);
                }
                ((None, None, Some(last)), _) => {
                    combo = (None, Some(action.to_string()), Some(last.to_string()));
                }
                ((None, Some(first), None), _) => {
                    combo = (Some(first.to_string()), Some(action.to_string()), None);
                }
                _ => {}
            },
            ComboType::ComboFirst => match (&combo, uses_balance) {
                ((None, None, None), false) => {
                    combo = (Some(action.to_string()), None, None);
                    uses_balance = true;
                }
                ((None, None, Some(last)), _) => {
                    combo = (Some(action.to_string()), None, Some(last.to_string()));
                }
                ((None, Some(last), None), _) => {
                    combo = (Some(action.to_string()), None, Some(last.to_string()));
                }
                _ => {}
            },
            ComboType::ComboSecond => match (&combo, uses_balance) {
                ((None, None, None), false) => {
                    combo = (None, None, Some(action.to_string()));
                    uses_balance = true;
                }
                ((Some(first), None, None), _) => {
                    combo = (Some(first.to_string()), None, Some(action.to_string()));
                }
                ((None, Some(first), None), _) => {
                    combo = (Some(first.to_string()), None, Some(action.to_string()));
                }
                _ => {}
            },
            ComboType::Hackles => match hackles {
                None => {
                    hackles = Some(action.replace("{}", target));
                }
                _ => {}
            },
            ComboType::ZenithEq => {
                if let ClassState::Zealot(ZealotClassState { zenith, .. }) = me.class_state {
                    match (&eq, zenith.active()) {
                        (None, true) => {
                            eq = Some(action.replace("{}", target));
                        }
                        _ => {}
                    }
                }
            }
            ComboType::AnyEq => match &eq {
                None => {
                    eq = Some(action.replace("{}", target));
                }
                _ => {}
            },
            ComboType::Full => match (uses_balance, &eq) {
                (false, None) => {
                    uses_balance = true;
                    eq = Some(action.replace("{}", target));
                }
                _ => {}
            },
            ComboType::Free => match free_act {
                Some(other) => {
                    free_act = Some(format!("{};;{}", other, action.replace("{}", target)));
                }
                None => {
                    free_act = Some(action.replace("{}", target));
                }
            },
        }
    }
    let combo_action = match combo {
        (Some(first), None, Some(last)) => format!("flow {} {} {};;dash d", target, first, last),
        (Some(first), Some(last), None) => format!("flow {} {} {};;dash d", target, first, last),
        (None, Some(first), Some(last)) => format!("flow {} {} {};;dash d", target, first, last),
        _ => "".to_string(),
    };
    match (free_act.as_ref(), eq.as_ref()) {
        (Some(ref free_act_str), Some(ref eq_str)) => {
            if (*eq_str).contains("scorch")
                && (free_act_str.contains("firefist") || me.is(FType::Firefist))
            {
                eq = Some(format!(
                    "enact heatspear {};;enact scorch {}",
                    target, target
                ));
            }
        }
        _ => {}
    }
    let mut full_combo = match (combo_action.as_ref(), eq) {
        ("", Some(eq)) => eq,
        (combo, Some(eq)) => format!("{};;{}", combo, eq),
        (combo, None) => combo.to_string(),
    };
    if let Some(free_act) = free_act {
        full_combo = format!("{};;{}", free_act, full_combo);
    }
    if me.is(FType::Fallen) && !full_combo.contains("risekick") {
        full_combo = format!("stand;;{}", full_combo);
    }
    if let Some(parry) = get_needed_parry(timeline, &timeline.who_am_i(), target, strategy, db) {
        full_combo = format!("fend {};;{}", parry.to_string(), full_combo);
    }
    let db_p = db.map(|db| (db, target));
    if let Some(hackles) = hackles {
        format!("qs {}%%qeb {}", hackles, full_combo)
    } else {
        format!("qeb {}", full_combo)
    }
}

pub fn get_attack(
    timeline: &AetTimeline,
    target: &String,
    strategy: &String,
    db: Option<&DatabaseModule>,
) -> String {
    let mut balance = get_balance_attack(timeline, target, strategy, db);
    let mut attack = "".to_string();
    if balance != "" {
        attack = format!("{}", balance);
    }

    attack
}

#[cfg(test)]
#[path = "./tests/zealot_tests.rs"]
mod zealot_timeline_tests;
