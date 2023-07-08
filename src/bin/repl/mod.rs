use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

mod send;

pub fn start(port: u16) {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin);

    let addr = format!("localhost:{}", port);
    let mut stream = TcpStream::connect(&addr).unwrap();

    loop {
        let mut buffer = String::new();
        reader.read_line(&mut buffer).expect("Could not read from buffer");

        match send::send_request(&mut stream, &buffer) {
            Ok(response) => {
                println!("{}", response);
            }
            Err(e) => {
                eprintln!("ERR: {}", e);
            }
        }
    }
}
