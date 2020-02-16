#[macro_use]
extern crate lazy_static;
extern crate strum;
#[macro_use]
extern crate strum_macros;
extern crate regex;
mod actions;
mod alpha_beta;
mod battle_stats;
mod classes;
mod curatives;
mod io;
mod observables;
mod timeline;
mod types;
use crate::io::provide_action;

fn main() {
    // dummy_dstab_simulation();
    // echo_time_slice();
    provide_action();
}
