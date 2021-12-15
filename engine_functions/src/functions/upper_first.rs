use crate::ValueType;
use super::EngineFunction;

// Function to transform the first char of string to uppercase.
//
// Usage:
// ````
// upper_first(<string>)
// ````
//
// Example:
// ````
// upper_first('engine') -> 'Engine'
// ````
#[derive(Debug, PartialEq)]
pub struct UpperFirst;

impl EngineFunction for UpperFirst {
    fn call(&self, args: &Vec<String>) -> String {
        let first_char: Vec<String> = args[0]
            .chars()
            .enumerate()
            .into_iter()
            .map(|(i, ch)| {
                if i == 0 {
                    return ch.to_uppercase().to_string();
                }
                ch.to_string()
            })
            .collect();
        first_char.join("")
    }

    fn params_type(&self) -> Option<Vec<ValueType>> {
        Some(vec![ValueType::String])
    }

    fn return_type(&self) -> ValueType {
        ValueType::String
    }
}