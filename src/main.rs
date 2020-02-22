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
mod telnet;
mod timeline;
mod types;
use crate::io::provide_action;
use crate::telnet::proxy;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

fn main() {
    // dummy_dstab_simulation();
    // echo_time_slice();
    let (send_lines, receive_lines): (Sender<String>, Receiver<String>) = mpsc::channel();
    let t = thread::spawn(|| {
        provide_action(send_lines);
    });
    thread::spawn(|| {
        proxy(receive_lines);
    });
    println!("{:?}", t.join());
}
