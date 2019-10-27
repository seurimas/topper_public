mod actions;
mod alpha_beta;
mod types;
use crate::actions::*;
use crate::alpha_beta::*;
use crate::types::*;

use std::time::Instant;

fn main() {
    let mut player = SimulationAgent::new(
        SimulationStrategy::Strict,
        vec![
            heal_action("sip".to_string(), 700).always(),
            attack_action("bit".to_string(), 1100, BType::Balance, 2.75).always(),
            shield_action("touch shield".to_string()).always(),
        ],
    );
    player.initialize_stat(SType::Health, 3600, 3600);
    player.initialize_stat(SType::Mana, 3600, 3600);
    player.initial_state.set_flag(FType::Player, true);
    let mut enemy = SimulationAgent::new(
        SimulationStrategy::Strict,
        vec![
            attack_action("mob".to_string(), 1300, BType::Balance, 3.0).always(),
            wiff_action("mob_miss".to_string(), BType::Balance, 3.0).always(),
        ],
    );
    enemy.initialize_stat(SType::Health, 8000, 8000);

    let ab_sim = ABSimulation::new(
        Box::new(|slice| {
            let me = &slice.states[0];
            let enemy = &slice.states[1];
            if !alive()(&me, &enemy) {
                i32::min_value() + slice.time
            } else if !alive()(&enemy, &me) {
                i32::max_value()
                    - me.stats[SType::Sips as usize] * 10
                    - me.stats[SType::Shields as usize] * 100
            } else {
                me.stats[SType::Health as usize] + -enemy.stats[SType::Health as usize] * 3
                    - me.stats[SType::Sips as usize] * 100
                    - me.stats[SType::Shields as usize] * 1000
            }
        }),
        Box::new(|state: &SimulationState, transitions: &Vec<Transition>| {
            if state.states[state.turn].is(FType::Player) {
                let mut can_pass = true;
                for transition in transitions.iter() {
                    if let Transition::Act(name, action_id, target) = transition {
                        if name.eq_ignore_ascii_case("touch shield")
                            || name.eq_ignore_ascii_case("bit")
                        {
                            can_pass = false;
                            break;
                        }
                    }
                }
                can_pass
            } else {
                transitions.len() == 0
            }
        }),
        Box::new(
            |state: &SimulationState, transition: &Transition| match transition {
                Transition::Act(name, action_id, target) => {
                    let me = &state.states[state.turn];
                    if name.eq_ignore_ascii_case("sip") {
                        if me.stats[SType::Health as usize]
                            > me.max_stats[SType::Health as usize] - 300
                        {
                            1
                        } else {
                            -1
                        }
                    } else if name.eq_ignore_ascii_case("touch shield") {
                        if me.stats[SType::Health as usize] < 1300 {
                            -1
                        } else {
                            1
                        }
                    } else if name.eq_ignore_ascii_case("bit") && me.is(FType::Shield) {
                        if me.stats[SType::Health as usize] < 3000 {
                            1
                        } else {
                            -1
                        }
                    } else {
                        0
                    }
                }
                _ => 0,
            },
        ),
        vec![
            vec![
                attack_action("bit".to_string(), 1100, BType::Balance, 2.75),
                shield_action("touch shield".to_string()),
                heal_action("sip".to_string(), 700),
            ],
            vec![
                attack_action("mob".to_string(), 1300, BType::Balance, 3.0),
                wiff_action("mob_miss".to_string(), BType::Balance, 3.0),
            ],
        ],
        vec![player.initial_state.clone(), enemy.initial_state.clone()],
    );
    let start = Instant::now();
    let mut stats = Stats::new();
    let best_path = ab_sim.run(1000, &mut stats);
    println!("{:?} {:?} {:?}", start.elapsed(), stats, best_path);
}
