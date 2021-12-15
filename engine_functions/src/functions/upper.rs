use crate::ValueType;
use super::EngineFunction;

// Function to transform all characters of string to uppercase.
//
// Usage:
// ````
// upper(<string>)
// ````
//
// Example:
// ````
// upper('hi') -> 'HI'
// ````
#[derive(Debug, PartialEq)]
pub struct Upper;

impl EngineFunction for Upper {
    fn call(&self, args: &Vec<String>) -> String {
        args[0].to_uppercase()
    }

    fn params_type(&self) -> Option<Vec<ValueType>> {
        Some(vec![ValueType::String])
    }

    fn return_type(&self) -> ValueType {
        ValueType::String
    }
}