use std::collections::HashSet;

use crate::expressions::ExpressionTree;
use chrono::{TimeZone, Utc};

pub mod standard_nodes;

#[derive(Clone)]
pub struct Column {
    pub alias: String,
    pub expression: ExpressionTree,
}

pub trait RelationalNode {
    fn get_inputs(&self) -> Vec<&dyn RelationalNode>;
    fn get_relation_definition(&self) -> RelationDefinition;
}

pub struct RelationDefinition {
    columns: Vec<Column>,
    physical_properties: Vec<String>,
}
