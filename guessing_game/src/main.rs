use std::{cmp::Ordering, io};

use rand::Rng;
fn main() {
    println!("guessing game!");

   

    let secret_number = rand::rng().random_range(1..=100);
println!("Secret number is {}", secret_number); 
    
 
let mut guess = String::new();
    loop {
        guess.clear();
 println!("Please provide your guessss.");
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue
    };
    

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("to small!"),
        Ordering::Greater => println!("To big"),
        Ordering::Equal => println!("you guessed right!")
    }
}
}
