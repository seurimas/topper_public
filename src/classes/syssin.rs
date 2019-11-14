use crate::actions::*;
use crate::curatives::*;
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
                    Observation::Afflicts(FType::Anorexia),
                    Observation::Afflicts(FType::BrokenLeftArm),
                    Observation::Balance(BType::Balance, 2.8),
                ],
            })],
            prompt: Prompt::Blackout,
            time: 0,
        };
        timeline.push_time_slice(dstab_slice);
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.get_flag(FType::BrokenLeftArm), false);
        assert_eq!(seur_state.get_flag(FType::Anorexia), false);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Balance), true);
        assert_eq!(bene_state.get_flag(FType::BrokenLeftArm), true);
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
                    Observation::Afflicts(FType::Anorexia),
                    Observation::Rebounds,
                    Observation::Afflicts(FType::BrokenLeftArm),
                    Observation::Rebounds,
                    Observation::Balance(BType::Balance, 2.8),
                ],
            })],
            prompt: Prompt::Blackout,
            time: 0,
        };
        timeline.push_time_slice(dstab_slice);
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.get_flag(FType::BrokenLeftArm), true);
        assert_eq!(seur_state.get_flag(FType::Anorexia), true);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Balance), true);
        assert_eq!(bene_state.get_flag(FType::BrokenLeftArm), false);
        assert_eq!(bene_state.get_flag(FType::Anorexia), false);
    }
}

pub fn handle_combat_action(combat_action: &CombatAction, agent_states: &mut TimelineState) {
    match combat_action.skill.as_ref() {
        "Doublestab" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            let mut you = agent_states.get_agent(&combat_action.target);
            me.set_balance(BType::Balance, 2.8);
            apply_observed_afflictions(
                if combat_action.rebounded() {
                    &mut me
                } else {
                    &mut you
                },
                2,
                &combat_action.associated,
            );
            apply_or_infer_balance(&mut me, (BType::Balance, 2.8), &combat_action.associated);
            agent_states.set_agent(&combat_action.caster, me);
            agent_states.set_agent(&combat_action.target, you);
        }
        "Bedazzle" => {}
        _ => {}
    }
}

const EPTETH_A: (&'static str, FType) = ("epteth", FType::BrokenLeftLeg);
const EPTETH_B: (&'static str, FType) = ("epteth", FType::BrokenRightLeg);
const EPSETH_A: (&'static str, FType) = ("epseth", FType::BrokenLeftArm);
const EPSETH_B: (&'static str, FType) = ("epseth", FType::BrokenRightArm);
const SLIKE: (&'static str, FType) = ("slike", FType::Anorexia);
const KALMIA: (&'static str, FType) = ("kalmia", FType::Slickness);
const JALK: (&'static str, FType) = ("jalk", FType::Asthma);

const VENOMS: [(&'static str, FType); 7] =
    [EPTETH_A, EPTETH_B, EPSETH_A, EPSETH_B, SLIKE, KALMIA, JALK];

pub fn get_offensive_actions() -> Vec<StateAction> {
    let mut actions = vec![];
    for venom_a in VENOMS.iter() {
        for venom_b in VENOMS.iter() {
            if venom_a != venom_b {
                actions.push(dstab_action(
                    (venom_a.0.to_string(), venom_a.1),
                    (venom_b.0.to_string(), venom_b.1),
                ));
            }
        }
    }
    actions
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
