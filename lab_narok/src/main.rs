use std::io;
fn main() {
    let mut player_1 = String::new();
    let mut player_2 = String::new();
    println!("Enter first player name:"); // input the first player name
    io::stdin().read_line(&mut player_1).expect("Failed to read line");
    let player_1: &str = player_1.trim();
    println!("Enter second player name:"); // input the second player name
    io::stdin().read_line(&mut player_2).expect("Failed to read line");
    let player_2: &str = player_2.trim();

    let longest_name: usize; // find the longest name
    if player_1.len() > player_2.len() {
        longest_name = player_1.len();
    } else {
        longest_name = player_2.len();
    }
    for i in 0..9 {
        if i == 0 || i == 4 || i == 8 {
            for j in 0..(4 + "Player 1: ".len() + longest_name) {
                print!("*");
            }
            println!();
        } else if i == 1 || i == 3 || i == 5 || i == 7 {
            for j in 0..(4 + "Player 1: ".len() + longest_name) {
                if j == 0 || j == (4 + "Player 1: ".len() + longest_name - 1) {
                    print!("*");
                } else {
                    print!(" ");
                }
            }
            println!();
        } else {
            if player_1.len() > player_2.len() {
                if i == 2 {
                    for j in 0..5 {
                        if j == 0 || j == 4 {
                            print!("*");
                        } else if j == 1 || j == 3 {
                            print!(" ");
                        } else {
                            print!("Player 1: {}", player_1);
                        }
                    }
                } else {
                    for j in 0..5 + longest_name - player_2.len() {
                        if j == 0 || j == 4 + longest_name - player_2.len() {
                            print!("*");
                        } else if j == 2 {
                            print!("Player 2: {}", player_2);
                        } else {
                            print!(" ");
                        }
                    }
                }
            } else {
                if i == 2 {
                    for j in 0..5 + longest_name - player_1.len() {
                        if j == 0 || j == 4 + longest_name - player_1.len() {
                            print!("*");
                        } else if j == 2 {
                            print!("Player 1: {}", player_1);
                        } else {
                            print!(" ");
                        }
                    }
                } else {
                    for j in 0..5 {
                        if j == 0 || j == 4 {
                            print!("*");
                        } else if j == 1 || j == 3 {
                            print!(" ");
                        } else {
                            print!("Player 2: {}", player_2);
                        }
                    }
                }
            }
            println!();
        }
    }
}
