use crate::ValueType;
use super::EngineFunction;

// Function to get the index of a specific character of string.
// Usage:
// ````
// get_index(<string>, <char>)
// ````
//
// Example:
// ````
// get_index('Rust', 'R') -> 0
// ````
#[derive(Debug, PartialEq)]
pub struct GetIndex;

impl EngineFunction for GetIndex {
    fn call(&self, args: &Vec<String>) -> String {
        let position = args[0].chars().position(|c| c.to_string() == args[1]);

        match position {
            Some(idx) => idx.to_string(),
            None => "-1".to_string(),
        }
    }

    fn params_type(&self) -> Option<Vec<ValueType>> {
        Some(vec![ValueType::String, ValueType::String])
    }

    fn return_type(&self) -> ValueType {
        ValueType::Integer
    }
}
