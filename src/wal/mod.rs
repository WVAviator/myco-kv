use self::wal_error::WALError;
use crate::operation::Operation;
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
};

mod wal_error;

pub struct WriteAheadLog {
    file: File,
}

impl WriteAheadLog {
    pub fn new() -> Result<WriteAheadLog, WALError> {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("log.txt")
            .map_err(|error| WALError::OpenError(error))?;

        Ok(WriteAheadLog { file })
    }

    pub fn write(&mut self, operation: &Operation) -> Result<(), WALError> {
        let output = match operation {
            // Ignore get operations since they have no affect on db state
            Operation::Get(_) => return Ok(()),

            Operation::Put(key, value) => format!("PUT {} \"{}\"\n", key, value),
            Operation::Delete(key) => format!("DELETE {}\n", key),
        };

        self.file
            .write_all(output.as_bytes())
            .map_err(|error| WALError::WriteError(error))?;

        Ok(())
    }

    pub fn read_all_lines(
        &self,
    ) -> Result<impl Iterator<Item = std::io::Result<String>>, WALError> {
        self.read_from(0)
    }

    pub fn read_from(
        &self,
        offset: usize,
    ) -> Result<impl Iterator<Item = std::io::Result<String>>, WALError> {
        let file = File::open("log.txt").map_err(|error| WALError::OpenError(error))?;
        let reader = BufReader::new(file);
        let lines = reader.lines().skip(offset);
        Ok(lines)
    }

    // pub fn restore(&self, kvmap: &mut KVMap) {
    //     let start = SystemTime::now();
    //     let mut file = File::open("log.txt").expect("Cannot open log file.");
    //     let mut contents = String::new();
    //     file.read_to_string(&mut contents)
    //         .expect("Cannot read log file.");

    //     let lines = contents.split("\n");
    //     let mut line_count = 0;
    //     for line in lines {
    //         if line.eq("") {
    //             continue;
    //         }
    //         line_count += 1;
    //         let operation = parse_operation(line).expect("Cannot parse operation from logfile.");
    //         kvmap
    //             .process_operation(operation)
    //             .expect("Cannot process operation from logfile.");
    //     }
    //     let end = SystemTime::now();
    //     println!(
    //         "Restored {} entries from log in {}ms",
    //         line_count,
    //         end.duration_since(start).unwrap().as_millis()
    //     );
    // }
}
