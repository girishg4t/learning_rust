use rand::Rng;
use std::{cmp::Ordering, io};

pub fn guess_number() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Please input your suggestion");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {secret_number}");

    let int_guess: i32 = guess.trim().parse().unwrap();
    match int_guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too Large"),
        Ordering::Equal => println!("You Win"),
    }
}
