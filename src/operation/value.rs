use super::parse_error::ParseError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
}

impl Value {
    pub fn parse(value: &str) -> Result<Self, ParseError> {
        match value {
            "null" => Ok(Value::Null),
            "true" => Ok(Value::Boolean(true)),
            "false" => Ok(Value::Boolean(false)),
            _ => {
                if let Ok(number) = value.parse::<i64>() {
                    Ok(Value::Integer(number))
                } else if let Ok(number) = value.parse::<f64>() {
                    Ok(Value::Float(number))
                } else if value.starts_with('"') && value.ends_with('"') {
                    Ok(Value::String(value[1..value.len() - 1].to_string()))
                } else {
                    Err(ParseError::InvalidValue(value.to_string()))
                }
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Value::String(string) => string.to_string(),
            Value::Integer(number) => number.to_string(),
            Value::Float(number) => number.to_string(),
            Value::Boolean(boolean) => boolean.to_string(),
            Value::Null => "null".to_string(),
        }
    }
}
