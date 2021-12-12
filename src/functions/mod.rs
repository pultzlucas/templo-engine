use crate::{
    syntax_tree::TreeValueType,
    utils::{errors::invalid_input_error, string::split_by},
};
use std::{fmt::Debug, io::Error};

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

pub trait OneParamFunction {
    fn validate_args(args: &Vec<String>) -> Result<(), Error> {
        if args.len() == 0 {
            return Err(invalid_input_error("One parameter is required."));
        }
        Ok(())
    }
}
pub trait TwoParamFunction {
    fn validate_args(args: &Vec<String>) -> Result<(), Error> {
        if args.len() < 2 {
            return Err(invalid_input_error("Two parameters is required."));
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct Upper;

// #[derive(Debug, PartialEq)]
// pub struct Lower;

// #[derive(Debug, PartialEq)]
// pub struct UpperFirst;

// #[derive(Debug, PartialEq)]
// pub struct Join;

#[derive(Debug, PartialEq)]
pub struct GetChar;

#[derive(Debug, PartialEq)]
pub struct GetIndex;

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

// impl EngineFunction for Lower {
//     fn call(args: &Vec<String>) -> String {
//         args[0].to_lowercase()
//     }

//     fn return_type() -> TreeValueType {
//         TreeValueType::String
//     }
// }

// impl EngineFunction for UpperFirst {
//     fn call(args: &Vec<String>) -> String {
//         let first_char: Vec<String> = args[0]
//             .chars()
//             .enumerate()
//             .into_iter()
//             .map(|(i, ch)| {
//                 if i == 0 {
//                     return ch.to_uppercase().to_string();
//                 }
//                 ch.to_string()
//             })
//             .collect();
//         first_char.join("")
//     }

//     fn return_type() -> TreeValueType {
//         TreeValueType::String
//     }
// }

// impl EngineFunction for Join {
//     fn call(args: &Vec<String>) -> String {
//         let sep = if args[1].is_empty() { " " } else { &args[1] };
//         let array = split_by(&args[0], sep);
//         array.join("")
//     }

//     fn return_type() -> TreeValueType {
//         TreeValueType::String
//     }
// }

impl EngineFunction for GetChar {
    fn call(&self, args: &Vec<String>) -> String {
        let idx = args[1].parse::<usize>().expect("Index must be a integer");
        args[0].chars().nth(idx).expect("Index exceded").to_string()
    }

    fn params_type(&self) -> Option<Vec<TreeValueType>> {
        Some(vec![TreeValueType::String, TreeValueType::Integer])
    }

    fn return_type(&self) -> TreeValueType {
        TreeValueType::String
    }
}

impl EngineFunction for GetIndex {
    fn call(&self, args: &Vec<String>) -> String {
        let position = args[0]
            .chars()
            .position(|c| c.to_string() == args[1]);

        match position {
            Some(idx) => idx.to_string(),
            None => "-1".to_string()
        }
    }

    fn params_type(&self) -> Option<Vec<TreeValueType>> {
        Some(vec![TreeValueType::String, TreeValueType::String])
    }

    fn return_type(&self) -> TreeValueType {
        TreeValueType::Integer
    }
}
