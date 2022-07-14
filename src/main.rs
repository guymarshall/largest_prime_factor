//The prime factors of 13195 are 5, 7, 13 and 29. What is the largest prime factor of the number 600851475143?

use std::io;

fn get_user_input(prompt: &str) -> u64 {
    println!("{}", prompt);

    let mut user_input: String = String::new();

    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    let number: u64 = user_input.trim().parse().expect("Please enter an integer!");

    return number;
}

fn is_prime_number(number: u64) -> bool {
    if number % 2 == 0 {
        return false;
    }
    let root_of_number: u64 = (number as f64).sqrt() as u64;
    for i in (3..root_of_number).step_by(2) {
        if number % i == 0 {
            return false
        }
    }

    return true;
}

fn main() {
    let number: u64 = get_user_input("Please enter an integer:");
    let mut largest_prime_factor: u64 = 0;

    for i in 1..number + 1 {
        if is_prime_number(i) {
            if number % i == 0 {
                largest_prime_factor = i;
            }
        }
    }

    println!("Largest prime factor of {}: {}", number, largest_prime_factor);
}