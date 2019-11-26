use crate::actions::*;
use crate::alpha_beta::*;
use crate::classes::{get_venoms, AFFLICT_VENOMS};
use crate::curatives::*;
use crate::io::*;
use crate::timeline::*;
use crate::types::*;

#[cfg(test)]
mod timeline_tests {
    use super::*;

    #[test]
    fn test_dstab() {
        let mut timeline = Timeline::new();
        let dstab_slice = TimeSlice {
            incidents: vec![Incident::CombatAction(CombatAction {
                caster: "Seurimas".to_string(),
                category: "Assassination".to_string(),
                skill: "Doublestab".to_string(),
                target: "Benedicto".to_string(),
                annotation: "".to_string(),
                associated: vec![
                    Observation::Devenoms("slike".into()),
                    Observation::Devenoms("kalmia".into()),
                ],
            })],
            prompt: Prompt::Blackout,
            time: 0,
        };
        timeline.push_time_slice(dstab_slice);
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.get_flag(FType::Asthma), false);
        assert_eq!(seur_state.get_flag(FType::Anorexia), false);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Balance), true);
        assert_eq!(bene_state.get_flag(FType::Asthma), true);
        assert_eq!(bene_state.get_flag(FType::Anorexia), true);
    }

    #[test]
    fn test_dstab_rebounds() {
        let mut timeline = Timeline::new();
        let dstab_slice = TimeSlice {
            incidents: vec![Incident::CombatAction(CombatAction {
                caster: "Seurimas".to_string(),
                category: "Assassination".to_string(),
                skill: "Doublestab".to_string(),
                target: "Benedicto".to_string(),
                annotation: "".to_string(),
                associated: vec![
                    Observation::Devenoms("slike".into()),
                    Observation::Rebounds,
                    Observation::Devenoms("kalmia".into()),
                    Observation::Rebounds,
                ],
            })],
            prompt: Prompt::Blackout,
            time: 0,
        };
        timeline.push_time_slice(dstab_slice);
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.get_flag(FType::Asthma), true);
        assert_eq!(seur_state.get_flag(FType::Anorexia), true);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Balance), true);
        assert_eq!(bene_state.get_flag(FType::Asthma), false);
        assert_eq!(bene_state.get_flag(FType::Anorexia), false);
    }

    #[test]
    fn test_bite() {
        let mut timeline = Timeline::new();
        let dstab_slice = TimeSlice {
            incidents: vec![Incident::CombatAction(CombatAction {
                caster: "Seurimas".to_string(),
                category: "Assassination".to_string(),
                skill: "Bite".to_string(),
                target: "Benedicto".to_string(),
                annotation: "scytherus".to_string(),
                associated: vec![],
            })],
            prompt: Prompt::Blackout,
            time: 0,
        };
        timeline.push_time_slice(dstab_slice);
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.get_flag(FType::ThinBlood), false);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Balance), true);
        assert_eq!(bene_state.get_flag(FType::ThinBlood), true);
    }
}

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut TimelineState,
) -> Result<(), String> {
    match combat_action.skill.as_ref() {
        "Doublestab" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            let mut you = agent_states.get_agent(&combat_action.target);
            apply_observed_venoms(
                if combat_action.rebounded() {
                    &mut me
                } else {
                    &mut you
                },
                &combat_action.associated,
            )?;
            apply_or_infer_balance(&mut me, (BType::Balance, 2.8), &combat_action.associated);
            agent_states.set_agent(&combat_action.caster, me);
            agent_states.set_agent(&combat_action.target, you);
        }
        "Bite" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            let mut you = agent_states.get_agent(&combat_action.target);
            me.set_balance(BType::Balance, 1.9);
            apply_venom(&mut you, &combat_action.annotation)?;
            apply_or_infer_balance(&mut me, (BType::Balance, 1.9), &combat_action.associated);
            agent_states.set_agent(&combat_action.caster, me);
            agent_states.set_agent(&combat_action.target, you);
        }
        "Flay" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            let mut you = agent_states.get_agent(&combat_action.target);
            me.set_balance(BType::Balance, 1.9);
            apply_or_infer_balance(&mut me, (BType::Balance, 1.9), &combat_action.associated);
            apply_observed_venoms(&mut you, &combat_action.associated)?;
            if combat_action.annotation == "failure" {
                if you.is(FType::Rebounding) {
                    you.set_flag(FType::Rebounding, false);
                } else if you.is(FType::HardenedSkin) {
                    you.set_flag(FType::HardenedSkin, false);
                }
            }
            agent_states.set_agent(&combat_action.caster, me);
            agent_states.set_agent(&combat_action.target, you);
        }
        _ => {
            apply_observations(&combat_action.associated, agent_states)?;
        }
    }
    Ok(())
}

lazy_static! {
    static ref COAG_STACK: Vec<FType> = vec![
        FType::Allergies,
        FType::Vomiting,
        FType::Haemophilia,
        FType::Asthma,
        FType::Paresis,
        FType::Stuttering,
    ];
}

