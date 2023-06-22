use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

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

    let response = format!("Confirmed: {}", request);

    stream.write_all(response.as_bytes()).unwrap();
}
