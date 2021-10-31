use crate::token::{Token, TokenType};

#[derive(Debug, Clone)]
pub enum TreeType {
    FunctionCall,
    Input,
    Root,
}

#[derive(Debug, Clone)]
pub struct SyntaxTree {
    node: String,
    tree_type: TreeType,
    childs: Vec<SyntaxTree>,
}

impl SyntaxTree {
    pub fn new(node: String, tree_type: TreeType, childs: Vec<Self>) -> Self {
        Self {
            childs,
            node,
            tree_type,
        }
    }

    pub fn append_child(&mut self, child: SyntaxTree) -> &mut Self {
        self.childs.push(child);
        self
    }
}

// I -> a-z|A-Z|0-9
// Arg -> Fn|I|Fn,|I,
// Fn -> I(Arg*)

pub fn build_tree(tokens: Vec<Token>) {
    let mut tokens_iter = tokens.iter();
    let root = tokens_iter.next().unwrap();
    let mut tree: SyntaxTree = SyntaxTree::new(root.value.clone(), TreeType::FunctionCall, vec![]);

    let mut n_of_args = 0;
    let mut _memory: Box<SyntaxTree>;

    for (_, token) in tokens_iter.rev().enumerate() {
        if token.type_ == TokenType::BracketRight {
            n_of_args = 0;
        }

        if token.type_ == TokenType::Function {
            n_of_args = 1;
            // println!("----- {} -----", token.value);
            // println!("range -> {:?}", n_of_args..);
            // println!("{:?}\n", tree.childs);
            let mut args: Vec<SyntaxTree> = tree.childs.splice(n_of_args.., []).collect();
            args.reverse();
            let fun = SyntaxTree::new(token.value.clone(), TreeType::FunctionCall, args);
            tree.append_child(fun);
            println!("{:?}", tree.childs);
        }

        if token.type_ == TokenType::Input {
            n_of_args += 1;
            let input = SyntaxTree::new(token.value.clone(), TreeType::Input, vec![]);
            tree.append_child(input);
        }
    }
    tree.childs.reverse();
    println!("####\n{:?}\n", tree);
}
