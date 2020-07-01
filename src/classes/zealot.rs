use crate::curatives::MENTAL_AFFLICTIONS;
use crate::timeline::*;
use crate::topper::*;
use crate::types::*;

pub fn get_preferred_parry(
    timeline: &Timeline,
    me: &String,
    target: &String,
    strategy: &String,
) -> Result<LType, String> {
    Ok(LType::TorsoDamage)
}

fn apply_combo_balance(
    agent_states: &mut TimelineState,
    caster: &String,
    expected: (BType, f32),
    after: &Vec<Observation>,
) {
    let mut me = agent_states.get_agent(caster);
    apply_or_infer_combo_balance(&mut me, expected, after);
    agent_states.set_agent(caster, me);
}

fn attack_limb_damage(
    agent_states: &mut TimelineState,
    target: &String,
    expected: (LType, f32),
    after: &Vec<Observation>,
) {
    let mut you = agent_states.get_agent(target);
    apply_limb_damage(&mut you, expected, after);
    agent_states.set_agent(target, you);
}

fn attack_afflictions(
    agent_states: &mut TimelineState,
    target: &String,
    affs: Vec<FType>,
    after: &Vec<Observation>,
) {
    if attack_hit(after) {
        let mut you = agent_states.get_agent(target);
        for aff in affs.iter() {
            you.set_flag(*aff, true);
        }
        agent_states.set_agent(target, you);
    }
}

fn for_agent(agent_states: &mut TimelineState, target: &String, act: fn(&mut AgentState)) {
    let mut you = agent_states.get_agent(target);
    act(&mut you);
    agent_states.set_agent(target, you);
}