lazy_static! {
    static ref SOFT_STACK: Vec<FType> = vec![
        FType::Asthma,
        FType::Paresis,
        FType::Anorexia,
        FType::Slickness,
    ];
}

pub fn get_attack(topper: &Topper, target: &String, strategy: &String) -> String {
    if strategy == "Soft Coag" {
        let you = topper.timeline.state.borrow_agent(target);
        if you.is(FType::Shield) || you.is(FType::Rebounding) {
            let defense = if you.is(FType::Shield) {
                "shield"
            } else {
                "rebounding"
            };
            if let Some(venom) = get_venoms(COAG_STACK.to_vec(), 1, &you).pop() {
                return format!("qeb fl {} {}", defense, venom);
            } else {
                return format!("qeb fl {}", defense);
            }
        } else {
            let mut venoms = get_venoms(SOFT_STACK.to_vec(), 3, &you);
            if venoms.len() > 2 {
                venoms = get_venoms(COAG_STACK.to_vec(), 2, &you);
            }
            let v1 = venoms.pop();
            let v2 = venoms.pop();
            if let (Some(v1), Some(v2)) = (v1, v2) {
                return format!("qeb qds {} {}%%qs dis", v1, v2);
            } else if let Some(v1) = v1 {
                return format!("qeb qds {} {}%%qs dis", v1, "aconite");
            } else {
                return format!("qeb qds {} {}qs%%dis", "delphinium", "delphinium");
            }
        }
    } else if strategy == "Damage" {
        let you = topper.timeline.state.borrow_agent(target);
        if you.is(FType::HardenedSkin) {
            return "qeb fl hardenedskin%%qs blank".into();
        } else {
            return "qeb bitv camus%%qs blank".into();
        }
    } else {
        "touch shield".into()
    }
}
pub fn get_offensive_actions() -> Vec<StateAction> {
    let mut actions = vec![];
    // Aggro Stack
    actions.push(dstab_stack(vec![
        FType::Paresis,
        FType::Asthma,
        FType::ThinBlood,
        FType::Stupidity,
        FType::Vomiting,
        FType::Allergies,
        FType::Anorexia,
        FType::Slickness,
    ]));
    // Coag Stack
    actions.push(dstab_stack(COAG_STACK.to_vec()));
    // Salve Stack
    actions.push(dstab_stack(vec![
        FType::LeftLegBroken,
        FType::RightLegBroken,
        FType::LeftArmBroken,
        FType::RightArmBroken,
        FType::Anorexia,
        FType::Slickness,
        FType::Asthma,
    ]));
    actions
}

#[cfg(test)]
mod simulation_tests {
    use super::*;

    #[test]
    fn test_dstab_stack() {
        let salve_stack = dstab_stack(vec![
            FType::LeftLegBroken,
            FType::RightLegBroken,
            FType::LeftArmBroken,
            FType::RightArmBroken,
            FType::Anorexia,
        ]);
        let mut simulation = SimulationState::new(&vec![BASE_STATE.clone(), BASE_STATE.clone()]);
        simulation.apply_action(&salve_stack, 0, 1);
        assert_eq!(simulation.states[1].is(FType::LeftLegBroken), true);
        assert_eq!(simulation.states[1].is(FType::RightLegBroken), true);
        assert_eq!(simulation.states[1].is(FType::LeftArmBroken), false);
        assert_eq!(simulation.states[1].is(FType::RightArmBroken), false);
        assert_eq!(simulation.states[1].is(FType::Anorexia), false);
        simulation.apply_action(&salve_stack, 0, 1);
        assert_eq!(simulation.states[1].is(FType::LeftLegBroken), true);
        assert_eq!(simulation.states[1].is(FType::RightLegBroken), true);
        assert_eq!(simulation.states[1].is(FType::LeftArmBroken), true);
        assert_eq!(simulation.states[1].is(FType::RightArmBroken), true);
        assert_eq!(simulation.states[1].is(FType::Anorexia), false);
        simulation.apply_action(&salve_stack, 0, 1);
        assert_eq!(simulation.states[1].is(FType::LeftLegBroken), true);
        assert_eq!(simulation.states[1].is(FType::RightLegBroken), true);
        assert_eq!(simulation.states[1].is(FType::LeftArmBroken), true);
        assert_eq!(simulation.states[1].is(FType::RightArmBroken), true);
        assert_eq!(simulation.states[1].is(FType::Anorexia), true);
    }

