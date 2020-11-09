#![allow(warnings)]
#[macro_use]
extern crate lazy_static;
extern crate strum;
#[macro_use]
extern crate strum_macros;
extern crate regex;
#[macro_use]
extern crate log;
extern crate chrono;
extern crate simplelog;
mod aetolia;
mod timeline;
mod topper;
use crate::topper::provide_action;
use crate::topper::telnet::proxy;
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
                println!("{:?}", err);
            }
        },
    };

    let log_name = format!("{}/{}.log", log_dir, time);
    println!("Logging to: {:?}", Path::new(&log_dir).canonicalize());

    WriteLogger::init(
        LevelFilter::Debug,
        Config::default(),
        File::create(log_name).unwrap(),
    )
    .unwrap();
    let (send_lines, receive_lines): (Sender<String>, Receiver<String>) = mpsc::channel();
    let t = thread::spawn(|| {
        provide_action(send_lines);
    });
    thread::spawn(|| {
        proxy(receive_lines);
    });
    println!("{:?}", t.join());
}
