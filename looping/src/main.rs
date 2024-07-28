use std::io;

fn main() {
    loop {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a valid number!");
                continue
            }
        };
        println!("You picked the number {}!", input);
    }
}
