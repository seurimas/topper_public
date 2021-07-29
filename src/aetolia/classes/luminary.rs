use crate::aetolia::timeline::*;
use crate::aetolia::topper::*;
use crate::aetolia::types::*;
use crate::topper::Topper;

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
        _ => {}
    }
    Ok(())
}

pub fn get_balance_attack(topper: &mut AetTopper, target: &String, strategy: &String) -> String {
    if strategy == "damage" {
        let you = topper.get_timeline().state.borrow_agent(target);
        if you.parrying == Some(LType::HeadDamage) {
            return format!("flow {} clawtwist clawtwist", target);
        } else {
            return format!(
                "hackles {} wristlash;;flow {} sunkick pummel left;;psi shock {}",
                target, target, target
            );
        }
    } else {
        "".into()
    }
}

pub fn get_attack(topper: &mut AetTopper, target: &String, strategy: &String) -> String {
    let mut balance = get_balance_attack(topper, target, strategy);
    let mut attack = "".to_string();
    if balance != "" {
        attack = format!("qeb {}", balance);
    }

    attack
}
