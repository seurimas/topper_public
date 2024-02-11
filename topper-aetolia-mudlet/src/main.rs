#![allow(warnings)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate topper_derive;
extern crate strum;
#[macro_use]
extern crate strum_macros;
extern crate regex;
#[macro_use]
extern crate log;
extern crate chrono;
extern crate simplelog;
use chrono::prelude::*;
use simplelog::*;
use std::env;
use std::fs::create_dir;
use std::fs::File;
use std::io::ErrorKind;
use std::path::Path;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use topper_core_mudlet::topper::telnet::proxy;
use topper_core_mudlet::topper::Topper;

use crate::topper::battle_stats::BattleStats;
use crate::topper::AetTopper;
mod sect_parser;
mod topper;

fn main() {
    // dummy_dstab_simulation();
    // echo_time_slice();

    let utc: DateTime<Utc> = Utc::now();
    let time = utc.format("%Y-%m-%d_%H_%M_%S");
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    let log_dir = args
        .get(1)
        .map_or("log".to_string(), |string| string.to_string());
    match create_dir(&log_dir) {
        Ok(()) => {}
        Err(err) => match err.kind() {
            ErrorKind::AlreadyExists => {}
            _ => {
                println!("Could not create dir: {:?}", err);
            }
        },
    };
    let log_name = format!("{}/{}.log", log_dir, time);
    println!("Logging to: {:?}", Path::new(&log_dir).canonicalize());

    let db_dir = args
        .get(2)
        .map_or("topper.db".to_string(), |string| string.to_string());

    let triggers_dir = args
        .get(3)
        .map_or("triggers".to_string(), |string| string.to_string());

    let behavior_trees_dir = args
        .get(4)
        .map_or("behavior_trees".to_string(), |string| string.to_string());

    let stacks_dir = args
        .get(5)
        .map_or("aff_stacks".to_string(), |string| string.to_string());

    let publish_dir = args.get(6).cloned();

    WriteLogger::init(
        LevelFilter::Debug,
        Config::default(),
        File::create(log_name).unwrap(),
    )
    .unwrap();
    let (send_lines, receive_lines): (Sender<String>, Receiver<String>) = mpsc::channel();
    let t = thread::spawn(|| {
        let mut topper = AetTopper::new(
            send_lines,
            db_dir,
            triggers_dir,
            behavior_trees_dir,
            stacks_dir,
            publish_dir,
        );
        topper.provide_action();
    });
    thread::spawn(|| {
        proxy::<BattleStats>(receive_lines);
    });
    println!("{:?}", t.join());
}
