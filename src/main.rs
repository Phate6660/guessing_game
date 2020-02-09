use std::io;
use std::cmp::Ordering;
use rand::Rng; // from the rand crate specified in Cargo.toml

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101); // generate a random number between 1 and 101
   
    // loop the input and result
    loop {
        println!("Please input your guess.");
    
        let mut guess = String::new(); // create mutable variable with the contents of a new string
        io::stdin().read_line(&mut guess) // variable guess is filled with contents from stdin (e.g. user input)
            .expect("Failed to read line"); // catch the error if one appears

        // ensure guess is an unsigned 32-bit integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {}", guess);

        // compare the guess to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You win!");
                break; // break out of the loop if the right number is guessed
            }
        }
    }
}
