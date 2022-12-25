// Problem: 
// - Convert temperature from Celsius to Fahrenheit
// - Write a function called celsius_to_fahrenheit
// - Input: Celsius
// - Return: Fahrenheit
// - Formula: (C * 1.8) + 32.0 = F

fn main () {
    let celsius_temp = 23.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);
    
    assert_eq!(fahrenheit_temp, 73.4);
    println!("Test passed!");
}

/* Your code goes here */
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 1.8) + 32.0
}