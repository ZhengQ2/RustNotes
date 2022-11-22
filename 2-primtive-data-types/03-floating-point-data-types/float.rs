fn main() {
    // Rust's decimal number uses IEEE 754 floating-point standard
    // Rust has two types of floating-point numbers:
    // - Single-precision (32-bit): f32
    // - Double-precision (64-bit): f64
    // Values are stored as fractional and exponent parts
    // Max value: f32 = 3.402823e+38
    // Min value: f32 = 1.175494e-38
    // Max value: f64 = 1.797693e+308
    // Min value: f64 = 2.2250738585072014e-308
    // However, the persistence of floating-point numbers is not guaranteed

    // Eg: Pi = 3.1415926535897932846264338327950288419...
    // as f32 = 3.1415927410125732421875
    // as f64 = 3.141592653589793115997963468544185161590576171875
    // This is because the decimal number is not guaranteed to be the same
    // f32 can do about 6-9 significant figures
    // f64 can do about 15-17 significant figures

    let x = 10.0; // f64 (this is default unless specified)
    println!("x is {}", x);
    // > x is 10
    // because the default formating for print macro does not show decimal places if it is 0

    let y = 10.123456789123456789;
    println!("y is {}", y);
    // > y is 10.123456789123457
    // because we reach the limit of sigfigs

    let z: f32 = 10.123456789123456789; // f32 as specified
    println!("z is {}", z);
    // > z is 10.123457
    // because f32 has less percision than f64
}