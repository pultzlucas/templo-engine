use crate::{
    syntax_tree::{SyntaxTree, TreeType},
    token::{Token, TokenType},
};

pub fn parse(tokens: Vec<Token>) -> SyntaxTree {
    let mut tokens_iter = tokens.iter();
    let mut tree: SyntaxTree = SyntaxTree::new(
        tokens_iter.next().unwrap().value.clone(),
        TreeType::FunctionCall,
        vec![],
    );
    let mut n_of_args = 0;

    for token in tokens_iter.rev() {
        if token.type_ == TokenType::BracketRight {
            n_of_args = 0;
        }

        if token.type_ == TokenType::Function {
            n_of_args = 1;
            let mut args: Vec<SyntaxTree> = tree.get_child(n_of_args..);
            args.reverse();
            let fun = SyntaxTree::new(token.value.clone(), TreeType::FunctionCall, args);
            tree.append_child(fun);
        }

        if token.type_ == TokenType::Input {
            n_of_args += 1;
            let input = SyntaxTree::new(token.value.clone(), TreeType::Input, vec![]);
            tree.append_child(input);
        }
    }
    tree.childs.reverse();
    tree
}
