fn fibonacci(number: i32) -> i32 {
    if number <= 2 {
        return 1;
    } else {
        return fibonacci(number - 1) + fibonacci(number - 2);
    }
}

fn main() {
    println!("result: {}", fibonacci(5));
}
