use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Input compare number!");
    let mut compare_number = String::new();
    io::stdin().read_line(&mut compare_number).expect("Invalid number!");

    println!("Generating secret number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: '{}'", secret_number);

    match compare_number.trim().parse::<u32>() {
        Ok(guess) =>
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => println!("Lucky man!!! You win!"),
            }
        Err(_) => println!("Invalid input. Please enter a number."),
    }
}
