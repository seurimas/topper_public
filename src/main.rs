mod agents;
mod types;
use crate::agents::*;
use crate::types::*;
use std::collections::HashMap;

fn main() {
    let mut player = MainAgent::new(vec![
        healAction(700).always(),
        attackAction(1100, BType::Balance, 2.7).always(),
        shieldAction().always(),
    ]);
    player.initialize_stat(SType::Health, 3600, 3600);
    player.initialize_stat(SType::Mana, 3600, 3600);
    let mut enemy = MainAgent::new(vec![attackAction(800, BType::Balance, 4.0).always()]);
    enemy.initialize_stat(SType::Health, 8000, 8000);
    let simulation = AgentSimulation {
        time: 0,
        me_state: player.initial_state.clone(),
        enemy_state: enemy.initial_state.clone(),
    };
    println!(
        "{:?}",
        simulation.next_state(&player.actions, &enemy.actions)
    );
}
