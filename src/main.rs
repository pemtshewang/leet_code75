mod lib;
use std::io;
fn main() {
    let mut input_array = String::new();
    let mut target = String::new();
    println!("Array values separated by spaces");
    io::stdin()
        .read_line(&mut input_array)
        .expect("Failed to read a array");

    let input_array:Vec<i32> = input_array
    .split_whitespace()
    .map(|s| s.parse().expect("Failed to parse a value"))
    .collect();

    let solution = lib::contains_duplicate(input_array);
    println!("{solution}");


}
