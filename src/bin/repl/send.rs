use std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
};

pub fn send_request(server_address: &str, message: &str) -> Result<String, std::io::Error> {
    let mut stream = TcpStream::connect(server_address)?;

    stream.write_all(message.as_bytes())?;

    let buf_reader = BufReader::new(&mut stream);
    let response = buf_reader.lines().map(|line| line.unwrap()).collect();

    Ok(response)
}
