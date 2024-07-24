fn matrix_multi(v1: Vec<[i32; 3]>, v2: Vec<[i32; 2]>) -> Vec<Vec<i32>> {
    let mut final_vec: Vec<Vec<i32>> = Vec::new();
    for rows in 0..v1.len() {
        let mut new_vec: Vec<i32> = Vec::new();
        for columns in 0..v2[0].len() {
            let mut total: i32 = 0;
            for num in 0..v1[0].len() {
                let result: i32 = v1[rows][num] * v2[num][columns];
                total += result;
            }
            new_vec.push(total);
        }
        final_vec.push(new_vec);
    }
    return final_vec;
}

fn main() {
    let v1: Vec<[i32; 3]> = vec![
        [1, 4, 4], 
        [2, 3, 2], 
        [1, 3, 5]
    ];

    let v2: Vec<[i32; 2]> = vec![
        [3, 2], 
        [3, 4], 
        [5, 6]
    ];

    let result: Vec<Vec<i32>> = matrix_multi(v1, v2);

    for item in &result {
        println!("{:?}", item);
    }
}
