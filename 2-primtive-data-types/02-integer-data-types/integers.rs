fn main() {
    // Rust is statically typed language
    // - all variable data types must be known at compile time

    // Rust Scalar Data Types:
    // - Integers
    // - Floating-point
    // - Boolean
    // - Characters

    // Rust Integer Data Types:
    // - Charaterized by number of bits
    // - Unsigned and signed

    // 8 bit integers:
    // Represents 2^8 = 256 possible values
    // - u8: unsigned, range 0 to 255
    // - i8: signed, range -128 to 127

    // 16 bit integers:
    // Represents 2^16 = 65536 possible values
    // - u16: unsigned, range 0 to 65535
    // - i16: signed, range -32768 to 32767

    // 32 bit integers:
    // Represents 2^32 = 4294967296 possible values
    // - u32: unsigned, range 0 to 4294967295
    // - i32: signed, range -2147483648 to 2147483647
    
    // Rust Integer Data Types:
    // - u8, u16, u32, u64, u128
    // - i8, i16, i32, i64, i128
    
    // x: i8 forces x to be an i8 data type
    let x: u8 = 255;
    println!("x is {}", x);

    // let x: u8 = -10; // compile time error
    // because x is an i8, it can't be negative

    // let x: u8 = 256; // compile time error
    // because x is an i8, it can't be greater than 255
    
    let mut y: u8 = 255;
    // y = y + 1; // run time error
    // because y is an i8, it can't be greater than 255
    println!("y is {}", y);
}