use crate::logical_node::RelationTree;

pub enum ExpressionKind {
    RowLevel,
    Aggregate,
}

#[derive(Clone)]
pub enum LiteralValue {
    String(String),
    Integer(i64),
    Float(f64),
    DateTime(chrono::NaiveDateTime),
}

//can the tree be typed by the return type of the expr node?
#[derive(Clone)]
pub struct ExpressionTree {
    pub expr: ExpressionNode,
    pub children: Vec<ExpressionTree>,
}

impl ExpressionTree {
    //todo use arrow data types here
    pub fn get_type() -> String {
        unimplemented!()
    }
}

#[derive(Clone)]
pub enum ExpressionNode {
    Literal {
        value: LiteralValue,
        data_type: String,
    },
    FunctionCall {
        function_name: String,
        return_type: String,
    },
    ColumnRef {
        column_index: u32,
        data_type: String,
    },
    Array {
        data_type: String,
    },
    Set {
        data_type: String,
    },
    Tuple {
        first_type: String,
        second_type: String,
    },
}

fn lit_value(string: &str) -> LiteralValue {
    LiteralValue::String(String::from(string))
}

fn lit_node(string: &str) -> ExpressionNode {
    ExpressionNode::Literal {
        value: lit_value(string),
        data_type: String::from("String"),
    }
}

pub fn lit(string: &str) -> ExpressionTree {
    ExpressionTree {
        expr: lit_node(string),
        children: vec![],
    }
}

pub fn col_ref(idx: u32, rel: &RelationTree) -> ExpressionTree {
    let input_row_type = rel.get_row_type();
    let column_type = input_row_type
        .get(idx as usize)
        //todo add rel in message
        .expect(format!("Column with index {} not found in rel", idx).as_str());
    ExpressionTree {
        expr: ExpressionNode::ColumnRef {
            column_index: idx,
            data_type: column_type.clone(),
        },
        children: vec![],
    }
}

//pub trait Expression {
//    todo some function definition or impl here?
//    fn get_function(&self) -> &str;
//    fn get_inputs(&self) -> Vec<&dyn Expression>;
//    fn get_type(&self) -> &str; //arrow::datatypes::DataType;
//    fn get_kind(&self) -> ExpressionKind;
//}

pub mod math {
    use crate::expressions::{ExpressionKind, ExpressionTree};

    #[derive(Clone)]
    pub enum MathExpressions {
        Plus,
        Minus,
        Multiply,
        Divide,
    }

    /*
        impl Expression for MathExpressions {
            fn get_function(&self) -> &str {
                match self {
                    MathExpressions::Plus => "+",
                    MathExpressions::Minus => "-",
                    MathExpressions::Multiply => "*",
                    MathExpressions::Divide => "/",
                }
            }

            fn get_inputs(&self) -> Vec<&dyn Expression> {
                unimplemented!()
            }

            fn get_type(&self) -> &str {
                unimplemented!()
            }

            fn get_kind(&self) -> ExpressionKind {
                unimplemented!()
            }
        }
    */

}

pub mod comparison {
    //equals, lt, gt, etc...
}

pub struct FieldRefExpression {
    field_name: String,
}
