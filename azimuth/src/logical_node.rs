//add Arrow for data types
pub enum ExpressionKind {
    RowLevel,
    Aggregate,
}

pub trait Expression {
    //todo some function definition or impl here?
    fn get_function(&self) -> &str;
    fn get_inputs(&self) -> Vec<&dyn Expression>;
    fn get_type(&self) -> arrow::datatypes::DataType;
    fn get_kind(&self) -> ExpressionKind;
}

pub enum MathExpressions {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl Expression for MathExpressions {
    fn get_function(&self) -> &str {
        match self {
            Plus => "+",
            Minus => "-",
            Multiply => "*",
            Divide => "/",
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

pub struct FieldRefExpression {
    field_name: String,
}

pub struct Column {
    alias: String,
    expression: Box<Expression>,
}

pub trait RelationalNode {
    fn get_inputs(&self) -> Vec<&dyn RelationalNode>;
    fn get_schema(&self) -> Vec<Column>;
}

pub enum StandardRelations {
    Scan {
        //schema
    },
    Project {
        //projection expressions
    },
    Filter {
        //filters
    },
    Aggregate {
        //grouping keys
    //agg functions
    },
    Sort {
        //sort columns
    //limit + offset
    },
    Join {
        //join inputs
    //join condition boolean expression
    },
    Union {
        //union inputs
    //distinct/all
    },
}

impl RelationalNode for StandardRelations {
    fn get_inputs(&self) -> Option<Vec<RelationalNode>> {
        unimplemented!()
    }

    fn get_schema(&self) -> Vec<Column> {
        unimplemented!()
    }
}
