// Scope: Region of code where a variable is valid
// Variables are valid from the point of declaration (come into scope) 
// to the end of the scope (goes out of scope)

// Variable binding are constrained to live within a block of code
// The block is delimited by curly braces: {}

fn main() {

    {
        let planet = "Earth";
        println!("The planet is {}", planet);
        // > The planet is Mars
    }
    println!("The planet is {}", planet);
    // > ERROR: planet is not in scope
    // This is because the variable planet is valid only within the block
    // where it was declared, so planet is out of scope
}