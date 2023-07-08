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
        if let Err(_) = reader.read_line(&mut buffer) {
            println!("Unable to read input, please try again.");
            continue;
        }

        match send::send_request(&mut stream, &buffer) {
            Ok(response) => {
                println!("{}", response);
            }
            Err(e) => {
                eprintln!("Error occurred communicating with server: {}", e);
            }
        }
    }
}
