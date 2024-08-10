struct Car {
    name: String,
    age: i32,
    color: String,
}

struct Boat {
    name: String,
    age: i32,
    color: String,
}

trait Info {
    fn info(&self);
}

impl Info for Car {
    fn info(&self) {
        println!("{} {} {}", self.name, self.age, self.color)
    }
}
impl Info for Boat {
    fn info(&self) {
        println!("{} {} {}", self.name, self.age, self.color)
    }
}

fn main() {
    let car = Car {
        name: String::from("Ford"),
        age: 30,
        color: String::from("Red"),
    };

    let boat = Boat {
        name: String::from("idk"),
        age: 14,
        color: String::from("White"),
    };

    car.info();
    boat.info();
}
