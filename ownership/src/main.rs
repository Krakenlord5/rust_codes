
fn main() {
    let a: i32 = 5; // owner
    let b: i32 = a; // x is copied, not moved
    println!("{} {}", a, b); // both are accessible

    let s1: String = String::from("hello"); // owner
    let s2: String = s1; // ownership moved to s2, s1 is no longer accessible
    //println!("{}", s1); // error
    println!("{}", s2); // no error

    let mut s: String = String::from("hello");

    // Immutable borrow
    let r1: &String = &s; 
    let r2: &String = &s; // multiple borrows are allowed
    println!("{} {}", r1, r2);

    let len: usize = calculate_length(&s);
    println!("The length of '{}' is {}", s, len);

    // Mutable borrow to change the string
    change(&mut s);
    println!("After change, s is '{}'", s);

    // Mutable borrow (only allowed because r1 and r2 are no longer used)
    let r3: &mut String = &mut s;
    r3.push_str("!!!!!!!!!!!!");
    println!("After mutable borrow, s is '{}'", r3);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world!");
}
