use std::time::SystemTime;

use crate::errors::TransactionError;

use self::{expiration::Expiration, value::Value};

pub mod expiration;
pub mod value;

#[derive(Debug, PartialEq)]
pub enum Operation {
    Get(String),
    Put(String, Value),
    Delete(String),
    ExpireAt(Expiration),
    Purge,
}

impl Operation {
    pub fn parse(command: String) -> Result<Self, TransactionError> {
        let mut parts = command.split_whitespace();

        match parts.next() {
            Some("GET") => {
                let key = parts.next().ok_or(TransactionError::MissingKey)?;
                Ok(Operation::Get(key.to_string()))
            }
            Some("PUT") => {
                let key = parts.next().ok_or(TransactionError::MissingKey)?;

                let value = parts.collect::<Vec<&str>>().join(" ");
                if value.is_empty() {
                    return Err(TransactionError::MissingValue);
                }

                let value = Value::parse(&value)?;

                Ok(Operation::Put(key.to_string(), value))
            }
            Some("DELETE") => {
                let key = parts.next().ok_or(TransactionError::MissingKey)?;
                Ok(Operation::Delete(key.to_string()))
            }
            Some("PURGE") => Ok(Operation::Purge),
            Some("EXPIREAT") => {
                let key = parts.next().ok_or(TransactionError::MissingKey)?;
                let timestamp = parts
                    .next()
                    .ok_or(TransactionError::MissingValue)?
                    .parse::<i64>()
                    .map_err(|_| TransactionError::InvalidValue("timestamp".to_string()))?;

                Ok(Operation::ExpireAt(Expiration::new(
                    key.to_string(),
                    timestamp,
                )))
            }
            Some("EXPIRE") => {
                let key = parts.next().ok_or(TransactionError::MissingKey)?;
                let duration = parts
                    .next()
                    .ok_or(TransactionError::MissingValue)?
                    .parse::<i64>()
                    .map_err(|_| TransactionError::InvalidValue("duration".to_string()))?;
                let timestamp = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as i64
                    + duration;

                Ok(Operation::ExpireAt(Expiration::new(
                    key.to_string(),
                    timestamp,
                )))
            }
            Some(other) => Err(TransactionError::UnknownCommand(other.to_string())),
            None => Err(TransactionError::MissingCommand),
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
            Operation::Put("key".to_string(), Value::String("value".to_string()))
        );
    }

    #[test]
    fn parse_put_float() {
        let test_statement = "PUT key 123.45";
        let operation = Operation::parse(test_statement.to_string()).unwrap();

        assert_eq!(
            operation,
            Operation::Put("key".to_string(), Value::Float(123.45))
        );
    }

    #[test]
    fn parse_put_integer() {
        let test_statement = "PUT key -123";
        let operation = Operation::parse(test_statement.to_string()).unwrap();

        assert_eq!(
            operation,
            Operation::Put("key".to_string(), Value::Integer(-123))
        );
    }

    #[test]
    fn parse_put_boolean() {
        let test_statement = "PUT key true";
        let operation = Operation::parse(test_statement.to_string()).unwrap();

        assert_eq!(
            operation,
            Operation::Put("key".to_string(), Value::Boolean(true))
        );
    }

    #[test]
    fn parse_put_null() {
        let test_statement = "PUT key null";
        let operation = Operation::parse(test_statement.to_string()).unwrap();

        assert_eq!(operation, Operation::Put("key".to_string(), Value::Null));
    }

    #[test]
    fn parse_put_long_value() {
        let test_statement = "PUT key \"long value with many spaces\"";
        let operation = Operation::parse(test_statement.to_string()).unwrap();

        assert_eq!(
            operation,
            Operation::Put(
                "key".to_string(),
                Value::String("long value with many spaces".to_string())
            )
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
            Err(TransactionError::UnknownCommand("INVALID".to_string()))
        );
    }

    #[test]
    fn parse_missing_key() {
        let test_statement = "PUT";
        let operation = Operation::parse(test_statement.to_string());

        assert_eq!(operation, Err(TransactionError::MissingKey));
    }

    #[test]
    fn parse_missing_value() {
        let test_statement = "PUT key";
        let operation = Operation::parse(test_statement.to_string());

        assert_eq!(operation, Err(TransactionError::MissingValue));
    }

    #[test]
    fn parse_invalid_value() {
        let test_statement = "PUT key value";
        let operation = Operation::parse(test_statement.to_string());

        assert_eq!(
            operation,
            Err(TransactionError::InvalidValue("value".to_string()))
        );
    }

    #[test]
    fn parse_purge() {
        let test_statement = "PURGE";
        let operation = Operation::parse(test_statement.to_string());
        assert_eq!(operation, Ok(Operation::Purge));
    }

    #[test]
    fn parse_expireat() {
        let test_statement = "EXPIREAT key 1234567890";
        let operation = Operation::parse(test_statement.to_string());
        assert_eq!(
            operation,
            Ok(Operation::ExpireAt(Expiration::new(
                "key".to_string(),
                1234567890
            ))),
        );
    }

    #[test]
    fn parse_expire() {
        let test_statement = "EXPIRE key 100";
        let operation = Operation::parse(test_statement.to_string());
        assert_eq!(
            operation,
            Ok(Operation::ExpireAt(Expiration::new(
                "key".to_string(),
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64
                    + 100
            ))),
        );
    }
}
