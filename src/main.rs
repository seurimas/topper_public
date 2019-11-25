#[macro_use]
extern crate lazy_static;
extern crate strum;
#[macro_use]
extern crate strum_macros;
mod actions;
mod alpha_beta;
mod battle_stats;
mod classes;
mod curatives;
mod io;
mod timeline;
mod types;
use crate::actions::*;
use crate::alpha_beta::*;
use crate::classes::syssin::get_offensive_actions;
use crate::curatives::*;
use crate::io::{echo_time_slice, provide_action};
use crate::timeline::*;
use crate::types::*;

use std::time::Instant;

fn main() {
    // dummy_dstab_simulation();
    // echo_time_slice();
    provide_action();
}

fn dummy_simulation() {
    let mut player = AgentState::default();
    player.initialize_stat(SType::Health, 5000);
    player.initialize_stat(SType::Mana, 5000);
    player.set_flag(FType::Player, true);
    let mut enemy = AgentState::default();
    enemy.initialize_stat(SType::Health, 8000);
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
        vec![player.clone(), enemy.clone()],
    );
    let start = Instant::now();
    let mut stats = Stats::new();
    let best_path = ab_sim.run(1000, &mut stats);
    println!("{:?} {:?} {:?}", start.elapsed(), stats, best_path);
}

fn dummy_dstab_simulation() {
    let mut player = AgentState::default();
    player.initialize_stat(SType::Health, 5000);
    player.initialize_stat(SType::Mana, 5000);
    player.set_flag(FType::Player, true);
    player.set_flag(FType::Ally, true);
    let mut enemy = AgentState::default();
    enemy.initialize_stat(SType::Health, 8000);
    enemy.set_flag(FType::Player, true);
    let ab_sim = ABSimulation::new(
        Box::new(|slice| {
            let me = &slice.states[0];
            let enemy: &AgentState = &slice.states[1];
            if enemy.is(FType::Anorexia) && enemy.is(FType::Slickness) && enemy.is(FType::Asthma) {
                i32::max_value() - slice.time
            } else {
                enemy.affliction_count() * 1000
            }
        }),
        Box::new(|state: &SimulationState, transitions: &Vec<Transition>| transitions.len() == 0),
        Box::new(|state: &SimulationState, transition: &Transition| 0),
        vec![get_offensive_actions(), get_curative_actions()],
        vec![player.clone(), enemy.clone()],
    );
    let start = Instant::now();
    let mut stats = Stats::new();
    let best_path = ab_sim.run(500, &mut stats);
    println!("{:?} {:?} {:?}", start.elapsed(), stats, best_path);
}
