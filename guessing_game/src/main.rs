use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess The Number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please eneter your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("To Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
