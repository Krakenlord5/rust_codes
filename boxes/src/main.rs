fn main() {
    let val: i32 = 5;
    println!("{:p} {}", &val, val);
    let new_val = &(&val + 2);
    println!("{:p}", new_val);

}
