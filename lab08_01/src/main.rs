use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::process::exit;

fn main() {
    let mut file_name: Option<String> = None;  // Use Option to track if a file is open
    let mut file_content: Vec<String> = Vec::new();
    loop {
        let mut choice_vec: Vec<&str> = Vec::new();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice = choice.trim();

        for i in choice.split_whitespace() {
            choice_vec.push(i);
        }
        match choice_vec[0] {
            "open" => {
                if choice_vec.len() == 1 {
                    println!("No file name given.");
                } else if choice_vec.len() > 2 {
                    println!("Can only provide 1 file only.");
                } else {
                    file_name = open_file(&mut file_content, choice_vec[1].to_string());  // Update file_name
                }
            }
            "display" => {
                if file_name.is_none() {
                    println!("No file open. Please open a file first.");
                } else {
                    display_content(&file_content);
                }
            }
            "add" => {
                if file_name.is_none() {
                    println!("No file open. Please open a file first.");
                } else {
                    if choice_vec.len() != 1 {
                        choice_vec.remove(0);
                        add_line(&mut file_content, choice_vec.join(" ").to_string());
                    } else {
                        println!("Please provide text to add.");
                    }
                }
            }
            "delete" => {
                if file_name.is_none() {
                    println!("No file open. Please open a file first.");
                } else {
                    if choice_vec.len() == 2 {
                        match choice_vec[1].trim().parse() {
                            Ok(num) => delete_line(&mut file_content, num),
                            Err(_) => println!("Please enter a valid line number.")
                        }
                    } else if choice_vec.len() > 2 {
                        println!("Can only provide 1 argument.");
                    } else {
                        println!("Please provide a line number.");
                    }
                }
            }
            "write" => {
                if let Some(ref file) = file_name {
                    write_file(file, &file_content);
                } else {
                    println!("No file open. Please open a file first.");
                }
            }
            "exit" => {
                println!("Exiting...");
                exit(0);
            }
            _ => {
                println!("Invalid command.");
            }
        }
    }
}

fn open_file(file_content: &mut Vec<String>, file_name: String) -> Option<String> {

    if Path::new(&file_name).exists() {
        let file = File::open(&file_name).expect("Unable to open file");
        let reader = io::BufReader::new(file);
        file_content.clear();
        for line in reader.lines() {
            file_content.push(line.expect("Unable to read line"));
        }
        println!("File opened successfully.");
        Some(file_name)  // Return the file name
    } else {
        println!("File does not exist. Do you want to create a new file? (y/n)");
        let mut create_new = String::new();
        io::stdin().read_line(&mut create_new).expect("Failed to read line");
        if create_new.trim().eq_ignore_ascii_case("y") {
            file_content.clear();
            println!("New file created. You can now add lines and save the file.");
            Some(file_name)  // Return the new file name
        } else {
            println!("File not opened.");
            None  // No file name, return None
        }
    }
}

fn display_content(file_content: &[String]) {
    if file_content.is_empty() {
        println!("File is empty.");
    } else {
        println!("File content:");
        for (index, line) in file_content.iter().enumerate() {
            println!("{}: {}", index + 1, line);
        }
    }
}

fn add_line(file_content: &mut Vec<String>, new_line: String) {
    file_content.push(new_line.trim().to_string());
    println!("Line added.");
}

fn delete_line(file_content: &mut Vec<String>, line_number: usize) {

    if line_number > 0 && line_number <= file_content.len() {
        file_content.remove(line_number - 1);
        println!("Line deleted.");
    } else {
        println!("Invalid line number.");
    }
}

fn write_file(file_name: &str, file_content: &[String]) {
    let mut file = File::create(file_name).expect("Unable to create file");
    for line in file_content {
        writeln!(file, "{}", line).expect("Unable to write line");
    }
    println!("File written successfully.");
}
