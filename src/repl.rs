use std::io::{BufRead, BufReader};

pub fn start() {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin);

    loop {
        let mut buffer = String::new();
        reader.read_line(&mut buffer).unwrap();

        println!("Command: {}", buffer);
    }
}
