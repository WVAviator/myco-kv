use std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
};

pub fn send_request(stream: &mut TcpStream, message: &str) -> Result<String, std::io::Error> {
    stream.write_all(message.as_bytes())?;

    let mut buf_reader = BufReader::new(stream);
    let mut response = String::new();

    buf_reader.read_line(&mut response)?;

    Ok(response)
}
