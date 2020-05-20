use crate::topper::{send_response, TopperMessage, TopperModule, TopperResponse};
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

pub struct TelnetModule {
    send_lines: Sender<String>,
}

impl TelnetModule {
    pub fn new(send_lines: Sender<String>) -> Self {
        TelnetModule { send_lines }
    }
}

impl TopperModule for TelnetModule {
    fn handle_message(&mut self, message: &TopperMessage) -> Result<TopperResponse, String> {
        match message {
            TopperMessage::Event(timeslice) => {
                for (line, _line_number) in timeslice.lines.iter() {
                    match self.send_lines.send(line.to_string()) {
                        Ok(()) => {}
                        Err(err) => {
                            println!("Line: {:?}", err);
                        }
                    };
                }
                Ok(TopperResponse::silent())
            }
            _ => Ok(TopperResponse::silent()),
        }
    }
}

pub fn proxy(receive_lines: Receiver<String>) {
    println!("Starting proxy!");
    let listener = TcpListener::bind("0.0.0.0:12323").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Found connection!");

        println!(
            "Connection: {:?}",
            handle_connection(stream, &receive_lines)
        );
    }
}

const ACCESS_CODE: &str = "I'm the best around! I really am.";

fn handle_connection(mut stream: TcpStream, receive_lines: &Receiver<String>) -> Result<(), Error> {
    if stream.write(&"Enter Access Code:".as_bytes()).is_err() {
        println!("Failed to write Prompt.");
        stream.shutdown(Shutdown::Both)?;
        return Ok(());
    }
    let mut buffer = [0; 4096];
    stream.set_read_timeout(Some(Duration::new(2, 0)))?;
    if let Ok(size) = stream.read(&mut buffer) {
        if !String::from_utf8_lossy(&buffer[..size])
            .to_string()
            .starts_with(ACCESS_CODE)
        {
            stream.write(&"Incorrect.".as_bytes())?;
            println!("{}", String::from_utf8_lossy(&buffer[..size]));
            stream.flush()?;
            stream.shutdown(Shutdown::Both)?;
            return Ok(());
        } else {
            stream.write(&"Welcome!\n".as_bytes())?;
            stream.flush()?;
        }
    } else {
        stream.write(&"Too slow!".as_bytes())?;
        println!("SLOW");
        return Ok(());
    }
    if let Err(err) = stream.set_nonblocking(true) {
        println!("Failed to set stream to non-blocking: {}", err);
        return Ok(());
    };
    let mut buffer = [0; 4096];
    let mut inactive = 0;
    loop {
        while let Ok(line) = receive_lines.try_recv() {
            inactive = 0;
            match stream.write(&line.as_bytes()) {
                Ok(_) => {}
                Err(err) => match err.kind() {
                    ErrorKind::WouldBlock => {}
                    other => {
                        println!("{:?}", other);
                        break;
                    }
                },
            }
        }
        match stream.read(&mut buffer) {
            Ok(size) => {
                if size > 0 {
                    inactive = 0;
                    send_response(&TopperResponse::qeb(
                        String::from_utf8_lossy(&buffer[..size]).to_string(),
                    ));
                } else {
                    println!("EOF");
                    break;
                }
            }
            Err(err) => match err.kind() {
                ErrorKind::ConnectionAborted => {
                    break;
                }
                ErrorKind::WouldBlock => {}
                other => {
                    println!("{:?}", other);
                    break;
                }
            },
        }
        inactive += 1;
        if inactive % 1000 == 0 {
            stream.write("\x1B[38;5;33m.".as_bytes())?;
        }
        thread::sleep_ms(10);
    }
    Ok(())
}
