use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use crate::lib::operation::Operation;
use crate::lib::parser::parse_operation;

pub fn start(port: u16) {
    let addr = format!("localhost:{}", port);

    let listener = TcpListener::bind(&addr).unwrap();

    println!("Server listening on {}", &addr);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);

    let mut request = String::new();
    buf_reader.read_line(&mut request).unwrap();

    let operation = parse_operation(&request);
    println!("Processed operation: {:?}", operation);

    let response = match operation {
        Ok(Operation::Get(key)) => format!("Got key: {}", key),
        Ok(Operation::Put(key, value)) => format!("Put key: {}, value: {}", key, value),
        Ok(Operation::Delete(key)) => format!("Deleted key: {}", key),
        Err(e) => format!("Error: {}", e.message()),
    };

    stream.write_all(response.as_bytes()).unwrap();
}
