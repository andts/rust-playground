pub fn run() {
    let mut sum: i64 = 0;
    for num in 1..1000 {
        if num % 3 == 0 || num % 5 == 0 {
            sum += num;
        }
    }
    println!("Result: {}", sum);
}
