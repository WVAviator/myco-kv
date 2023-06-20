use std::io::{BufRead, BufReader};

use crate::repl::send::send_request;
mod send;

const SERVER_ADDRESS: &str = "127.0.0.1:6922";

pub fn start() {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin);

    loop {
        let mut buffer = String::new();
        reader.read_line(&mut buffer).unwrap();

        match send_request(&SERVER_ADDRESS, &buffer) {
            Ok(response) => {
                println!("Received: {}", response);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
