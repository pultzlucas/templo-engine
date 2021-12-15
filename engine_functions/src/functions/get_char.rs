use crate::ValueType;
use super::EngineFunction;

// Function to get the character of index informed.
//
// Usage:
// ````
// get_char(<index>, <string>)
// ````
//
// Example:
// ````
// get_char(1, 'Hello') -> 'e'
// ````
#[derive(Debug, PartialEq)]
pub struct GetChar;

impl EngineFunction for GetChar {
    fn call(&self, args: &Vec<String>) -> String {
        let idx = args[0].parse::<usize>().expect("Index must be a integer");
        args[1].chars().nth(idx).expect("Index exceded").to_string()
    }

    fn params_type(&self) -> Option<Vec<ValueType>> {
        Some(vec![ValueType::Integer, ValueType::String])
    }

    fn return_type(&self) -> ValueType {
        ValueType::String
    }
}