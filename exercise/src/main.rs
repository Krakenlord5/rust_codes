use std::io;

fn ask_float() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input.trim().parse::<f64>().unwrap();
}

fn sum(a: f64, b: f64) -> f64 {
    return a + b;
}

fn main() {
    let a = ask_float();
    let b = ask_float();
    let ans = sum(a, b);
    println!("First Number: {}", a);
    println!("Second Number: {}", b);
    println!("{} + {} = {}", a, b, ans);
}
