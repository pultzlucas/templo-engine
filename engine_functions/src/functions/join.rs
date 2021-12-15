use crate::ValueType;
use super::EngineFunction;
// Function to delete some part of string and joined the other parts.
//
// Usage:
// ````
// join(<string>, <string>)
// ````
//
// Example:
// ````
// join('Templo-tool-LGTM', '-') -> 'TemplotoolLGTM'
// ````
#[derive(Debug, PartialEq)]
pub struct Join;

impl EngineFunction for Join {
    fn call(&self, args: &Vec<String>) -> String {
        let sep = if args[1].is_empty() { " " } else { &args[1] };
        let array = split_by(&args[0], sep);
        array.join("")
    }

    fn params_type(&self) -> Option<Vec<ValueType>> {
        Some(vec![ValueType::String, ValueType::String])
    }

    fn return_type(&self) -> ValueType {
        ValueType::String
    }
}

fn split_by(string: &str, sep: &str) -> Vec<String> {
    string
        .split(sep)
        .into_iter()
        .map(|piece: &str| piece.to_string())
        .collect()
}
