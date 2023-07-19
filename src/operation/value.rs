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

#[cfg(test)]
mod test {
    
    use super::*;
    
    #[test]
    fn parses_string() {
        assert_eq!(Value::parse(r#""hello""#), Ok(Value::String("hello".to_string())));
    }

    #[test]
    fn parses_integer() {
        assert_eq!(Value::parse("123"), Ok(Value::Integer(123)));
    }

    #[test]
    fn parses_float() {
        assert_eq!(Value::parse("123.456"), Ok(Value::Float(123.456)));
    }

    #[test]
    fn parses_boolean() {
        assert_eq!(Value::parse("true"), Ok(Value::Boolean(true)));
        assert_eq!(Value::parse("false"), Ok(Value::Boolean(false)));
    }

    #[test]
    fn parses_null() {
        assert_eq!(Value::parse("null"), Ok(Value::Null));
    }

    #[test]
    fn parses_invalid_value() {
        assert_eq!(Value::parse("hello"), Err(ParseError::InvalidValue("hello".to_string())));
    }

    #[test]
    fn converts_string_to_string() {
        assert_eq!("hello".to_string(), Value::String("hello".to_string()).to_string());
    }

    #[test]
    fn converts_float_to_string() {
        assert_eq!("123.45", Value::Float(123.45).to_string());
    }

    #[test]
    fn converts_null_to_string() {
        assert_eq!("null", Value::Null.to_string());
    }

}
