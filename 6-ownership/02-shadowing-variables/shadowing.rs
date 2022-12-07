// Rust's variables are immutable by default
// However, Rust allows you to declare a variable with the same name as a previous variable
// This is called shadowing
// When we access the variable, Rust will use the most recent value

fn main() {
    let planet = "Earth";
    println!("The planet is {}", planet);
    // > The planet is Earth
    {
        let planet = "Mars";
        println!("The planet is {}", planet);
        // > The planet is Mars
        // The variable planet is now bound to the value "Mars"
        // This is an example of shadowing

        // Shadowing is different from reassignment
        // Reassignment is when we change the value of a variable
        // Shadowing is when we declare a new variable with the same name as a previous variable
        
        // Shadowing allows us to change the type of a variable
        let mut planet = 4;
        println!("The planet is {}", planet);
        // Note that here we not only change the data type, but also set it to a mutable variable
        planet = 5;
        println!("The planet is {}", planet);
    }
    // The variable planet is now bound to the value "Earth"
    // Since all previous shadowings of planet are out of scope, the variable planet is now bound to the value "Earth"
    println!("The planet is {}", planet);
    
}
