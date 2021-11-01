use std::vec;

use crate::{
    syntax_tree::{SyntaxTree, TreeType},
    token::{Token, TokenType},
};

#[derive(Debug)]
struct Brother {
    layer: usize,
    brothers: usize,
}

pub fn parse(tokens: Vec<Token>) -> SyntaxTree {
    let tokens = tokens[..tokens.len() - 1].to_vec();
    let mut tokens_iter = tokens.iter();
    let mut tree = SyntaxTree::new(
        tokens_iter.next().unwrap().value.clone(),
        TreeType::FunctionCall,
        vec![],
    );

    let mut n_of_args = 0;
    let mut layer = 0;
    let mut last_layer = 0;
    let mut brothers: Vec<Brother> = vec![Brother {
        layer: 0,
        brothers: 0,
    }];
    let mut last_token = &Token::new("\0".to_string(), TokenType::Null);

    for token in tokens_iter.rev() {
        if token.type_ == TokenType::Function {
            last_layer = layer;
            layer -= 1;

            if brothers[layer].layer == layer {
                brothers.push(Brother { brothers: 0, layer });
            }

            n_of_args -= brothers[layer].brothers;
            let range = (tree.childs.len() - n_of_args)..;
            let mut args: Vec<SyntaxTree> = tree.get_childs(range);
            args.reverse();

            let fun = SyntaxTree::new(token.value.clone(), TreeType::FunctionCall, args);
            tree.append_child(fun);

            n_of_args = brothers[layer].brothers;
            n_of_args += 1;
        }

        if token.type_ == TokenType::Input {
            let input = SyntaxTree::new(token.value.clone(), TreeType::Input, vec![]);
            tree.append_child(input);

            if last_token.type_ == TokenType::Comma {
                brothers[layer].brothers += 1;
            } else {
                brothers.push(Brother { brothers: 0, layer });
            }

            n_of_args += 1;
        }

        if token.type_ == TokenType::BracketRight {
            if last_layer != layer {
                n_of_args = 0;
            }

            if last_token.type_ == TokenType::Comma {
                brothers[layer].brothers += 1;
            } else {
                brothers.push(Brother { brothers: 0, layer });
            }

            last_layer = layer;
            layer += 1;
        }

        if token.type_ == TokenType::Comma {
            last_layer = layer;
        }

        last_token = token;
    }

    tree.childs.reverse();
    tree
}
