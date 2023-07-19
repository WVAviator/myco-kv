use self::parse_error::ParseError;

pub mod parse_error;
pub mod value;

#[derive(Debug, PartialEq)]
pub enum Operation {
    Get(String),
    Put(String, String),
    Delete(String),
}

impl Operation {
    pub fn parse(command: String) -> Result<Self, ParseError> {
        let mut parts = command.split_whitespace();

        match parts.next() {
            Some("GET") => {
                let key = parts.next().ok_or(ParseError::MissingKey)?;
                Ok(Operation::Get(key.to_string()))
            }
            Some("PUT") => {
                let key = parts.next().ok_or(ParseError::MissingKey)?;

                let value = parts.collect::<Vec<&str>>().join(" ");
                if value.is_empty() {
                    return Err(ParseError::MissingValue);
                }
                if value.chars().next() != Some('"') || value.chars().last() != Some('"') {
                    return Err(ParseError::InvalidValue(value));
                }
                let value = value.trim_matches('"');

                Ok(Operation::Put(key.to_string(), value.to_string()))
            }
            Some("DELETE") => {
                let key = parts.next().ok_or(ParseError::MissingKey)?;
                Ok(Operation::Delete(key.to_string()))
            }
            _ => Err(ParseError::InvalidCommand(command.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_get_operation() {
        let test_statement = "GET key";
        let operation = Operation::parse(test_statement.to_string()).unwrap();

        assert_eq!(operation, Operation::Get("key".to_string()));
    }

    #[test]
    fn parse_put_operation() {
        let test_statement = "PUT key \"value\"";
        let operation = Operation::parse(test_statement.to_string()).unwrap();

        assert_eq!(
            operation,
            Operation::Put("key".to_string(), "value".to_string())
        );
    }

    #[test]
    fn parse_put_long_value() {
        let test_statement = "PUT key \"long value with many spaces\"";
        let operation = Operation::parse(test_statement.to_string()).unwrap();

        assert_eq!(
            operation,
            Operation::Put("key".to_string(), "long value with many spaces".to_string())
        );
    }

    #[test]
    fn parse_delete_operation() {
        let test_statement = "DELETE key";
        let operation = Operation::parse(test_statement.to_string()).unwrap();

        assert_eq!(operation, Operation::Delete("key".to_string()));
    }

    #[test]
    fn parse_invalid_operation() {
        let test_statement = "INVALID key";
        let operation = Operation::parse(test_statement.to_string());

        assert_eq!(
            operation,
            Err(ParseError::InvalidCommand("INVALID key".to_string()))
        );
    }

    #[test]
    fn parse_missing_key() {
        let test_statement = "PUT";
        let operation = Operation::parse(test_statement.to_string());

        assert_eq!(operation, Err(ParseError::MissingKey));
    }

    #[test]
    fn parse_missing_value() {
        let test_statement = "PUT key";
        let operation = Operation::parse(test_statement.to_string());

        assert_eq!(operation, Err(ParseError::MissingValue));
    }

    #[test]
    fn parse_invalid_value() {
        let test_statement = "PUT key value";
        let operation = Operation::parse(test_statement.to_string());

        assert_eq!(
            operation,
            Err(ParseError::InvalidValue("value".to_string()))
        );
    }
}
