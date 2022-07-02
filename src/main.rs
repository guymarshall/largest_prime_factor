//The prime factors of 13195 are 5, 7, 13 and 29. What is the largest prime factor of the number 600851475143?

use std::io;

fn get_user_input(prompt: &str) -> i32 {
    println!("{}", prompt);

    let mut user_input: String = String::new();

    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    let number: i32 = user_input.trim().parse().expect("Please enter an integer!");

    return number;
}

fn main() {
    let number: i32 = get_user_input("Please enter an integer:");

    println!("{}", number);
}