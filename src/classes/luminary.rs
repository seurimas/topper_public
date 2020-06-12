use crate::timeline::*;
use crate::topper::*;
use crate::types::*;

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut TimelineState,
    _before: &Vec<Observation>,
    after: &Vec<Observation>,
) -> Result<(), String> {
    match combat_action.skill.as_ref() {
        "Aura" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            apply_or_infer_balance(&mut me, (BType::Equil, 3.0), after);
            agent_states.set_agent(&combat_action.caster, me);
            let mut affected = if combat_action.target == "" {
                &combat_action.caster
            } else {
                &combat_action.target
            };
            let mut you = agent_states.get_agent(affected);
            you.set_flag(FType::Shielded, true);
            agent_states.set_agent(affected, you);
        }
        _ => {}
    }
    Ok(())
}

pub fn get_balance_attack(topper: &mut Topper, target: &String, strategy: &String) -> String {
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

pub fn get_attack(topper: &mut Topper, target: &String, strategy: &String) -> String {
    let mut balance = get_balance_attack(topper, target, strategy);
    let mut attack = "".to_string();
    if balance != "" {
        attack = format!("qeb {}", balance);
    }

    attack
}
