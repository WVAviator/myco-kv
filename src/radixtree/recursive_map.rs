use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::operation::value::Value;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum RecursiveMap {
    Map(HashMap<String, RecursiveMap>),
    Value(Value),
}

impl RecursiveMap {
    pub fn new() -> Self {
        RecursiveMap::Map(HashMap::new())
    }
    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
    pub fn to_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self)
    }
}

#[cfg(test)]
mod test {
    use assert_json_diff::assert_json_eq;
    use serde_json::json;

    use super::*;

    #[test]
    fn properly_serializes_to_json() {
        let mut cupboard_contents = HashMap::new();
        cupboard_contents.insert(
            String::from("cups"),
            RecursiveMap::Value(Value::String("3".to_string())),
        );
        cupboard_contents.insert(
            String::from("plates"),
            RecursiveMap::Value(Value::String("4".to_string())),
        );

        let mut refrigerator_contents = HashMap::new();
        refrigerator_contents.insert(
            String::from("milk"),
            RecursiveMap::Value(Value::String("1".to_string())),
        );
        refrigerator_contents.insert(
            String::from("eggs"),
            RecursiveMap::Value(Value::String("12".to_string())),
        );

        let mut kitchen_contents = HashMap::new();
        kitchen_contents.insert(
            String::from("cupboard"),
            RecursiveMap::Map(cupboard_contents),
        );
        kitchen_contents.insert(
            String::from("refrigerator"),
            RecursiveMap::Map(refrigerator_contents),
        );

        let mut house_contents = HashMap::new();
        house_contents.insert(String::from("kitchen"), RecursiveMap::Map(kitchen_contents));

        let expected = json!(
          {
            "kitchen": {
              "cupboard": {
                "cups": "3",
                "plates": "4"
              },
              "refrigerator": {
                "milk": "1",
                "eggs": "12"
              }
            }
          }
        );

        let actual = RecursiveMap::Map(house_contents).to_json().unwrap();
        println!("{}", actual);
        // let actual = serde_json::to_value(result).unwrap();

        assert_json_eq!(expected, actual);
    }
}
