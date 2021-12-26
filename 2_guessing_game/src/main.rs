use std::io;
use rand::Rng;
use rand::distributions::Uniform;

fn main() {
    let val = rand::thread_rng().sample(Uniform::new_inclusive(1, 100));
    println!("A uniformly random number is: {}", val);

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
