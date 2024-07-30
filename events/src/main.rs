use std::io;

enum Move {
    North,
    South,
    East,
    West
}

struct Player {
    name: String,
    hp: i32,
    stamina: i32
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

impl Player {
    fn new(name: &str, hp: i32, stamina: i32) -> Self {
        return Player {
            name: name.to_string(),
            hp: hp,
            stamina: stamina
        }
    }

    fn stamina_deduct(&mut self) {
        self.stamina -= 1;
    }

    fn show_stat(&self) {
        println!("Name: {}\nHP: {}\nStamina: {}", self.name, self.hp, self.stamina);
    }
}

fn main() {
    let mut player_1: Player = Player::new("Alex", 100, 50);
    let mut player_2: Player = Player::new("Jacob", 90, 70);

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

        n.moving();
        player_1.stamina_deduct();
        player_2.stamina_deduct();

        player_1.show_stat();
        player_2.show_stat();
    }
}
