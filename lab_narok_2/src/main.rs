use std::io;
fn main() {
    let mut employees: Vec<(String, i32, f64)> = Vec::new();

    for employee_number in 1..=5 {

        let mut name: String = String::new();
        let mut age: String = String::new();
        let mut salary: String = String::new();

        println!("Employee {} name:", employee_number);
        io::stdin().read_line(&mut name).expect("Failed to read line");
        println!("Employee {} age:", employee_number);
        io::stdin().read_line(&mut age).expect("Failed to read line");
        println!("Employee {} salary:", employee_number);
        io::stdin().read_line(&mut salary).expect("Failed to read line");

        let name = name.trim().to_string();
        let age: i32 = age.trim().parse().expect("Error");
        let salary: f64 = salary.trim().parse().expect("Error");

        employees.push((name, age, salary));
    }

    let mut oldest_employee_age: &i32 = &employees[0].1; 
    let mut oldest_employee_name: &str = &employees[0].0;
    let mut highest_salary_value: &f64 = &employees[0].2; 
    let mut highest_salary_name: &str = &employees[0].0;

    for employee_number in 0..5 {
        println!("Employee {}: Name = \"{}\", Age = {}, Salary = {}", employee_number + 1, employees[employee_number].0, employees[employee_number].1, employees[employee_number].2);

        if &employees[employee_number].1 > oldest_employee_age {
            oldest_employee_age = &employees[employee_number].1;
            oldest_employee_name = &employees[employee_number].0;
        }

        if &employees[employee_number].2 > highest_salary_value {
            highest_salary_value = &employees[employee_number].2;
            highest_salary_name = &employees[employee_number].0;
        }
    }

    println!("Oldest employee: {} with the age of {}!", oldest_employee_name, oldest_employee_age);
    println!("Employee with the highest salary: {} with the salary of {}!", highest_salary_name, highest_salary_value);
}
