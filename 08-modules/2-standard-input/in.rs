// Standard Input
// - Read command line inputs from the user
// - part of the std::io module

use std::io;

fn main() {
    // Used to read input from the user
    let mut buffer = String::new();
    println!("Enter a message: ");
    io::stdin().read_line(&mut buffer);
    println!("You entered: {}", buffer);
}
