use crate::{
    function_call::FunctionCall,
    syntax_tree::{SyntaxTree, TreeType},
};
use std::io::Error;

pub fn generate(tree: SyntaxTree) -> Result<String, Error> {
    if tree.tree_type == TreeType::Input {
        Ok(tree.node)
    } else {
        execute_fn_tree(&tree)
    }
}

fn execute_fn_tree(tree: &SyntaxTree) -> Result<String, Error> {
    let mut args: Vec<String> = vec![];

    for child in tree.childs.iter() {
        // println!("child -> {:?}", child);
        if child.tree_type == TreeType::FunctionCall {
            let fn_result = execute_fn_tree(child)?;
            args.push(fn_result);
        }

        if child.tree_type == TreeType::Input {
            args.push(child.node.clone())
        }
    }

    let function = FunctionCall {
        function: tree.node.clone(),
        args,
    };

    // println!("{:?}", function);

    function.call()
}
