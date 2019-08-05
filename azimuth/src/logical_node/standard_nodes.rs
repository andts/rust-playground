use crate::logical_node::{Column, RelationDefinition, RelationalNode};

pub struct Scan {
    row_type: [Column],
    //source
}

impl RelationalNode for Scan {
    fn get_inputs(&self) -> Vec<&dyn RelationalNode> {
        vec![]
    }

    fn get_relation_definition(&self) -> RelationDefinition {
        RelationDefinition {
            columns: self.row_type.to_vec(),
            physical_properties: vec![],
        }
    }
}

#[derive(Clone)]
pub struct Project {
    input: Box<dyn RelationalNode>,
    expressions: Vec<Box<dyn Expression>>,
}

impl RelationalNode for Project {
    fn get_inputs(&self) -> Vec<&dyn RelationalNode> {
        vec![self.input.as_ref()]
    }

    fn get_relation_definition(&self) -> RelationDefinition {
        unimplemented!()
    }
}

pub struct Filter {
    input: Box<dyn RelationalNode>,
    conditions: Vec<Box<dyn Expression>>,
}

impl RelationalNode for Filter {
    fn get_inputs(&self) -> Vec<&dyn RelationalNode> {
        vec![self.input.as_ref()]
    }

    fn get_relation_definition(&self) -> RelationDefinition {
        self.input.get_relation_definition()
    }
}

pub struct Aggregate {
    input: Box<dyn RelationalNode>,
    groups: Vec<Box<dyn Expression>>,
    aggregations: Vec<Box<dyn Expression>>,
}

pub struct Sort {
    input: Box<dyn RelationalNode>,
    sorts: Vec<Box<dyn Expression>>, //top of the tops sorting?
    limit: i32,
    offset: i32,
}

pub struct Join {
    //join inputs
    //join condition boolean expression
    left: Box<dyn RelationalNode>,
    right: Box<dyn RelationalNode>,
    conditions: Vec<Box<dyn Expression>>,
    join_type: JoinType,
}

pub enum JoinType {
    Inner,
    LeftOuter,
    RightOuter,
    FullOuter,
}

pub struct Union {
    inputs: Vec<Box<dyn RelationalNode>>,
    distinct: bool,
}
