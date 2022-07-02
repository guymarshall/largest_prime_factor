//The prime factors of 13195 are 5, 7, 13 and 29. What is the largest prime factor of the number 600851475143?

use std::io;

fn main() {
    println!("Please enter an integer:");

    let mut user_input: String = String::new();

    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    let number: i32 = user_input.trim().parse().expect("Please type a number!");

    println!("You entered: {}", number);
}