    #[test]
    fn test_flay_stack() {
        let salve_stack = flay_stack(vec![
            FType::LeftLegBroken,
            FType::RightLegBroken,
            FType::LeftArmBroken,
            FType::RightArmBroken,
            FType::Anorexia,
        ]);
        let mut simulation = SimulationState::new(&vec![BASE_STATE.clone(), BASE_STATE.clone()]);
        simulation.states[1].set_flag(FType::Shield, true);
        simulation.states[1].set_flag(FType::Rebounding, true);
        simulation.apply_action(&salve_stack, 0, 1);
        assert_eq!(simulation.states[1].is(FType::LeftLegBroken), true);
        assert_eq!(simulation.states[1].is(FType::RightLegBroken), false);
        assert_eq!(simulation.states[1].is(FType::LeftArmBroken), false);
        assert_eq!(simulation.states[1].is(FType::RightArmBroken), false);
        assert_eq!(simulation.states[1].is(FType::Anorexia), false);
        assert_eq!(simulation.states[1].is(FType::Shield), false);
        assert_eq!(simulation.states[1].is(FType::Rebounding), true);
        simulation.apply_action(&salve_stack, 0, 1);
        assert_eq!(simulation.states[1].is(FType::LeftLegBroken), true);
        assert_eq!(simulation.states[1].is(FType::RightLegBroken), true);
        assert_eq!(simulation.states[1].is(FType::LeftArmBroken), false);
        assert_eq!(simulation.states[1].is(FType::RightArmBroken), false);
        assert_eq!(simulation.states[1].is(FType::Anorexia), false);
        assert_eq!(simulation.states[1].is(FType::Shield), false);
        assert_eq!(simulation.states[1].is(FType::Rebounding), false);
    }
}

pub fn dstab_stack(afflictions: Vec<FType>) -> StateAction {
    StateAction {
        name: format!("dstab {:?}", afflictions),
        changes: vec![
            balance_change(BType::Balance, 2.8),
            afflict_in_order(afflictions.clone()),
            afflict_in_order(afflictions.clone()),
        ],
        initial: vec![
            alive(),
            target(alive()),
            target(lacks(FType::Rebounding)),
            target(lacks(FType::Shield)),
            has(BType::Balance),
            has(BType::Equil),
            target(lacks_some(afflictions)),
        ],
    }
}

pub fn flay_stack(afflictions: Vec<FType>) -> StateAction {
    let flayable = vec![FType::Shield, FType::Rebounding];
    StateAction {
        name: "flay".into(),
        changes: vec![
            balance_change(BType::Balance, 2.5),
            flag_me(FType::Shield, false),
            strip_in_order(flayable.clone()),
            afflict_in_order(afflictions.clone()),
        ],
        initial: vec![
            alive(),
            target(alive()),
            has(BType::Balance),
            has(BType::Equil),
            target(some(flayable)),
        ],
    }
}

pub fn dstab_action(
    (venom1, affliction1): (String, FType),
    (venom2, affliction2): (String, FType),
) -> StateAction {
    StateAction {
        name: format!("dstab {} {}", venom1, venom2),
        changes: vec![
            balance_change(BType::Balance, 2.8),
            flag_me(FType::Shield, false),
            afflict(affliction1),
            afflict(affliction2),
        ],
        initial: vec![
            alive(),
            target(alive()),
            target(lacks(FType::Rebounding)),
            target(lacks(FType::Shield)),
            has(BType::Balance),
            has(BType::Equil),
            target(lacks(affliction1)),
            target(lacks(affliction2)),
        ],
    }
}

pub fn bite_one(affliction: FType) -> StateAction {
    StateAction {
        name: "bite".into(),
        changes: vec![
            balance_change(BType::Balance, 1.9),
            flag_me(FType::Shield, false),
            afflict(affliction),
        ],
        initial: vec![
            alive(),
            target(alive()),
            target(lacks(FType::HardenedSkin)),
            target(lacks(FType::Shield)),
            has(BType::Balance),
            has(BType::Equil),
            target(lacks(affliction)),
        ],
    }
}

pub fn flay_one(defense: FType) -> StateAction {
    StateAction {
        name: "flay".into(),
        changes: vec![
            balance_change(BType::Balance, 2.5),
            flag_me(FType::Shield, false),
            strip_in_order(vec![defense]),
        ],
        initial: vec![
            alive(),
            target(alive()),
            has(BType::Balance),
            has(BType::Equil),
            target(some(vec![defense])),
        ],
    }
}

pub fn flay_action() -> StateAction {
    let flayable = vec![FType::Shield, FType::Rebounding, FType::HardenedSkin];
    StateAction {
        name: "flay".into(),
        changes: vec![
            balance_change(BType::Balance, 2.5),
            flag_me(FType::Shield, false),
            strip_in_order(flayable.clone()),
        ],
        initial: vec![
            alive(),
            target(alive()),
            has(BType::Balance),
            has(BType::Equil),
            target(some(flayable)),
        ],
    }
}

pub fn snipe_action(affliction: FType) -> StateAction {
    StateAction {
        name: "snipe".into(),
        changes: vec![
            attack_change(900),
            balance_change(BType::Balance, 3.25),
            afflict(affliction),
        ],
        initial: vec![
            alive(),
            target(alive()),
            has(BType::Balance),
            has(BType::Equil),
            target(lacks(affliction)),
        ],
    }
}
