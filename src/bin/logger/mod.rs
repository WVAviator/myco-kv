use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    time::SystemTime,
};

use myco_kv::{
    eventbroker::{event::Event, subscriber::Subscriber},
    kvmap::KVMap,
    parser::parse_operation,
};

pub struct Logger {
    file: File,
}

impl Logger {
    pub fn new() -> Logger {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("log.txt")
            .expect("Cannot open log file.");

        Logger { file }
    }

    pub fn restore(&self, kvmap: &mut KVMap) {
        let start = SystemTime::now();
        let mut file = File::open("log.txt").expect("Cannot open log file.");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Cannot read log file.");

        let lines = contents.split("\n");
        let mut line_count = 0;
        for line in lines {
            if line.eq("") {
                continue;
            }
            line_count += 1;
            let operation = parse_operation(line).expect("Cannot parse operation from logfile.");
            kvmap
                .process_operation(operation)
                .expect("Cannot process operation from logfile.");
        }
        let end = SystemTime::now();
        println!(
            "Restored {} entries from log in {}ms",
            line_count,
            end.duration_since(start).unwrap().as_millis()
        );
    }
}

impl Subscriber for Logger {
    fn notify(&mut self, event: &Event) {
        match event {
            Event::Put { key, value } => {
                let output = format!("PUT {} \"{}\"\n", key, value);
                self.file.write(output.as_bytes()).unwrap();
            }
            Event::Delete { key } => {
                let output = format!("DELETE {}\n", key);
                self.file.write(output.as_bytes()).unwrap();
            }
            _ => {}
        }
    }
}
