fn swap_elements<T>(a: Vec<T>, b: Vec<T>) -> (Vec<T>, Vec<T>) {
    (b, a)
}

fn main() {
    let vec_1 = vec![1, 2, 3];
    let vec_2 = vec![4, 5, 6];
    let result = swap_elements(vec_1,vec_2);
    println!("{:?} {:?}", result.0, result.1);
}
