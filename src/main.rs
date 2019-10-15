mod actions;
mod agents;
mod simulation;
mod types;
use crate::actions::*;
use crate::simulation::*;
use crate::types::*;

fn main() {
    let mut player = SimulationAgent::new(
        SimulationStrategy::Maximin,
        vec![
            heal_action("sip".to_string(), 700).always(),
            attack_action("bit".to_string(), 1100, BType::Balance, 2.7).always(),
            shield_action("touch shield".to_string()).always(),
        ],
    );
    player.initialize_stat(SType::Health, 3600, 3600);
    player.initialize_stat(SType::Mana, 3600, 3600);
    let mut enemy = SimulationAgent::new(
        SimulationStrategy::Strict,
        vec![attack_action("mob".to_string(), 1300, BType::Balance, 4.0).always()],
    );
    enemy.initialize_stat(SType::Health, 8000, 8000);
    let mut simulation = AgentSimulation::new();
    simulation.add_ally(player);
    simulation.add_enemy(enemy);
    let sim_node = simulation.next_till((BALANCE_SCALE * 10.0) as i32);
    println!("{}", sim_node.size());
    println!(
        "{:?}",
        simulation.best_path(
            &|slice| {
                let me = &slice.states[0];
                let enemy = &slice.states[1];
                if !alive()(&me, &enemy) {
                    i32::min_value()
                } else if !alive()(&enemy, &me) {
                    i32::max_value() - slice.time
                } else {
                    me.stats[SType::Health as usize] - enemy.stats[SType::Health as usize]
                }
            },
            &sim_node
        )
    );
}
