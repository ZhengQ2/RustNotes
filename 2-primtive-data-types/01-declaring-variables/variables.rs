fn main() {
    let x = 10;
    println!("x is {}", x);
    // {} is a placeholder for the value of the next argument
    
    // x = 20;
    // // x is immutable, cannot be reassigned
    // // produce compile time error
    // println!("x is {}", x);

    // NOTE: variables are immutable by default in Rust
    //      you can change the mutability of a variable by prefixing it with mut

    let mut y = 10;
    println!("y is {}", y);
    y = 20;
    println!("y is {}", y);

    // Rules for variable names:
    // 1. Can contain letters, numbers, and underscores
    // 2. Can't start with a number
    // 3. Names are case sensitive
    // 4. Can't be keywords, like mut or let

    // Naming conventions: https://rust-lang.github.io/api-guidelines/naming.html
}