fn for_agent_closure(
    agent_states: &mut TimelineState,
    target: &String,
    act: Box<Fn(&mut AgentState)>,
) {
    let mut you = agent_states.get_agent(target);
    act(&mut you);
    agent_states.set_agent(target, you);
}

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut TimelineState,
    _before: &Vec<Observation>,
    after: &Vec<Observation>,
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
                    println!("{:?} welt", limb);
                    you.welt(limb);
                }),
            );
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
            println!(
                "{} {:?} hit? {}",
                &combat_action.target,
                limb,
                attack_hit(after)
            );
            attack_limb_damage(agent_states, &combat_action.caster, (limb, 6.5), after);
        }
        "Dislocated" => {
            let limb = match combat_action.annotation.as_ref() {
                "left arm" => LType::LeftArmDamage,
                "right arm" => LType::RightArmDamage,
                "left leg" => LType::LeftLegDamage,
                "right leg" => LType::RightLegDamage,
                _ => LType::SIZE, // I don't want to panic
            };
        }
        "InfernalSeal" => {
            let mut you = agent_states.get_agent(&combat_action.caster);
            you.set_flag(FType::Ablaze, true);
            agent_states.set_agent(&combat_action.target, you);
        }
        "Zenith" => {
            for_agent(agent_states, &combat_action.caster, |you| {
                you.zenith_state.initiate()
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
                attack_limb_damage(agent_states, &combat_action.target, (limb, 9.5), after);
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
                attack_limb_damage(agent_states, &combat_action.target, (limb, 9.5), after);
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
                (LType::TorsoDamage, 9.5),
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
                (LType::HeadDamage, 9.5),
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
                vec![FType::Prone],
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
                you.dewelt(LType::LeftLegDamage);
                you.dewelt(LType::RightLegDamage);
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
                you.dewelt(LType::LeftArmDamage);
                you.dewelt(LType::RightArmDamage);
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
                you.dewelt(LType::TorsoDamage);
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
                you.dewelt(LType::HeadDamage);
            });
        }
        "Pendulum" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            apply_or_infer_balance(&mut me, (BType::Equil, 3.0), after);
            me.set_balance(BType::Pendulum, 10.0);
            agent_states.set_agent(&combat_action.caster, me);

            let mut you = agent_states.get_agent(&combat_action.target);
            you.rotate_limbs(combat_action.annotation == "anti-clockwise");
            agent_states.set_agent(&combat_action.target, you);
        }
        "Quicken" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            let mut you = agent_states.get_agent(&combat_action.target);
            apply_or_infer_balance(&mut me, (BType::Equil, 3.0), after);
            you.tick_flag_up(FType::Ablaze);
            you.tick_flag_up(FType::Ablaze);
            you.tick_flag_up(FType::Ablaze);
            agent_states.set_agent(&combat_action.caster, me);
            agent_states.set_agent(&combat_action.target, you);
        }
        "Infernal" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            apply_or_infer_balance(&mut me, (BType::Equil, 2.0), after);
            agent_states.set_agent(&combat_action.caster, me);
            let mut you = agent_states.get_agent(&combat_action.target);
            you.set_flag(FType::InfernalSeal, true);
            agent_states.set_agent(&combat_action.target, you);
        }
        "Scorch" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            let mut you = agent_states.get_agent(&combat_action.target);
            apply_or_infer_balance(&mut me, (BType::Equil, 2.0), after);
            you.set_flag(FType::Ablaze, true);
            agent_states.set_agent(&combat_action.caster, me);
            agent_states.set_agent(&combat_action.target, you);
        }
        "Heatspear" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            let mut you = agent_states.get_agent(&combat_action.target);
            apply_or_infer_balance(&mut me, (BType::Equil, 3.0), after);
            you.set_flag(FType::Ablaze, true);
            you.set_flag(FType::Heatspear, true);
            agent_states.set_agent(&combat_action.caster, me);
            agent_states.set_agent(&combat_action.target, you);
        }
        "Wrath" => {
            for_agent(agent_states, &combat_action.caster, |me| {
                me.set_balance(BType::Wrath, 30.0);
            });
        }
        "Recover" => {
            for_agent(agent_states, &combat_action.caster, |me| {
                me.set_balance(BType::ClassCure1, 20.0);
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
    fn(&AgentState, &AgentState, &LimbsState) -> (ComboType, f32),
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
            let timer = timer as f32 / BALANCE_SCALE;
            let timer_value = timer * 10.0;
            if let Some(shifted) = limb.rotated(!counter) {
                let damage_change = (you.limb_damage.damage[limb as usize]
                    - you.limb_damage.damage[shifted as usize])
                    as f32
                    / 100.0;
                let value = timer_value + damage_change * 2.0;
                if me.get_qeb_balance() < timer {
                    value
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

lazy_static! {
    static ref MAIN_STACK: Vec<ZealotPriority> = vec![
        ("wrath", |me, you, target_limbs| {
            (
                ComboType::Free,
                if me.balanced(BType::Wrath) { 1.0 } else { 0.0 },
            )
        }),
        ("swagger", |me, you, target_limbs| {
            (
                ComboType::Free,
                if me.is(FType::Swagger) {
                    0.0
                } else if me.get_count(FType::SappedStrength) < 5 {
                    1.0
                } else {
                    0.0
                },
            )
        }),
        ("enact immolation {}", |me, you, target_limbs| {
            (
                ComboType::Full,
                if you.get_count(FType::Ablaze) > 12 {
                    1000.0
                } else {
                    0.0
                },
            )
        }),
        ("psi recover", |me, you, target_limbs| {
            (
                ComboType::Full,
                if me.balanced(BType::ClassCure1) {
                    me.affs_count(&MENTAL_AFFLICTIONS.to_vec()) as f32 * 40.0
                } else {
                    0.0
                },
            )
        }),
        ("enact pendulum {}", |me, you, target_limbs| {
            (
                ComboType::Full,
                value_pendulum(me, you, target_limbs, false),
            )
        }),
        ("enact pendulum {} reverse", |me, you, target_limbs| {
            (ComboType::Full, value_pendulum(me, you, target_limbs, true))
        }),
        ("enact zenith", |me, you, target_limbs| {
            (
                ComboType::Full,
                if me.zenith_state.can_initiate() {
                    1000.0
                } else {
                    0.0
                },
            )
        }),
        ("enact scorch {}", |me, you, target_limbs| {
            (
                ComboType::ZenithEq,
                if !you.is(FType::Ablaze) { 100.0 } else { 0.0 },
            )
        }),
        ("enact heatspear {}", |me, you, target_limbs| {
            (
                ComboType::ZenithEq,
                if you.is(FType::Ablaze) && !you.is(FType::Heatspear) {
                    if target_limbs.torso.damage > 33.3 {
                        200.0
                    } else {
                        100.0
                    }
                } else {
                    0.0
                },
            )
        }),
        ("enact quicken {}", |me, you, target_limbs| {
            (
                ComboType::ZenithEq,
                if you.is(FType::Ablaze) && you.is(FType::Heatspear) {
                    100.0
                } else {
                    0.0
                },
            )
        }),
        ("enact infernal {}", |me, you, target_limbs| {
            (
                if me.zenith_state.active() {
                    ComboType::ZenithEq
                } else {
                    ComboType::Full
                },
                if target_limbs.torso.damage > 33.3 && !you.is(FType::InfernalSeal) {
                    1000.0
                } else {
                    0.0
                },
            )
        }),
        ("risekick", |me, you, target_limbs| {
            (
                ComboType::ComboFirst,
                if me.is(FType::Prone) && me.can_stand() {
                    100.0
                } else {
                    0.0
                },
            )
        }),
        ("twinpress", |me, you, target_limbs| {
            (
                ComboType::ComboAny,
                if !you.is(FType::MuscleSpasms) && target_limbs.restores_to_zeroes() > 1 {
                    100.0
                } else {
                    0.0
                },
            )
        }),
        ("palmforce", |me, you, target_limbs| {
            (
                ComboType::ComboAny,
                if !you.is(FType::Prone)
                    && (target_limbs.left_leg.broken || target_limbs.right_leg.broken)
                {
                    30.0
                } else {
                    0.0
                },
            )
        }),
        ("clawtwist", |me, you, target_limbs| {
            (
                ComboType::ComboAny,
                if target_limbs.torso.damage < 30.0
                    && !target_limbs.torso.is_restoring
                    && !target_limbs.torso.is_parried
                {
                    20.0
                } else if target_limbs.torso.damage > 33.3
                    && you.is(FType::Heatspear)
                    && !target_limbs.torso.is_parried
                {
                    40.0
                } else {
                    0.0
                },
            )
        }),
        ("sunkick", |me, you, target_limbs| {
            (ComboType::ComboAny, 5.0)
        }),
        ("clawtwist", |me, you, target_limbs| {
            (ComboType::ComboAny, 5.0)
        }),
        ("dislocate left arm", |me, you, target_limbs| {
            (
                ComboType::ComboAny,
                if target_limbs.restores_to_zeroes() > 1
                    && !target_limbs.left_arm.is_parried
                    && !you.is(FType::LeftArmDislocated)
                {
                    30.0 - target_limbs.left_arm.damage
                } else {
                    0.0
                },
            )
        }),
        ("dislocate right arm", |me, you, target_limbs| {
            (
                ComboType::ComboAny,
                if target_limbs.restores_to_zeroes() > 1
                    && !target_limbs.right_arm.is_parried
                    && !you.is(FType::RightArmDislocated)
                {
                    30.0 - target_limbs.right_arm.damage
                } else {
                    0.0
                },
            )
        }),
        ("dislocate left leg", |me, you, target_limbs| {
            (
                ComboType::ComboAny,
                if target_limbs.restores_to_zeroes() > 1
                    && !target_limbs.left_leg.is_parried
                    && !you.is(FType::LeftLegDislocated)
                {
                    30.0 - target_limbs.left_leg.damage
                } else {
                    0.0
                },
            )
        }),
        ("dislocate right leg", |me, you, target_limbs| {
            (
                ComboType::ComboAny,
                if target_limbs.restores_to_zeroes() > 1
                    && !target_limbs.right_leg.is_parried
                    && !you.is(FType::RightLegDislocated)
                {
                    30.0 - target_limbs.right_leg.damage
                } else {
                    0.0
                },
            )
        }),
        ("pummel left", |me, you, target_limbs| {
            (
                ComboType::ComboAny,
                if target_limbs.left_arm.damage < 33.0
                    && !target_limbs.left_arm.is_restoring
                    && !target_limbs.left_arm.is_parried
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
        ("pummel right", |me, you, target_limbs| {
            (
                ComboType::ComboAny,
                if target_limbs.right_arm.damage < 33.0
                    && !target_limbs.right_arm.is_restoring
                    && !target_limbs.right_arm.is_parried
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
        ("wanekick left", |me, you, target_limbs| {
            (
                ComboType::ComboAny,
                if target_limbs.left_leg.damage < 33.0
                    && !target_limbs.left_leg.is_restoring
                    && !target_limbs.left_leg.is_parried
                    && me.can_stand()
                {
                    10.0
                } else {
                    0.0
                },
            )
        }),
        ("wanekick right", |me, you, target_limbs| {
            (
                ComboType::ComboAny,
                if target_limbs.right_leg.damage < 33.0
                    && !target_limbs.right_leg.is_restoring
                    && !target_limbs.right_leg.is_parried
                    && me.can_stand()
                {
                    10.0
                } else {
                    0.0
                },
            )
        }),
        ("hackles {} jawcrack", |me, you, target_limbs| {
            (
                ComboType::Hackles,
                if !you.is(FType::BlurryVision) && !you.is(FType::Stuttering) {
                    35.0
                } else {
                    0.0
                },
            )
        }),
        ("hackles {} uprise", |me, you, target_limbs| {
            (
                ComboType::Hackles,
                if target_limbs.head.welt {
                    50.0
                } else if !you.is(FType::Whiplash) {
                    15.0
                } else {
                    0.0
                },
            )
        }),
        ("hackles {} wristlash", |me, you, target_limbs| {
            (
                ComboType::Hackles,
                if target_limbs.left_arm.welt | target_limbs.right_arm.welt {
                    50.0
                } else if !you.is(FType::SoreWrist) && target_limbs.torso.is_parried {
                    20.0
                } else {
                    0.0
                },
            )
        }),
        ("hackles {} anklepin", |me, you, target_limbs| {
            (
                ComboType::Hackles,
                if target_limbs.left_leg.welt | target_limbs.right_leg.welt {
                    50.0
                } else if !you.is(FType::SoreAnkle) && target_limbs.torso.is_parried {
                    10.0
                } else {
                    0.0
                },
            )
        }),
        ("hackles {} descent", |me, you, target_limbs| {
            (
                ComboType::Hackles,
                if target_limbs.torso.welt {
                    50.0
                } else if !you.is(FType::Backstrain) && you.is(FType::Prone) {
                    40.0
                } else {
                    0.0
                },
            )
        }),
        ("hackles {} whipburst", |me, you, target_limbs| {
            (
                ComboType::Hackles,
                if target_limbs.torso.broken && you.is(FType::Heatspear) {
                    40.0
                } else if you.is(FType::Heatspear) {
                    20.0
                } else {
                    0.0
                },
            )
        }),
    ];
}

pub fn get_balance_attack(timeline: &Timeline, target: &String, strategy: &String) -> String {
    if strategy == "damage" {
        let you = timeline.state.borrow_agent(target);
        if you.parrying == Some(LType::HeadDamage) {
            return format!("flow {} clawtwist clawtwist", target);
        } else {
            return format!(
                "hackles {} wristlash;;flow {} sunkick pummel left;;psi shock {}",
                target, target, target
            );
        }
    }
    let me = timeline.state.borrow_me();
    let you = timeline.state.borrow_agent(target);
    let limbs_state = you.get_limbs_state();
    let mut actions = Vec::new();
    {
        let stack = MAIN_STACK.to_vec();
        for (action, checker) in stack.iter() {
            actions.push((*action, checker(&me, &you, &limbs_state)));
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
            ComboType::ZenithEq => match (&eq, me.zenith_state.active()) {
                (None, true) => {
                    eq = Some(action.replace("{}", target));
                }
                _ => {}
            },
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
    println!("{:?}", limbs_state);
    println!("{:?} {:?} {:?}", combo, eq, hackles);
    let combo_action = match combo {
        (Some(first), None, Some(last)) => format!("flow {} {} {}", target, first, last),
        (Some(first), Some(last), None) => format!("flow {} {} {}", target, first, last),
        (None, Some(first), Some(last)) => format!("flow {} {} {}", target, first, last),
        _ => "".to_string(),
    };
    let mut full_combo = match (combo_action.as_ref(), eq) {
        ("", Some(eq)) => eq,
        (combo, Some(eq)) => format!("{};;{}", combo, eq),
        (combo, None) => combo.to_string(),
    };
    if let Some(free_act) = free_act {
        full_combo = format!("{};;{}", free_act, full_combo);
        if me.is(FType::Prone) && !full_combo.contains("risekick") {
            full_combo = format!("stand;;{}", full_combo);
        }
    }
    if let Some(hackles) = hackles {
        format!("{}%%qs {}", full_combo, hackles)
    } else {
        full_combo
    }
}

pub fn get_attack(timeline: &Timeline, target: &String, strategy: &String) -> String {
    let mut balance = get_balance_attack(timeline, target, strategy);
    let mut attack = "".to_string();
    if balance != "" {
        attack = format!("qeb {}", balance);
    }

    attack
}
