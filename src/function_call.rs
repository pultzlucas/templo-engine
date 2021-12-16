use crate::syntax_tree::{SyntaxTree, TreeType};
use engine_functions::{
    EngineFunction, GetChar, GetIndex, Join, Lower, Upper, UpperFirst, ValueType,
};
use std::io::Error;

pub fn call_eng_fn(function_name: String, args: Vec<SyntaxTree>) -> Result<SyntaxTree, Error> {
    let mut fn_args_value: Vec<String> = vec![];
    let mut fn_args_type: Vec<ValueType> = vec![];
    for arg in args.clone().into_iter() {
        fn_args_type.push(arg.tree_val_type);
        fn_args_value.push(arg.node)
    }

    let function_obj = get_fn_obj(&function_name);

    if args.len() > function_obj.params_quant() {
        panic!(
            "[ParamsQuantExceeded] Function '{}' expected '{}' parameters, received '{}'",
            function_name,
            function_obj.params_quant(),
            args.len()
        );
    }

    if let Some(params_type) = function_obj.params_type() {
        check_fn_args_type(fn_args_type, params_type);
    }

    let fn_output = create_output_obj(
        function_obj.call(&fn_args_value),
        function_obj.return_type(),
    );

    Ok(fn_output)
}

pub fn get_fn_obj(fn_name: &str) -> Box<dyn EngineFunction> {
    match fn_name.to_lowercase().as_str() {
        "upper" => Box::new(Upper),
        "lower" => Box::new(Lower),
        "upper_first" => Box::new(UpperFirst),
        "join" => Box::new(Join),
        "get_char" => Box::new(GetChar),
        "get_index" => Box::new(GetIndex),
        _ => panic!("Wrong engine function."),
    }
}

fn check_fn_args_type(inputs: Vec<ValueType>, fn_args: Vec<ValueType>) {
    fn_args
        .into_iter()
        .zip(inputs.into_iter())
        .for_each(|(fn_arg, input)| {
            if fn_arg != input {
                panic!(
                    "[InvalidInputType] Expected '{:?}', received '{:?}'.",
                    fn_arg, input
                );
            }
        })
}

fn create_output_obj(node: String, tree_val_type: ValueType) -> SyntaxTree {
    SyntaxTree {
        childs: vec![],
        node,
        tree_type: TreeType::Input,
        tree_val_type,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn function_call_test1() {
        let args = vec![SyntaxTree {
            childs: vec![],
            node: String::from("Templo"),
            tree_type: TreeType::Input,
            tree_val_type: ValueType::String,
        }];

        assert_eq!(
            call_eng_fn(String::from("upper"), args).unwrap(),
            SyntaxTree {
                childs: vec![],
                node: String::from("TEMPLO"),
                tree_type: TreeType::Input,
                tree_val_type: ValueType::String
            }
        );
    }

    #[test]
    fn function_call_test2() {
        let args = vec![
            SyntaxTree {
                childs: vec![],
                node: String::from("Templo Moon"),
                tree_type: TreeType::Input,
                tree_val_type: ValueType::String,
            },
            SyntaxTree {
                childs: vec![],
                node: String::from(" "),
                tree_type: TreeType::Input,
                tree_val_type: ValueType::String,
            },
        ];

        assert_eq!(
            call_eng_fn(String::from("join"), args).unwrap(),
            SyntaxTree {
                childs: vec![],
                node: String::from("TemploMoon"),
                tree_type: TreeType::Input,
                tree_val_type: ValueType::String,
            },
        );
    }

    #[test]
    #[should_panic]
    fn function_call_test3() {
        assert!(call_eng_fn(String::from("lower"), vec![]).is_err());
    }

    #[test]
    fn function_call_test4() {
        let args = vec![SyntaxTree {
            childs: vec![],
            node: String::from("Templo Sun"),
            tree_type: TreeType::Input,
            tree_val_type: ValueType::String,
        }];

        assert_eq!(
            call_eng_fn(String::from("lower"), args).unwrap(),
            SyntaxTree {
                childs: vec![],
                node: String::from("templo sun"),
                tree_type: TreeType::Input,
                tree_val_type: ValueType::String,
            }
        );
    }

    #[test]
    fn function_call_test5() {
        let input = "Templo Sun";
        let input_tree = SyntaxTree {
            childs: vec![],
            node: input.to_string(),
            tree_type: TreeType::Input,
            tree_val_type: ValueType::String,
        };

        let lower_res = call_eng_fn(String::from("lower"), vec![input_tree]).unwrap();
        let join_res = call_eng_fn(
            String::from("join"),
            vec![
                lower_res,
                SyntaxTree {
                    childs: vec![],
                    node: String::from(" "),
                    tree_type: TreeType::Input,
                    tree_val_type: ValueType::String,
                },
            ],
        )
        .unwrap();

        let final_result = call_eng_fn(String::from("upper_first"), vec![join_res]).unwrap();

        assert_eq!(
            final_result,
            SyntaxTree {
                childs: vec![],
                node: String::from("Templosun"),
                tree_type: TreeType::Input,
                tree_val_type: ValueType::String,
            },
        );
    }
}
