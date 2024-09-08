use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!"); // Print a string to the console

    let secret_number = rand::thread_rng().gen_range(1..=100); // Generate a random number between 1 and 100



loop{
    println!("Please input your guess."); // Print a string to the console

    let mut guess = String::new(); // Create a mutable variable named guess and bind it to a new, empty instance of a String

    io::stdin() // Call the stdin function from the io module
        .read_line(&mut guess)
        .expect("Failed to read line"); // Call the read_line method on the standard input handle to get input from the user


    let guess: u32 = match guess.trim().parse() { // Parse the string into a number
        Ok(num) => num,
        Err(_) => continue,
    };
        
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        }
    }
}
    
}
