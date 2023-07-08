use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use myco_kv::{kvmap::KVMap, parser::parse_operation};

pub fn start(port: u16, kvmap: Arc<Mutex<KVMap>>) {
    let addr = format!("localhost:{}", port);

    let listener = TcpListener::bind(&addr).unwrap();

    println!("Server listening on {}", &addr);

    let mut instances = Vec::new();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let kvmap_instance = Arc::clone(&kvmap);

        instances.push(thread::spawn(move || {
            handle_connection(stream, kvmap_instance)
        }));
    }
}

fn handle_connection(mut stream: TcpStream, kvmap: Arc<Mutex<KVMap>>) {
    loop {
        let mut buf_reader = BufReader::new(&mut stream);

        let mut request = String::new();
        match buf_reader.read_line(&mut request) {
            Ok(_) => {
                let operation = parse_operation(&request);
                println!("Parsed operation: {:?}", operation);

                let response = match operation {
                    Ok(operation) => {
                        let mut kvmap = kvmap.lock().unwrap();
                        let result = kvmap.process_operation(operation);
                        match result {
                            Ok(result) => result,
                            Err(e) => e.message(),
                        }
                    }

                    Err(e) => e.message(),
                };

                let response = response + "\n";
                if let Err(e) = stream.write_all(response.as_bytes()) {
                    eprintln!("Failed to send response: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Failed to read from connection: {}", e);
                break;
            }
        }
    }
}
