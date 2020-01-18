pub fn run() {
    let mut prev = 1;
    let mut fib = 2;
    let mut sum: i64 = 2;
    loop {
        let tmp = fib;
        fib += prev;
        prev = tmp;

        if fib % 2 == 0 {
            sum += fib;
        }

        if fib > 4_000_000 {
            break;
        }
    }
    println!("Result: {}", sum);
}
