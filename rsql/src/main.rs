use pest::Parser;
use rsql::sql_parser::*;

fn main() {
//    let query = r#"select city, sum(sales)
//    from some_table as t1
//    inner join other_table as t2
//    on t1.join_field = t2.join_field
//    where t1.filter_field < 15
//    and t2.filter_field = 'Hello!'
//    group by city
//    having sum(sales) > 500"#;
    let query = "sales as one, sum(sales) as two, min(planned_sales) as three";
//    let parse = SqlParser::parse(Rule::query_expression, query);
    let parse = SqlParser::parse(Rule::select_list, query);
    match parse {
        Err(e) => { println!("Error: {}", format!("{}", e)) }
        Ok(result) => { println!("Result: {:?}", result) }
    }
}
