// Crates
// - A collection of Rust source code files
// - Binary crates compile to produce an executable program
// - Library crates contain code for other programs to use
// Checkout https://crates.io/ for more crates

// Assume we need to generate random numbers
// There is no built-in random number generator in Rust
// We need to use a crate to get this functionality

// Cargo.toml
// [dependencies]
// rand = "0.8.0"

// use rand;
// Since here we are only using random function from rand crate, we can use
// use rand::random;
// however, this can create confusion, so be careful when using this
use rand::prelude::*;

fn main() {
    let number = random::<f64>();
    println!("number is {}", number);

    let number = thread_rng().gen_range(1..11);
    println!("number is {}", number);
}
