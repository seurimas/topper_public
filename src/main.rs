mod actions;
mod agents;
mod simulation;
mod types;
use crate::actions::*;
use crate::simulation::*;
use crate::types::*;

fn main() {
    let mut player = SimulationAgent::new(vec![
        heal_action("sip".to_string(), 700).always(),
        attack_action("bit".to_string(), 1100, BType::Balance, 2.7).always(),
        shield_action("touch shield".to_string()).always(),
    ]);
    player.initialize_stat(SType::Health, 3600, 3600);
    player.initialize_stat(SType::Mana, 3600, 3600);
    let mut enemy = SimulationAgent::new(vec![attack_action(
        "mob".to_string(),
        1300,
        BType::Balance,
        4.0,
    )
    .always()]);
    enemy.initialize_stat(SType::Health, 8000, 8000);
    let simulation_slice = AgentSimulationSlice {
        entrance: "Test".to_string(),
        time: 0,
        my_turn: true,
        me_state: player.initial_state.clone(),
        enemy_state: enemy.initial_state.clone(),
    };
    println!(
        "{:?}",
        simulation_slice.next_state(&player.actions, &enemy.actions, false)
    );

    let mut simulation = AgentSimulation::new(player, enemy);
    simulation.next_till((BALANCE_SCALE * 10.0) as i32, false);
    println!("{:?}", simulation.root,);
}
