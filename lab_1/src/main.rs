use std::io;

fn pascal(row: i32, col: i32) -> i32 {
    if col == 0 || col == row {
        return 1;
    } else {
        return pascal(row - 1, col - 1) + pascal(row - 1, col);
    }
}

fn print_pascal(n: i32, row: i32) {
    for _ in 0..(n - row + 1) {
        print!("  ");
    }
    for i in 0..row + 1 {
        print!("{:<4}", pascal(row, i));
    }
    println!();
}

fn main() {
    let n: i32 = loop {
        let mut input: String = String::new();
        println!("Enter a number between 1 and 9 (inclusive)");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse() {
            Ok(num) if (1..=9).contains(&num) => break num,
            _ => println!("Not a valid number between 1 and 9")
        }

    };
    for row in 0..n {
        print_pascal(n, row);
    }
}
