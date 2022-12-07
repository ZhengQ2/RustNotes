fn main() {
    let rocket_fuel = produce_fuel();
    println!("Rocket fuel is {}", rocket_fuel);
}

// What would happen if we return a reference?
// fn produce_fuel() -> &String {
//     let new_fuel = String::from("RP-1");
//     &new_fuel
//     // this is a dangling reference
//     // because the reference is pointing to a value that is out of scope
//     // as new_fuel would be freed from memory after the function returns
// }

// Solution: Return a String
fn produce_fuel() -> String {
    let new_fuel = String::from("RP-1");
    new_fuel
}
