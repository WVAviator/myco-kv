use crate::operation::Operation;
use crate::parser::parse_error::ParseError;

pub mod parse_error;

pub fn parse_operation(command: &str) -> Result<Operation, ParseError> {
    let mut parts = command.split_whitespace();

    match parts.next() {
        Some("GET") => {
            let key = parts.next().ok_or(ParseError::MissingKey)?;
            Ok(Operation::Get(key.to_string()))
        }
        Some("PUT") => {
            let key = parts.next().ok_or(ParseError::MissingKey)?;
            let value = parts.next().ok_or(ParseError::MissingValue)?;
            Ok(Operation::Put(key.to_string(), value.to_string()))
        }
        Some("DELETE") => {
            let key = parts.next().ok_or(ParseError::MissingKey)?;
            Ok(Operation::Delete(key.to_string()))
        }
        _ => Err(ParseError::InvalidCommand(command.to_string())),
    }
}
