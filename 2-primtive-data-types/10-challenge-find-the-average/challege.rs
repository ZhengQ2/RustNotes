// Topic: different data types and operators

// Problem:
// - Write a program that calculates the average of three numbers

fn main() {
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    // Key points:
    // f32 is less percise than f64
    // the default type for floating point numbers is f64
    // Thus, b is more precise than c
    // a is an integer, so it will be converted to a floating point number

    /* Your code goes here */
    let a1 = a as f64; // casting
    let c1 = c as f64; // casting
    let average = (a1 + b + c1) / 3.0;
    //                            ^ the easiest way to get a floating point number

    assert_eq!(average, 45.1);
    println!("The average is {}", average);
}
