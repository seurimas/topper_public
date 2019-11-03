use crate::timeline::*;
use std::io;

pub fn echo_time_slice() {
    let mut input = String::new();
    while true {
        match io::stdin().read_line(&mut input) {
            Ok(n) => println!("{:?}", parse_time_slice(&input)),
            Err(error) => println!("error: {}", error),
        }
    }
}
