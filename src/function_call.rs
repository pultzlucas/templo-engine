use crate::functions::{EngineFunction, GetChar, GetIndex, Upper};
use crate::syntax_tree::{SyntaxTree, TreeType, TreeValueType};
use crate::utils::errors::invalid_input_error;
use std::io::Error;

#[derive(Debug)]
pub struct FunctionCall {
    pub function: String,
    pub args: Vec<SyntaxTree>,
}

impl FunctionCall {
    pub fn call(&self) -> Result<SyntaxTree, Error> {
        let mut fn_args_value: Vec<String> = vec![];
        let mut fn_args_type: Vec<TreeValueType> = vec![];
        for arg in self.args.clone().into_iter() {
            fn_args_type.push(arg.tree_val_type);
            fn_args_value.push(arg.node)
        }

        let exec_fn = |function: &dyn EngineFunction| {
            if let Some(params_type) = function.params_type() {
                check_fn_args_type(fn_args_type, params_type);
            }

            create_output_obj(function.call(&fn_args_value), function.return_type())
        };

        let res = match self.function.to_lowercase().as_str() {
            "upper" => exec_fn(&Upper),
            // "lower" => {
            //     Lower::validate_args(&self.args)?;
            //     Lower::call(&self.args)
            // }
            // "upper_first" => {
            //     UpperFirst::validate_args(&self.args)?;
            //     UpperFirst::call(&self.args)
            // }
            // "join" => {
            //     Join::validate_args(&self.args)?;
            //     Join::call(&self.args)
            // }
            // "get_char" => exec_fn(&GetChar),
            "get_index" => exec_fn(&GetIndex),
            // "str" => Str::call(&self.args),
            _ => return Err(invalid_input_error("Wrong engine function.")),
        };

        Ok(res)
    }
}

fn check_fn_args_type(inputs: Vec<TreeValueType>, fn_args: Vec<TreeValueType>) {
    fn_args
        .into_iter()
        .zip(inputs.into_iter())
        .for_each(|(fn_arg, input)| {
            if fn_arg != input {
                panic!(
                    "[Incorrect input type] expected '{:?}', received '{:?}'.",
                    fn_arg, input
                );
            }
        })
}

fn create_output_obj(node: String, tree_val_type: TreeValueType) -> SyntaxTree {
    SyntaxTree {
        childs: vec![],
        node,
        tree_type: TreeType::Input,
        tree_val_type,
    }
}

// #[derive(Debug, PartialEq)]
// enum EngineFunction {
//     Upper(Upper),
//     Lower(Lower),
//     UpperFirst(UpperFirst),
//     Join(Join),
//     GetChar(GetChar),
//     Str(Str),
// }

pub fn get_fn_obj(fn_name: &str) -> Box<dyn EngineFunction> {
    match fn_name.to_lowercase().as_str() {
        "upper" => Box::new(Upper),
        // "lower" => Lower,
        // "upper_first" => UpperFirst,
        // "join" => Join,
        "get_char" => Box::new(GetChar),
        "get_index" => Box::new(GetIndex),
        // "str" => Str,
        _ => panic!("Wrong engine function."),
    }
}

#[cfg(test)]
mod test {
    // use super::*;

    // #[test]
    // fn function_call_test1() {
    //     let fc = FunctionCall {
    //         function: "upper".to_string(),
    //         args: vec!["Templo".to_string()],
    //     };
    //     assert_eq!(fc.call().unwrap(), "TEMPLO".to_string());
    // }

    // #[test]
    // fn function_call_test2() {
    //     let fc = FunctionCall {
    //         function: "join".to_string(),
    //         args: vec!["Templo Moon".to_string(), " ".to_string()],
    //     };
    //     assert_eq!(fc.call().unwrap(), "TemploMoon".to_string());
    // }

    // #[test]
    // fn function_call_test3() {
    //     let fc = FunctionCall {
    //         function: "lower".to_string(),
    //         args: vec![],
    //     };
    //     assert!(fc.call().is_err());
    // }

    // #[test]
    // fn function_call_test4() {
    //     let fc = FunctionCall {
    //         function: "lower".to_string(),
    //         args: vec!["Templo Sun".to_string()],
    //     };
    //     assert_eq!(fc.call().unwrap(), "templo sun".to_string());
    // }

    // #[test]
    // fn function_call_test5() {
    //     let lower = FunctionCall {
    //         function: "lower".to_string(),
    //         args: vec!["Templo Sun".to_string()],
    //     };

    //     let join = FunctionCall {
    //         function: "join".to_string(),
    //         args: vec![lower.call().unwrap(), " ".to_string()],
    //     };

    //     let upper_first = FunctionCall {
    //         function: "upper_first".to_string(),
    //         args: vec![join.call().unwrap()],
    //     };

    //     let final_result = upper_first.call().unwrap();

    //     assert_eq!(final_result, "Templosun".to_string());
    // }
}
