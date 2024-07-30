#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    food: String
}

impl Person {
    fn new(name: &str, age: u32, food: &str) -> Self {
        return Person {
            name: name.to_string(),
            age: age,
            food: food.to_string()
        }
    }

    fn show_info(&self) {
        println!("{} {} {}", self.name, self.age, self.food);
    }

    fn add_age(&mut self) {
        self.age += 1;
    }

    fn name_change(&mut self, new_name: String) {
        self.name = new_name;
    }
}

fn main() {

    let mut person: Person = Person::new("Alan", 18, "KFC");

    person.show_info();

    person.add_age();
    person.name_change("Cole".to_string());

    person.show_info();

    print_struct(&person);

    person.show_info();
    
}

fn print_struct(ps: &Person) {
    println!("{:?}", ps);
}
