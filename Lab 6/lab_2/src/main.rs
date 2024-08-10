use std::fmt;
struct DataStore<T> {
    //struct vector 
    items: Vec<T>,
}

impl <T> DataStore<T> {
    //Add item to store

    fn new() -> Self {
        DataStore { items: (Vec::new()) }
    }

    fn add_item(&mut self, item:T){
        self.items.push(item);

    }

    fn remove_item(&mut self, index: usize) -> Option<T>{
        if index < self.items.len() {
            Some(self.items.remove(index))
        }else {
            None
        }
    }

    fn get_item(&mut self, index: usize) -> Option<&T>{
        self.items.get(index)
    }

    fn find_item<F>(&mut self, predicate: F) -> Option<&T>
        where
        F: Fn(&T) -> bool,
        {
            self.items.iter().find(|&item| predicate(item))
        }
    
}

#[derive(Debug)]
enum DataType<T> {
    Number(T),
    Text(String),
    Boolean(bool)
}

impl<T: fmt::Display> fmt::Display for DataType<T>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            DataType::Number(n) => write!(f, "Number: {}", n),
            DataType::Text(t) => write!(f, "Text: {}", t),
            DataType::Boolean(b) => write!(f, "Boolean: {}", b),
        }
    }
}

impl <T: std::fmt::Display> DataType<T> {
    fn print(& self) {
        match self {
            DataType::Number(n) => println!("Number : {}", n),
            DataType::Text(t) => println!("Text : {}", t),
            DataType::Boolean(b) => println!("Boolean : {}", b)
        }
    }
}

fn main(){
    //Typei32
    let mut int_store = DataStore::<DataType<i32>>::new();
    int_store.add_item(DataType::Number(42));
    
    let mut float_store = DataStore::<DataType<f64>>::new();
    float_store.add_item(DataType::Number(3.14));

    let mut string_store = DataStore::<String>::new();
    string_store.add_item("Hello".to_string());

    println!("{}", int_store.get_item(0).unwrap());
    println!("{}", float_store.get_item(0).unwrap());
    println!("{}", string_store.get_item(0).unwrap());
    
let found = int_store.find_item(|item| match item {
    DataType::Number(n) => *n > 30,
    _ => false,
});
println!("Found {:?}", found)

}