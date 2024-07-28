fn matrix_multi(a: &[[i32; 3]; 2], b: &[[i32; 3]; 3]) -> [[i32; 3]; 2] {
    let mut final_ans: [[i32; 3]; 2] = a.clone();
    for i in 0..a.len() {
        let mut new_arr: [i32; 3] = [0, 0, 0];
        for j in 0..b[0].len() {
            let mut total: i32 = 0;
            for k in 0..a[0].len() {
                let result: i32 = a[i][k] * b[k][j];
                total += result;
            }
            new_arr[j] = total;
        }
        final_ans[i] = new_arr;
    }
    return final_ans;
}

fn main() {
    let mat_1: [[i32; 3]; 2] = [
        [1, 2, 3],
        [4, 5, 6]
    ];

    let mat_2: [[i32; 3]; 3] = [
        [7, 8, 9],
        [10, 11, 12],
        [13, 14, 15]
    ];

    let result: [[i32; 3]; 2] = matrix_multi(&mat_1, &mat_2);

    for i in &result {
        println!("{:?}", i);
    }

}
