use std::vec;

use crate::{
    syntax_tree::{SyntaxTree, TreeType},
    token::{Token, TokenType},
};

#[derive(Debug)]
struct Arg {
    layer: usize,
    args: usize,
}

pub fn parse(tokens: Vec<Token>) -> SyntaxTree {
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
            // println!("-- {} --", token.value);
            // println!("layer -> {}", layer);
            // println!("layer_args -> {:?}", layer_args);
            // println!("{:?}\n", tree);
            
            let args = layer_args[layer].args;
            let range = (tree.childs.len() - args)..;
            let mut args: Vec<SyntaxTree> = tree.get_childs(range);
            args.reverse();
            
            let fun = SyntaxTree::new(token.value.clone(), TreeType::FunctionCall, args);
            tree.append_child(fun);
            
            // println!("{:?}\n", tree);
            
            layer -= 1;
            layer_args[layer].args += 1;
        }

        if token.type_ == TokenType::Input {
            let input = SyntaxTree::new(token.value.clone(), TreeType::Input, vec![]);
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
