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

#[derive(Clone)]
pub struct ExpressionTree {
    pub expr: ExpressionNode,
    pub children: Vec<ExpressionTree>,
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
        column_name: String,
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

pub fn lit(string: &str) -> LiteralValue {
    LiteralValue::String(String::from(string))
}

pub fn lit_node(string: &str) -> ExpressionNode {
    ExpressionNode::Literal {
        value: lit(string),
        data_type: String::from("String"),
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
