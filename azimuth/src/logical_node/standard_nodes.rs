use std::path::Path;

use crate::expressions::ExpressionTree;
use crate::logical_node::{Column, RelationDefinition, RelationTree, RelationalNode};
use std::collections::HashSet;

#[derive(Debug)]
struct ColumnDef {
    name: String,
    data_type: String,
}

#[derive(Debug)]
pub struct CsvScan {
    row_type: Vec<ColumnDef>,
    file_name: String,
}

impl RelationalNode for CsvScan {
    fn get_relation_definition(&self, input: &Vec<RelationTree>) -> RelationDefinition {
        RelationDefinition {
            row_type: self
                .row_type
                .iter()
                .map(|col| col.data_type.clone())
                .collect(),
            physical_properties: HashSet::new(),
        }
    }
}

pub struct Project {
    expressions: Vec<Column>,
}

impl RelationalNode for Project {
    fn get_relation_definition(&self, input: &Vec<RelationTree>) -> RelationDefinition {
        unimplemented!()
    }
}

pub struct Filter {
    conditions: Vec<ExpressionTree>,
}

impl RelationalNode for Filter {
    fn get_relation_definition(&self, input: &Vec<RelationTree>) -> RelationDefinition {
        unimplemented!()
    }
}

pub struct Aggregate {
    groups: Vec<Column>,
    aggregations: Vec<Column>,
}

impl RelationalNode for Aggregate {
    fn get_relation_definition(&self, input: &Vec<RelationTree>) -> RelationDefinition {
        unimplemented!()
    }
}

pub struct Sort {
    sorts: Vec<ExpressionTree>,
    //top of the tops sorting?
    limit: i32,
    offset: i32,
}

pub struct Join {
    //join inputs
    //join condition boolean expression
    join_type: JoinType,
    left: Box<dyn RelationalNode>,
    right: Box<dyn RelationalNode>,
    conditions: Vec<ExpressionTree>,
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

//factory functions for rel nodes

pub fn col(alias: &str, expr: ExpressionTree) -> Column {
    Column {
        alias: alias.to_string(),
        expression: expr,
    }
}

pub fn scan_csv(file_name: &str) -> RelationTree {
    let mut csv_reader = csv::Reader::from_path(Path::new(file_name))
        .expect(format!("Can't open file {}", file_name).as_str());
    let headers = csv_reader
        .headers()
        .expect(format!("Can't read headers in file {}", file_name).as_str());

    let columns = headers
        .into_iter()
        .map(|header| {
            let header_elements: Vec<&str> = header.split(':').collect();
            ColumnDef {
                name: (*header_elements.get(0).expect("Column header not found")).to_string(),
                data_type: header_elements
                    .get(1)
                    .map(|s| (*s).to_string())
                    .unwrap_or_else(|| "string".to_string()),
            }
        })
        .collect();

    let scan_node = CsvScan {
        row_type: columns,
        file_name: file_name.to_string(),
    };

    RelationTree {
        node: Box::new(scan_node),
        children: vec![],
    }
}

pub fn project(expressions: Vec<Column>, child: RelationTree) -> RelationTree {
    //todo resolve columns here, not externally?
    let project_node = Project { expressions };

    RelationTree {
        node: Box::new(project_node),
        children: vec![child],
    }
}

pub fn filter(conditions: Vec<ExpressionTree>, child: RelationTree) -> RelationTree {
    //todo validate type of the expression?
    let filter_node = Filter { conditions };

    RelationTree {
        node: Box::new(filter_node),
        children: vec![child],
    }
}

pub fn agg(groups: Vec<Column>, aggs: Vec<Column>, child: RelationTree) -> RelationTree {
    //todo validate type of the expression?
    let agg_node = Aggregate {
        groups,
        aggregations: aggs,
    };

    RelationTree {
        node: Box::new(agg_node),
        children: vec![child],
    }
}
