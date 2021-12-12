use crate::{syntax_tree::TreeValueType, utils::string::split_by};
use std::fmt::Debug;

#[cfg(test)]
mod tests;

pub trait EngineFunction: Debug {
    fn call(&self, args: &Vec<String>) -> String;
    // fn validate_args(&self, args: &Vec<String>) -> Result<(), Error>;
    fn return_type(&self) -> TreeValueType {
        TreeValueType::Nil
    }

    fn params_type(&self) -> Option<Vec<TreeValueType>>;
}

#[derive(Debug, PartialEq)]
pub struct Upper;

#[derive(Debug, PartialEq)]
pub struct Lower;

#[derive(Debug, PartialEq)]
pub struct UpperFirst;

#[derive(Debug, PartialEq)]
pub struct Join;

#[derive(Debug, PartialEq)]
pub struct GetChar;

#[derive(Debug, PartialEq)]
pub struct GetIndex;

/// Function to transform all characters of string to uppercase.
///
/// Usage:
/// ````
/// upper(<string>)
/// ````
///
/// Example:
/// ````
/// upper('hi') -> 'HI'
/// ````
impl EngineFunction for Upper {
    fn call(&self, args: &Vec<String>) -> String {
        args[0].to_uppercase()
    }

    fn params_type(&self) -> Option<Vec<TreeValueType>> {
        Some(vec![TreeValueType::String])
    }

    fn return_type(&self) -> TreeValueType {
        TreeValueType::String
    }
}

/// Function to transform all characters of string to lowercase.
///
/// Usage:
/// ````
/// lower(<string>)
/// ````
///
/// Example:
/// ````
/// lower('TEMPLO') -> 'templo'
/// ````
impl EngineFunction for Lower {
    fn call(&self, args: &Vec<String>) -> String {
        args[0].to_lowercase()
    }

    fn params_type(&self) -> Option<Vec<TreeValueType>> {
        Some(vec![TreeValueType::String])
    }

    fn return_type(&self) -> TreeValueType {
        TreeValueType::String
    }
}

/// Function to transform the first char of string to uppercase.
///
/// Usage:
/// ````
/// upper_first(<string>)
/// ````
///
/// Example:
/// ````
/// upper_first('engine') -> 'Engine'
/// ````
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

    fn params_type(&self) -> Option<Vec<TreeValueType>> {
        Some(vec![TreeValueType::String])
    }

    fn return_type(&self) -> TreeValueType {
        TreeValueType::String
    }
}

/// Function to delete some part of string and joined the other parts.
///
/// Usage:
/// ````
/// join(<string>, <string>)
/// ````
///
/// Example:
/// ````
/// join('Templo-tool-LGTM', '-') -> 'TemplotoolLGTM'
/// ````
impl EngineFunction for Join {
    fn call(&self, args: &Vec<String>) -> String {
        let sep = if args[1].is_empty() { " " } else { &args[1] };
        let array = split_by(&args[0], sep);
        array.join("")
    }

    fn params_type(&self) -> Option<Vec<TreeValueType>> {
        Some(vec![TreeValueType::String, TreeValueType::String])
    }

    fn return_type(&self) -> TreeValueType {
        TreeValueType::String
    }
}

/// Function to get the character of index informed.
///
/// Usage:
/// ````
/// get_char(<index>, <string>)
/// ````
///
/// Example:
/// ````
/// get_char(1, 'Hello') -> 'e'
/// ````
impl EngineFunction for GetChar {
    fn call(&self, args: &Vec<String>) -> String {
        let idx = args[0].parse::<usize>().expect("Index must be a integer");
        args[1].chars().nth(idx).expect("Index exceded").to_string()
    }

    fn params_type(&self) -> Option<Vec<TreeValueType>> {
        Some(vec![TreeValueType::Integer, TreeValueType::String])
    }

    fn return_type(&self) -> TreeValueType {
        TreeValueType::String
    }
}

/// Function to get the index of a specific character of string.
/// Usage:
/// ````
/// get_index(<string>, <char>)
/// ````
///
/// Example:
/// ````
/// get_index('Rust', 'R') -> 0
/// ````
impl EngineFunction for GetIndex {
    fn call(&self, args: &Vec<String>) -> String {
        let position = args[0].chars().position(|c| c.to_string() == args[1]);

        match position {
            Some(idx) => idx.to_string(),
            None => "-1".to_string(),
        }
    }

    fn params_type(&self) -> Option<Vec<TreeValueType>> {
        Some(vec![TreeValueType::String, TreeValueType::String])
    }

    fn return_type(&self) -> TreeValueType {
        TreeValueType::Integer
    }
}
