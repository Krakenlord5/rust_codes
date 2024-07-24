use std::io;

fn main() {
    let mut products: Vec<(u32, String, u32)> = Vec::new();
    loop {
        println!("--------------------------------------");
        println!("Warehouse Inventory Management");
        println!("1. Add new product\n2. Update Stock Quantity\n3. Remove product\n4. List all products\n5. Exit");
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        let input: &str = input.trim();
        match input {
            "1" => {
                let mut duplicate: bool = false;
                let mut id: String = String::new();
                let mut name: String = String::new();
                let mut quantity: String = String::new();

                println!("Enter product ID:");
                io::stdin().read_line(&mut id).expect("Failed to read");
                let id: u32 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("ID is not a valid number!");
                        continue;
                    }
                };
                
                for product in &products {
                    if product.0 == id {
                        println!("Product already exists!");
                        duplicate = true;
                    }
                }

                if duplicate {
                    continue;
                }

                println!("Enter product name:");
                io::stdin().read_line(&mut name).expect("Failed to read");
                let name: String = name.trim().to_string();

                println!("Enter product quantity:");
                io::stdin().read_line(&mut quantity).expect("Failed to read");
                let quantity: u32 = match quantity.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Quantity is not a valid number!");
                        continue;
                    }
                };

                products.push((id, name, quantity));
                println!("Added product successfully");

            }

            "2" => {
                let mut found: bool = false;
                let mut id: String = String::new();
                println!("Enter product ID:");
                io::stdin().read_line(&mut id).expect("Failed to read");
                let id: u32 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("ID is not a valid number!");
                        continue;
                    }
                };

                for product in &mut products {
                    if product.0 == id {
                        found = true;
                        let mut new_quan: String = String::new();
                        println!("Enter the new quantity:");
                        io::stdin().read_line(&mut new_quan).expect("Failed to read");
                        let new_quan: u32 = match new_quan.trim().parse() {
                            Ok(num) => num,
                            Err(_) => {
                                println!("Quantity is not a valid number!");
                                continue;
                            }
                        };
                        product.2 = new_quan;
                        println!("Quantity changed successfully!");
                    }
                }

                if !found {
                    println!("Product doesn't exist!");
                }
            }

            "3" => {
                let mut changed: bool = false;
                let mut id: String = String::new();
                println!("Enter product ID you want to remove:");
                io::stdin().read_line(&mut id).expect("Failed to read");
                let id: u32 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("ID is not a valid number!");
                        continue;
                    }
                };

                for i in 0..products.len() {
                    if products[i].0 == id {
                        products.remove(i);
                        println!("Product removed successfully!");
                        changed = true;
                        break;
                    }
                }

                if !changed {
                    println!("Product doesn't exist!");
                }
            }

            "4" => {
                if products.is_empty() {
                    println!("Empty stock!");
                } else {
                    for product in &products {
                        println!("ID: {}, Name: {}, Quantity: {}", product.0, product.1, product.2);
                    }
                }
            }

            "5" => {
                println!("Bye bye!!!!!!!");
                break;
            }

            _ => {
                println!("Invalid command");
            }
        }
    }
}
