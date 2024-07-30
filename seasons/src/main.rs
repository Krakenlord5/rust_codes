enum Season {
    Summer,
    Rainy,
    Winter
}

impl Season {
    fn describe(&self) -> String {
        match self {
            Season::Summer => return "Summerrrr".to_string(),
            Season::Rainy => return "Rainyyyy".to_string(),
            Season::Winter => return "Winterrrr".to_string(),
        }
    }

    fn action(&self) {
        match self {
            Season::Summer => println!("Summer"),
            _ => println!("Not Summer")
        }
    }
}

fn main() {
    let s = Season::Summer;
    let r = Season::Rainy;

    println!("{}", s.describe());
    s.action();
    r.action();
}
