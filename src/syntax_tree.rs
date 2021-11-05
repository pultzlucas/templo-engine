use serde_derive::{Deserialize, Serialize};
use std::ops::RangeFrom;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TreeType {
    FunctionCall,
    Input,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyntaxTree {
    pub node: String,
    pub tree_type: TreeType,
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

    pub fn has_childs(&self) -> bool {
        self.childs.len() > 0
    }

    pub fn get_childs(&mut self, range: RangeFrom<usize>) -> Vec<SyntaxTree> {
        self.childs.splice(range, []).collect()
    }
}
