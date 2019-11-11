use crate::actions::*;
use crate::timeline::*;
use crate::types::*;
use std::collections::HashMap;
mod syssin;

pub fn get_offensive_actions(class: Option<&String>) -> Vec<StateAction> {
    vec![]
}

pub fn handle_combat_action(combat_action: &CombatAction, agent_states: &mut TimelineState) {
    match combat_action.category.as_ref() {
        "Subterfuge" | "Assassination" | "Hypnosis" => {
            syssin::handle_combat_action(combat_action, agent_states)
        }
        _ => {}
    }
}
