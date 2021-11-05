use std::vec;

use regex::Regex;

use crate::{
    syntax_tree::{SyntaxTree, TreeType},
    token::{Token, TokenType},
    Input,
};

#[derive(Debug)]
struct Arg {
    layer: usize,
    args: usize,
}

pub fn parse(tokens: Vec<Token>, inputs: Vec<Input>) -> SyntaxTree {
    if tokens[0].type_ == TokenType::Input {
        parse_input_exp(&tokens[0], inputs)
    } else {
        parse_fn_exp(tokens, inputs)
    }
}

pub fn parse_input_exp(token: &Token, inputs: Vec<Input>) -> SyntaxTree {
    let input_value = inputs.iter().find(|input| input.key == token.value);
    if input_value.is_none() && inputs.len() > 0 {
        panic!("{}", format!("Input '{}' is not informed.", token.value));
    }
    SyntaxTree {
        childs: vec![],
        node: input_value.unwrap().value.clone(),
        tree_type: TreeType::Input
    }
}

pub fn parse_fn_exp(tokens: Vec<Token>, inputs: Vec<Input>) -> SyntaxTree {
    let tokens = tokens[..tokens.len() - 1].to_vec();
    let mut tokens_iter = tokens.iter();
    let mut tree = SyntaxTree::new(
        tokens_iter.next().unwrap().value.clone(),
        TreeType::FunctionCall,
        vec![],
    );

    let mut layer = 0;
    let mut layer_args: Vec<Arg> = vec![Arg { layer: 0, args: 0 }];

    for token in tokens_iter.rev() {
        if token.type_ == TokenType::Function {
            let args = layer_args[layer].args;
            let range = (tree.childs.len() - args)..;
            let mut args: Vec<SyntaxTree> = tree.get_childs(range);
            args.reverse();

            let fun = SyntaxTree::new(token.value.clone(), TreeType::FunctionCall, args);
            tree.append_child(fun);

            layer -= 1;
            layer_args[layer].args += 1;
        }

        if token.type_ == TokenType::Input {
            let input_value = inputs.iter().find(|input| input.key == token.value);

            if input_value.is_none() && inputs.len() > 0 {
                panic!("{}", format!("Input '{}' is not informed.", token.value));
            }

            let input =
                SyntaxTree::new(input_value.unwrap().value.clone(), TreeType::Input, vec![]);
            tree.append_child(input);
            layer_args[layer].args += 1;
        }

        if token.type_ == TokenType::String {
            let value = &token.value[1..token.value.len() - 1];
            let middle_quotes_reg = Regex::new(r"\\'").unwrap();
            let real_string_value = middle_quotes_reg.replace_all(value, "'");

            let input = SyntaxTree::new(real_string_value.to_string(), TreeType::Input, vec![]);
            // println!("{:?}", input);

            tree.append_child(input);
            layer_args[layer].args += 1;
        }

        if token.type_ == TokenType::BracketRight {
            layer += 1;

            if layer_args.get(layer).is_none() {
                layer_args.push(Arg { args: 0, layer })
            } else {
                layer_args[layer] = Arg { args: 0, layer }
            }
        }
    }

    tree.childs.reverse();
    tree
}
