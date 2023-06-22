use std::io::{BufRead, BufReader};

use crate::repl::send::send_request;
mod send;

pub fn start(port: u16) {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin);

    let addr = format!("localhost:{}", port);

    loop {
        let mut buffer = String::new();
        reader.read_line(&mut buffer).unwrap();

        match send_request(&addr, &buffer) {
            Ok(response) => {
                println!("Received: {}", response);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
