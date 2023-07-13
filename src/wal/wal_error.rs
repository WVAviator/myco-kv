use std::io::Error;

#[derive(Debug)]
pub enum WALError {
    ReadError(Error),
    WriteError(Error),
    OpenError(Error),
}

impl WALError {
    pub fn message(&self) -> String {
        match self {
            WALError::ReadError(error) => format!("Cannot read from log file: {:?}", error),
            WALError::WriteError(error) => format!("Cannot write to log file: {:?}", error),
            WALError::OpenError(error) => format!("Cannot open log file: {:?}", error),
        }
    }
}
