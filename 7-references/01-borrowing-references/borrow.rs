fn main() {
    let rocket_fuel = String::from("RP-1");
    let rocket_fuel = process_fuel(rocket_fuel);
    println!("Rocket fuel is {}", rocket_fuel);
}

fn process_fuel(propellant: String) -> String {
    println!("Processing propellant {}...", propellant);
    propellant
}