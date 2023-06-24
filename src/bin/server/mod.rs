use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use myco_kv::{kvmap::KVMap, parser::parse_operation};

pub fn start(port: u16, kvmap: &mut KVMap) {
    let addr = format!("localhost:{}", port);

    let listener = TcpListener::bind(&addr).unwrap();

    println!("Server listening on {}", &addr);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, kvmap);
    }
}

fn handle_connection(mut stream: TcpStream, kvmap: &mut KVMap) {
    let mut buf_reader = BufReader::new(&mut stream);

    let mut request = String::new();
    buf_reader.read_line(&mut request).unwrap();

    let operation = parse_operation(&request);
    println!("Processed operation: {:?}", operation);

    let response = match operation {
        Ok(operation) => {
            let result = kvmap.process_operation(operation);
            match result {
                Ok(result) => result,
                Err(e) => e.message(),
            }
        }
        Err(e) => e.message(),
    };

    stream.write_all(response.as_bytes()).unwrap();
}
