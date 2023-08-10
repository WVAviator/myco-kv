use crate::{errors::TransactionError, operation::Operation};
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
};

pub struct WriteAheadLog {
    file: File,
    filename: String,
}

impl WriteAheadLog {
    pub fn new(filename: &str) -> Result<WriteAheadLog, TransactionError> {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(filename)
            .map_err(|_| TransactionError::LogLoadFail)?;

        Ok(WriteAheadLog {
            file,
            filename: filename.to_string(),
        })
    }

    pub fn write(&mut self, operation: &Operation) -> Result<(), TransactionError> {
        let output = match operation {
            // Ignore get operations since they have no affect on db state
            Operation::Get(_) => return Ok(()),

            Operation::Put(key, value) => format!("PUT {} {}\n", key, value.to_string()),
            Operation::Delete(key) => format!("DELETE {}\n", key),
            Operation::Purge => {
                self.clear()?;
                String::from("")
            }
            Operation::ExpireAt(key, timestamp) => format!("EXPIREAT {} {}\n", key, timestamp),
        };

        self.file
            .write_all(output.as_bytes())
            .map_err(|error| TransactionError::LogWriteFail(error.to_string()))?;

        Ok(())
    }

    pub fn read_all_lines(
        &self,
    ) -> Result<impl Iterator<Item = std::io::Result<String>>, TransactionError> {
        self.read_from(0)
    }

    pub fn read_from(
        &self,
        offset: usize,
    ) -> Result<impl Iterator<Item = std::io::Result<String>>, TransactionError> {
        let file = File::open(&self.filename).map_err(|_| TransactionError::LogLoadFail)?;
        let reader = BufReader::new(file);
        let lines = reader.lines().skip(offset);
        Ok(lines)
    }

    pub fn clear(&mut self) -> Result<(), TransactionError> {
        self.file = File::create(&self.filename).map_err(|_| TransactionError::LogLoadFail)?;
        Ok(())
    }
}
