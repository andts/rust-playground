use azimuth::expressions::{col_ref, ExpressionTree};
use azimuth::logical_node::standard_nodes::{agg, filter, project, scan_csv};
use azimuth::logical_node::Column;

fn main() {
    //use macros for pretty dsl?
    /*
            let query = aggregate(
                col(1), //groups
                sum(col(2)), //aggregations
                //child
                filter(
                    *[eq(col(3), lit("Kyiv"))], //conditions
                    //child
                    project(
                        vec![col(1), col(3), col(5)], //projection columns
                        //child
                        scan_csv("file.csv")),
                ),
            );
    */

    let scan = scan_csv("test.csv");
    let project = project(
        vec![Column {
            alias: "first".to_string(),
            expression: col_ref(0, &scan),
        }],
        scan,
    );

    let filter = filter(vec![], project);

    let query = agg(
        vec![Column {
            alias: "first".to_string(),
            expression: col_ref(0, &filter),
        }],
        vec![Column {
            alias: "first".to_string(),
            expression: col_ref(0, &filter),
        }],
        filter,
    );

    //    let optimized_query = optimizer.optimize_query(scan);
    //    let executor = query.build();
    //    let result_set = executor.execute();
    //    println!("Result:\n {}", result_set);
}
