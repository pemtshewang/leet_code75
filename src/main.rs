use std::io::{self, Read};
mod lib;
use lib::two_sum;
fn main() {
    let mut input_array = String::new();
    let mut target = String::new();
    println!("Array values separated by spaces");
    io::stdin()
        .read_line(&mut input_array)
        .expect("Failed to read a array");

    println!("Target value");
    io::stdin()
        .read_line(&mut target)
        .expect("Failed to read target value");

    let input_array:Vec<i32> = input_array
    .split_whitespace()
    .map(|s| s.parse().expect("Failed to parse a value"))
    .collect();

    let target = match target.trim().parse::<i32>(){
        Ok(n) => n,
        Err(_) =>{
            panic!("Failed to parse a given target value");
        }
    };

    let solution = two_sum(input_array, target);
    println!("{:?}",solution);

}
