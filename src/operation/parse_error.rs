#[derive(Debug, PartialEq)]

pub enum ParseError {
    InvalidCommand(String),
    MissingKey,
    MissingValue,
    InvalidValue(String),
}

impl ParseError {
    pub fn message(&self) -> String {
        match self {
            ParseError::InvalidCommand(command) => format!("Invalid command: {}", command),
            ParseError::MissingKey => String::from("Missing key"),
            ParseError::MissingValue => String::from("Missing value"),
            ParseError::InvalidValue(value) => format!("Invalid value: {}", value),
        }
    }
}
