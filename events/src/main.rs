use std::io;

enum Move {
    North,
    South,
    East,
    West
}

impl Move {
    fn moving(&self) {
        match self {
            Move::North => println!("You moved North!"),
            Move::South => println!("You moved South!"),
            Move::East => println!("You moved East!"),
            Move::West => println!("You moved West!"),
        }
    }
}

fn main() {
    loop {
        let mut n: String = String::new();
        println!("(1) North (2) South (3) East (4) West (5) byebye");
        io::stdin().read_line(&mut n).expect("Failed to read line");
        let n = match n.trim().parse() {
            Ok(1) => Move::North,
            Ok(2) => Move::South,
            Ok(3) => Move::East,
            Ok(4) => Move::West,
            Ok(5) => {
                println!("Bye bye!");
                break;
            }
            Ok(_) => {
                println!("Invalid command");
                continue;
            },
            Err(_) => {
                println!("Invalid command");
                continue;
            }
        };

        n.moving()
    }
}
