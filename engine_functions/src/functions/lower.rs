use crate::ValueType;
use super::EngineFunction;

// Function to transform all characters of string to lowercase.
//
// Usage:
// ````
// lower(<string>)
// ````
//
// Example:
// ````
// lower('TEMPLO') -> 'templo'
// ````
#[derive(Debug, PartialEq)]
pub struct Lower;

impl EngineFunction for Lower {
    fn call(&self, args: &Vec<String>) -> String {
        args[0].to_lowercase()
    }

    fn params_type(&self) -> Option<Vec<ValueType>> {
        Some(vec![ValueType::String])
    }

    fn return_type(&self) -> ValueType {
        ValueType::String
    }
}
