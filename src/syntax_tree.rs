use std::ops::RangeFrom;
#[derive(Debug, Clone, PartialEq)]
pub enum TreeType {
    FunctionCall,
    Input,
}

#[derive(Debug, Clone)]
pub struct SyntaxTree {
    pub node: String,
    pub tree_type: TreeType,
    pub childs: Vec<SyntaxTree>,
    pub tree_val_type: TreeValueType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TreeValueType {
    String,
    Integer,
    Nil
}

impl SyntaxTree {
    pub fn new(node: String, tree_type: TreeType, childs: Vec<Self>, tree_val_type: TreeValueType) -> Self {
        Self {
            childs,
            node,
            tree_type,
            tree_val_type,
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
