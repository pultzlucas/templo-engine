use serde_derive::{Deserialize, Serialize};
use std::ops::RangeFrom;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TreeType {
    FunctionCall,
    Input,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyntaxTree {
    node: String,
    tree_type: TreeType,
    pub childs: Vec<SyntaxTree>,
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

    pub fn get_childs(&mut self, range: RangeFrom<usize>) -> Vec<SyntaxTree> {
        self.childs.splice(range, []).collect()
    }
}
