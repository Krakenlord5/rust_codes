use std::ptr;

fn factorial(n: i32) -> i32 {
    if n == 0 {
        println!("Calculating factorial({})", n);
        println!("Value: {}, Memory Address: {:p}", n, &n);
        1
    } else {
        println!("Calculating factorial({})", n);
        println!("Value: {}, Memory Address: {:p}", n, &n);
        n * factorial(n - 1)
    }
}

fn main() {
    let result = factorial(5);
    println!("Factorial result: {}", result);
}
