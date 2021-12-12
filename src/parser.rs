use std::vec;

use regex::Regex;

use crate::{
    function_call::get_fn_obj,
    syntax_tree::{SyntaxTree, TreeType, TreeValueType},
    token::{Token, TokenType},
    EngineArg, EngineArgType,
};

#[derive(Debug)]
struct FnArg {
    layer: usize,
    args: usize,
}

pub fn parse(tokens: Vec<Token>, inputs: &Vec<EngineArg>) -> SyntaxTree {
    if tokens[0].type_ == TokenType::Input {
        parse_input_exp(&tokens[0], inputs)
    } else {
        parse_fn_exp(tokens, inputs)
    }
}

pub fn parse_input_exp(token: &Token, inputs: &Vec<EngineArg>) -> SyntaxTree {
    let input_value = inputs.iter().find(|input| input.key == token.value);
    if input_value.is_none() && inputs.len() > 0 {
        panic!("{}", format!("Input '{}' is not informed.", token.value));
    }
    SyntaxTree {
        childs: vec![],
        node: input_value.unwrap().value.clone(),
        tree_type: TreeType::Input,
        tree_val_type: TreeValueType::String,
    }
}

pub fn parse_fn_exp(tokens: Vec<Token>, inputs: &Vec<EngineArg>) -> SyntaxTree {
    let tokens = tokens[..tokens.len() - 1].to_vec();
    let mut tokens_iter = tokens.iter();
    let mut tree = SyntaxTree::new(
        tokens_iter.next().unwrap().value.clone(),
        TreeType::FunctionCall,
        vec![],
        TreeValueType::String,
    );

    let mut layer = 0;
    let mut layer_args: Vec<FnArg> = vec![FnArg { layer: 0, args: 0 }];

    for token in tokens_iter.rev() {
        if token.type_ == TokenType::Function {
            let args = layer_args[layer].args;
            let range = (tree.childs.len() - args)..;
            let mut args: Vec<SyntaxTree> = tree.get_childs(range);
            args.reverse();

            let fn_obj = get_fn_obj(&token.value);
            let fun = SyntaxTree::new(
                token.value.clone(),
                TreeType::FunctionCall,
                args,
                fn_obj.return_type(),
            );

            tree.append_child(fun);
            layer -= 1;
            layer_args[layer].args += 1;
        }

        if token.type_ == TokenType::Input {
            let input_value = inputs.into_iter().find(|input| input.key == token.value);

            if input_value.is_none() && inputs.len() > 0 {
                panic!("{}", format!("Input '{}' is not informed.", token.value));
            }

            let input_value = input_value.unwrap();

            let input_type = match input_value.value_type {
                EngineArgType::String => TreeValueType::String,
                EngineArgType::Integer => TreeValueType::Integer,
            };

            let input = SyntaxTree::new(
                input_value.value.clone(),
                TreeType::Input,
                vec![],
                input_type,
            );
            tree.append_child(input);
            layer_args[layer].args += 1;
        }

        if token.type_ == TokenType::Integer {
            let input = SyntaxTree::new(
                token.value.clone(),
                TreeType::Input,
                vec![],
                TreeValueType::Integer,
            );
            tree.append_child(input);
            layer_args[layer].args += 1;
        }

        if token.type_ == TokenType::String {
            let value = &token.value[1..token.value.len() - 1];
            let middle_quotes_reg = Regex::new(r"\\'").unwrap();
            let real_string_value = middle_quotes_reg.replace_all(value, "'");

            let input = SyntaxTree::new(
                real_string_value.to_string(),
                TreeType::Input,
                vec![],
                TreeValueType::String,
            );

            tree.append_child(input);
            layer_args[layer].args += 1;
        }

        if token.type_ == TokenType::BracketRight {
            layer += 1;

            if layer_args.get(layer).is_none() {
                layer_args.push(FnArg { args: 0, layer })
            } else {
                layer_args[layer] = FnArg { args: 0, layer }
            }
        }
    }

    tree.childs.reverse();
    tree
}
