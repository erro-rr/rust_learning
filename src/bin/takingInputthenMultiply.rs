use std::io;
fn main() {
    let mut input = String :: new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    let mut X:i32 = input.trim().parse().expect("Invalid value of X");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut Y:i32 = input.trim().parse().expect("Invalid Value of Y");
    
    println!("{}",X*Y);
   
}
