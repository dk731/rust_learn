use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess The Number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number: {secret_number}");

    println!("Please eneter your guess: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");

    let guess: i32 = guess.trim().parse().expect("Please enter a number");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Small!"),
        Ordering::Equal => println!("You Win!"),
        Ordering::Greater => println!("To Big!"),
    }
}
