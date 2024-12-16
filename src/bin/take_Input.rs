// use std::io;
// fn main(){
//     let mut input = String::new();
//     print!("Enter your nos\n");
//     io::stdin().read_line(&mut input).expect("Failed to read Input");
//     let num:u32 = match input.trim().parse(){
//         Ok(num) => num,
//         Err(_) =>{
//             print!("Please enter correct value");
//             return;
//         }
//     };
//     print!("{}",num);
// }


use std::io;
fn main() {
let mut input = String :: new();
    println!("Hello, world!");
    io::stdin().read_line(&mut input).expect("Failed to read Input");
    let mut num :i32= input.trim().parse().expect("Invalid Input");
    println!("{}",num);
}
