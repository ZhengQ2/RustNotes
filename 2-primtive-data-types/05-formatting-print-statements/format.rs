fn main() {
    let a = 10.0;
    let b = 3.0;
    let c = a / b;
    println!("c is {}", c);
    // > c is 3.3333333333333335

    // Consider: We want to control the number of significant figures
    // Solution: Rust build in formatting macros
    println!("c is {:.3}", c);
    // > c is 3.333
    println!("c is {:8.3}", c);
    // > c is    3.333
    // {:a.b} is the format specifier, a is the number of characters, b is the number of decimal places
    println!("c is {:08.3}", c);
    // > c is 0003.333
    // the 0 means occupying total of 8 characters, with '0' characters before the number
    println!("c is {:08.3}\na is {}", c, a);
    // > c is 0003.333  
    // > a is 10
    // "\n" is a newline character, println macro automatically adds a newline at the end of the line
    // to avoid the newline at the end of the line, use print! macro
    print!("this");
    println!(" is a newline");
    // > this is a newline

    // Consider: What if we want to have a parameter displayed at multiple places?
    // Solution: use the position macro
    println!("c is {0:08.3}\na is {1}\nonce again, c is {0}", c, a);
    // > c is 0003.333
    // > a is 10
    // > once again, c is 3.3333333333333335

    // More formatting macros see rust documentation
}