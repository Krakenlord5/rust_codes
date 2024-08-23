struct BoxedStack {
    data: Box<Vec<i32>>
}

impl BoxedStack {
    fn new() -> Self {
        BoxedStack {
            data: Box::new(Vec::new())
        }
    }

    fn push(&mut self, value: i32) {
        (*self.data).push(value);
        println!("Pushing {} onto the stack.", value);
    }

    fn pop(&mut self) -> Option<i32> {
        (*self.data).pop()
    }

    fn peek(&self) -> Option<&i32> {
        if !(*self.data).is_empty() {
            Some(&(*self.data)[(*self.data).len() - 1])
        } else {
            None
        }
    }

    fn is_empty(&self) -> bool {
        (*self.data).is_empty()
    }

    fn print_stack(&self) {
        let mut new_list = (*self.data).clone();
        new_list.reverse();
        println!("Stack contents: {:?}", new_list);
    }
}


impl std::fmt::Debug for BoxedStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BoxedStack").field("data", &self.data).finish()
    }
    
}

fn main() {
    let mut new_box = BoxedStack::new();
    new_box.push(5);
    new_box.push(3);
    new_box.push(4);
    new_box.push(8);
    new_box.print_stack();
    println!("Top of the stack: {}", new_box.peek().unwrap());
    while !new_box.is_empty() {
        println!("Popped {} from the stack.", new_box.pop().unwrap());
        new_box.print_stack();
    }
    if new_box.is_empty() {
        println!("Is the stack empty? True");
    } else {
        println!("Is the stack empty? False");
    }

    println!("{:?}", new_box);
}
