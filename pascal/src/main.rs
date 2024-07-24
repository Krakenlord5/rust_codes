use std::io;

fn pascal(i: i32, j: i32)-> i32{
    if j == 0 || j == i {
        return 1;
    } else {
        return pascal(i - 1, j - 1) + pascal(i - 1, j);
    }
}

fn main() {
    let mut input = String::new();
    println!("Enter number of rows:");
    io::stdin().read_line(&mut input).expect("failed to read line");
    let rows: i32 = match input.trim().parse(){
        Ok(num) => num,
        Err(_) =>{
            println!("Invalid input");
            0
        }
    };

    for i in 0..rows {
        for _ in 0..rows - i {
            print!(" ");
        }
        for j in 0..i + 1 {
            print!("{} ", pascal(i, j));
        }
        println!();

    }

}