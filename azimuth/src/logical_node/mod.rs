use std::collections::HashSet;

use crate::expressions::ExpressionTree;

pub mod standard_nodes;

#[derive(Clone)]
pub struct Column {
    pub alias: String,
    pub expression: ExpressionTree,
}

pub trait RelationalNode {
    fn get_relation_definition(&self, input: &Vec<RelationTree>) -> RelationDefinition;
}

pub struct RelationDefinition {
    //todo replace with data types
    pub row_type: Vec<String>,
    pub physical_properties: HashSet<String>,
}

pub struct RelationTree {
    pub node: Box<dyn RelationalNode>,
    pub children: Vec<RelationTree>,
}

impl RelationTree {
    ///return vector of types of columns
    pub fn get_row_type(&self) -> Vec<String> {
        self.node.get_relation_definition(&self.children).row_type
    }
}
