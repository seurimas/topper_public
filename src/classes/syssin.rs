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
                ],
            })],
            prompt: Prompt::Blackout,
            time: 0,
        };
        timeline.push_time_slice(dstab_slice);
        // let simulation = timeline.get_offensive_simulation("Seurimas", "Benedicto");
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
        }
        "Bedazzle" => {}
        _ => {}
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
