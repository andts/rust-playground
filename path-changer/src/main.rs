use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let current_path: Vec<&str> = args[0].split("/").collect();
    let target_path: Vec<&str> = args[1].split("/").collect();
    let curr_path_norm = calculate_path(&Vec::new(), &current_path);
    let target_path_result = calculate_path(&curr_path_norm, &target_path);
    let result_path_str = target_path_result.join("/");

    println!("result path: {:?}", result_path_str);
}

fn calculate_path<'a>(curr_path: &[&'a str], target_path: &[&'a str]) -> Vec<&'a str> {
    let mut result_path: Vec<&str> = curr_path.clone().to_vec();
    for s in target_path {
        if *s == ".." {
            if result_path.is_empty() {
                panic!("can't navigate out of root directory, wrong path - {}", target_path.join("/"));
            }
            result_path.pop();
        } else {
            result_path.push(*s);
        }
    }
    result_path
}
