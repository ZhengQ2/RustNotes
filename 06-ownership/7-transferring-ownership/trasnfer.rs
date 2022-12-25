// Difference between stack and heap also applies to passing variables to functions
fn main() {
    let rocket_fuel = 1;
    process_fuel_int(rocket_fuel);
    println!("rocket_fuel is {}", rocket_fuel);
    // This will compile because rocket_fuel is an integer, which is on the stack
    // So rocket_fuel still owns the value 1

    let rocket_fuel = String::from("RP-1");
    process_fuel_str(rocket_fuel);
    // println!("rocket_fuel is {}", rocket_fuel);
    // This will not compile because rocket_fuel is a String, which is on the heap
    // So rocket_fuel no longer owns the value "RP-1"

    // Solution 1: Use clone
    let rocket_fuel = String::from("RP-1");
    process_fuel_str(rocket_fuel.clone());
    println!("rocket_fuel is {}", rocket_fuel);

    // Solution 2: Let the function return a value
    let rocket_fuel = String::from("RP-1");
    let rocket_fuel = process_fuel_rtn(rocket_fuel);
    println!("rocket_fuel is {}", rocket_fuel);
}

fn process_fuel_int(mut propellant: i32) {
    // propellant is a copy of rocket_fuel because it is on the stack
    propellant += 1;
    println!("processing propellant {}...", propellant);
}

fn process_fuel_str(propellant: String) {
    // propellant is a move of rocket_fuel because it is on the heap
    println!("processing propellant {}...", propellant);
}

fn process_fuel_rtn(propellant: String) -> String {
    println!("processing propellant {}...", propellant);
    propellant
}