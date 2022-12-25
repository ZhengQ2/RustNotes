fn main() {
    let mut rocket_fuel = String::from("RP-1");
    let length = process_fuel(&mut rocket_fuel);
    // Similar to we need to declare a mutable variable
    // we also need to declare a mutable reference
    println!("Rocket fuel is {} and length is {}", rocket_fuel, length);
}

fn process_fuel(propellant: &mut String) -> usize {
    println!("Processing propellant {}...", propellant);
    propellant.push_str(" is highly flammable!");
    let length = propellant.len();
    length
}

// Limitations of mutable references
// - After a mutable reference is created, no more references can be created
// - This is to prevent data races
