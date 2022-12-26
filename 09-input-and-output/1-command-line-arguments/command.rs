// Command-line Arguments
// - Command-line arguments are the values passed to a program when it is run
// - Common Uses:
//   - File paths
//   - Configuration options

// std::env::args()
// - Returns an iterator of command-line arguments
// - First argument is traditionally the executable path

use std::env;

fn main() {
    if env::args().len() <= 2 {
        println!("Program requires at least 2 arguments");
        return;
    }

    for (index, argument) in env::args().enumerate() {
        println!("argument {} is {}", index, argument);
    }

    // What if we only want to know a specific argument?
    // We can use the nth() method on the iterator

    let arg2 = env::args().nth(2).unwrap();
    println!("argument 2 is {}", arg2);
    // Note that if the user only provides 1 argument, this will panic
    // Solution: a if statement

}
