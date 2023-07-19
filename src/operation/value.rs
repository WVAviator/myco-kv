use super::parse_error::ParseError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    String(String),
    Number(f64),
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
                if let Ok(number) = value.parse::<f64>() {
                    Ok(Value::Number(number))
                } else if value.starts_with('"') && value.ends_with('"') {
                    Ok(Value::String(value[1..value.len() - 1].to_string()))
                } else {
                    Err(ParseError::InvalidValue(value.to_string()))
                }
            }
        }
    }
}
