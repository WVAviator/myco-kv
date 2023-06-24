#[derive(Debug)]

pub enum ParseError {
    InvalidCommand(String),
    MissingKey,
    MissingValue,
}

impl ParseError {
    pub fn message(&self) -> String {
        match self {
            ParseError::InvalidCommand(command) => format!("Invalid command: {}", command),
            ParseError::MissingKey => String::from("Missing key"),
            ParseError::MissingValue => String::from("Missing value"),
        }
    }
}
