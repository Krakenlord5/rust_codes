fn print_matrix(matrix: &Vec<Vec<i32>>) {
    for item in matrix {
        println!("{:?}", item);
    }
}

fn sum_row(matrix: &Vec<Vec<i32>>, row: usize) -> i32 {
    let mut total = 0;
    for item in matrix[row].clone() {
        total += item;
    }
    total
}

fn sum_column(matrix: &Vec<Vec<i32>>, column: usize) -> i32 {
    let mut total = 0;
    for item in matrix {
        total += item[column];
    }
    total
}

fn sum_diagonal(matrix: &Vec<Vec<i32>>) -> i32 {
    let mut total = 0;
    for i in 0..matrix.len() {
        total += matrix[i][i];
    }
    total
}

fn sum_anti_diagonal(matrix: &Vec<Vec<i32>>) -> i32 {
    let mut total = 0;
    for i in 0..matrix.len() {
        total += matrix[matrix.len() - i - 1][i];
    }
    total
}

fn main() {
    let matrix_1: Vec<Vec<i32>> = vec![
        vec![1, 2, 3],
        vec![3, 5, 2],
        vec![7, 3, 5],
    ];

    print_matrix(&matrix_1);

    for i in 0..matrix_1.len() {
        println!("Row {} sum: {}", i + 1, sum_row(&matrix_1, i));
        println!("Column {} sum: {}", i + 1, sum_column(&matrix_1, i));
    }

    println!("Diagonal sum: {}", sum_diagonal(&matrix_1));
    println!("Anti-Diagonal sum: {}", sum_anti_diagonal(&matrix_1));
}
