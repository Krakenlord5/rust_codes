use std::{io, vec};

fn main() {
    let mut numbers: Vec<i32> = vec![];
    let mut sum: i32 = 0;
    let mut total: i32 = 0;
    loop {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        if input.trim() == "$" {
            break
        } else {
        let input: i32 = input.trim().parse().expect("Not a valid number");
        numbers.push(input);
        }
    }
    for i in 0..numbers.len() {
        sum += numbers[i];
        total += 1;
        if i == numbers.len() - 1 {
            println!("{}", numbers[i]);
        } else {
            print!("{}, ", numbers[i]);
        }
    }
    println!("Sum: {}", sum);
    println!("Average: {}", sum / total);
}
