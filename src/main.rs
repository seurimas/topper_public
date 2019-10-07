mod actions;
mod agents;
mod simulation;
mod types;
use crate::actions::*;
use crate::simulation::*;
use crate::types::*;

fn main() {
    let mut player = SimulationAgent::new(vec![
        heal_action("sip", 700).always(),
        attack_action("bit", 1100, BType::Balance, 2.7).always(),
        shield_action("touch shield").always(),
    ]);
    player.initialize_stat(SType::Health, 3600, 3600);
    player.initialize_stat(SType::Mana, 3600, 3600);
    let mut enemy = SimulationAgent::new(vec![
        attack_action("mob", 1300, BType::Balance, 4.0).always()
    ]);
    enemy.initialize_stat(SType::Health, 8000, 8000);
    let simulation_slice = AgentSimulationSlice {
        time: 0,
        me_state: player.initial_state.clone(),
        enemy_state: enemy.initial_state.clone(),
    };
    println!(
        "{:?}",
        simulation_slice.next_state(&player.actions, &enemy.actions)
    );

    let mut simulation = AgentSimulation::new(player, enemy);
    simulation.next_till(100);
    println!("{:?}", simulation,);
}
