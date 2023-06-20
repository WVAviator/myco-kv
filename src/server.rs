use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

const SERVER_ADDRESS: &str = "127.0.0.1:6922";

pub fn start() {
    let listener = TcpListener::bind(&SERVER_ADDRESS).unwrap();

    println!("Server listening on {}", &SERVER_ADDRESS);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);

    let mut request = String::new();
    buf_reader.read_line(&mut request).unwrap();

    let response = format!("Confirmed: {}", request);

    stream.write_all(response.as_bytes()).unwrap();
}
