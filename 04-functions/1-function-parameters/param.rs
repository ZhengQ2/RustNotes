// Functions
// - organize code into reusable blocks
// - every Rust program has at least one function: main

// Main function
// - special function that is the entry point of every Rust program
// - the main function is always the first code that runs in every executable Rust program

fn main() {
    say_hello();
    say_hello();
    // rust functions does not need to be declared before they are used

    let x = 1;
    let y = 2;
    say_the_sum(x, y);
    //            ^ note that x and y should be u8, not i32, because the function expects u8
    // Proof:
    // say_a_number(x);
    // Error: expected u8, found i32
}

fn say_hello() {
    println!("Hello!");
    say_a_number(13);
    //            ^ this is called an argument
}

// function parameters
// - variables allow functions to accept input
// - parameters are declared within the parentheses after the function name

fn say_a_number(number: i32) {
    //                   ^ the type of the parameter
    // Since Rust is a statically typed language, when declare a parameter, must declare its type.
    println!("number is {}", number);
}

fn say_the_sum(a: u8, b: u8) {
    let sum = a + b;
    println!("sum is {}", sum);
}
