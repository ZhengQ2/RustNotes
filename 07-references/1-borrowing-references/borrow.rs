fn main() {
    let rocket_fuel = String::from("RP-1");
    let length = process_fuel(&rocket_fuel);
    println!("Rocket fuel is {} and length is {}", rocket_fuel, length);
}

// What if we also want to return the length of the propellant?
// It is overkill if we want to use tuples.
// Solution: Borrowing

// Borrowing
// - Accessing data without taking ownership
// - Borrowing is done by passing a reference to the data
fn process_fuel(propellant: &String) -> usize {
    // propellant is a reference to a String
    // so it does not take ownership of the data
    println!("Processing propellant {}...", propellant);
    let length = propellant.len();
    length
}