use std::io;

fn main() {
    let mut history: Vec<Square> = vec![];
    #[derive(Debug)]
    struct Square {
        side: f32,
        area: f32
    }
    loop {
        println!("1) Input\n2) History\n3) Exit");
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        if input.trim() == "1" {
            println!("Input square side length");
            let mut input: String = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read");
            let input: f32 = input.trim().parse().expect("Error");
            println!("The area of side length {} is {}", input, input * input); 
            let square: Square = Square {
                side: input,
                area: input * input
            };
            history.push(square)

        } else if input.trim() == "2" {
            println!("{:?}", history);
        } else if input.trim() == "3" {
            break;
        }
        
        
    }
}
