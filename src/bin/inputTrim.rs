use std::io;

fn main() {
    let mut input = String::new();
    
    // Read the input line
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // Split the input into X and Y
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let x: i32 = parts[0].parse().expect("Invalid value for X");
    let y: i32 = parts[1].parse().expect("Invalid value for Y");

    // Calculate the maximum number of customers
    let total_customers = x * y;

    // Output the result
    println!("{}", total_customers);
}
