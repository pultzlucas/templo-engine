use crate::{
    function_call::call_eng_fn,
    syntax_tree::{SyntaxTree, TreeType},
};
use std::io::Error;

pub fn generate(tree: SyntaxTree) -> Result<SyntaxTree, Error> {
    if tree.tree_type == TreeType::Input {
        Ok(tree)
    } else {
        execute_fn_tree(&tree)
    }
}

fn execute_fn_tree(tree: &SyntaxTree) -> Result<SyntaxTree, Error> {
    let mut args: Vec<SyntaxTree> = vec![];

    for child in tree.childs.iter() {
        // println!("child -> {:?}", child);
        if child.tree_type == TreeType::FunctionCall {
            let fn_result = execute_fn_tree(child)?;
            args.push(fn_result);
        }

        if child.tree_type == TreeType::Input {
            args.push(child.clone())
        }
    }

    call_eng_fn(tree.node.clone(), args)
}
