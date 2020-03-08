use crate::io::*;
use crate::timeline::*;
use crate::types::*;

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut TimelineState,
    _before: &Vec<Observation>,
    after: &Vec<Observation>,
) -> Result<(), String> {
    match combat_action.skill.as_ref() {
        "Pendulum" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            let mut you = agent_states.get_agent(&combat_action.target);
            apply_or_infer_balance(&mut me, (BType::Equil, 3.0), after);
            you.rotate_limbs(combat_action.annotation == "anti-clockwise");
            agent_states.set_agent(&combat_action.caster, me);
            agent_states.set_agent(&combat_action.target, you);
        }
        _ => {}
    }
    Ok(())
}

pub fn get_balance_attack(topper: &Topper, target: &String, strategy: &String) -> String {
    if strategy == "damage" {
        let you = topper.timeline.state.borrow_agent(target);
        if you.parrying == Some(LType::HeadDamage) {
            return format!("flow {} clawtwist clawtwist", target);
        } else {
            return format!("flow {} sunkick uprise;;psi shock {}", target, target);
        }
    } else {
        "".into()
    }
}

pub fn get_attack(topper: &Topper, target: &String, strategy: &String) -> String {
    let mut balance = get_balance_attack(topper, target, strategy);
    let mut attack = "".to_string();
    if balance != "" {
        attack = format!("qeb {}", balance);
    }

    attack
}
