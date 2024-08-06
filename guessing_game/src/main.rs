use std::io;
use rand::Rng;

fn main() {
    let mut input: String = String::new();
    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);
    // println!("{}", secret_number);
    let mut min: u8 = 0;
    let mut max: u8 = 101;

    loop {
        input.clear();
        println!("-----------------------------------");
        println!("            {} < ??? < {}          ", min, max);
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let guess: u8 = match input.trim().parse() {
            Ok(1..=100) => input.trim().parse().expect("Failed to convert"),
            Ok(_) => {
                // println!("Out of range!");
                continue
            }
            Err(_) => {
                // println!("Not a valid number!");
                continue
            }
        };

        if guess < min || guess > max {
            continue
        }

        if guess != secret_number {
            if guess < secret_number {
                min = guess
            } else {
                max = guess
            }
        } else {
            println!("You win!");
            break
        }
    }
}
