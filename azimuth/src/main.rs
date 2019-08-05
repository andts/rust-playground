use azimuth::expressions;
use azimuth::expressions::{ExpressionNode, ExpressionTree, LiteralValue};
use azimuth::logical_node::*;

fn main() {
    let a = Column {
        alias: String::from("col1"),
        expression: ExpressionTree {
            expr: expressions::lit_node("hello"),
            children: vec![],
        },
    };
}
