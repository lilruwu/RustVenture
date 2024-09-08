use std::io;

fn main() {

    let mut username = String::new();
    println!("Please enter your name: ");
    io::stdin()
    .read_line(&mut username)
    .expect("Failed to read line");
    
    println!("Hello, {}!", username);
}
