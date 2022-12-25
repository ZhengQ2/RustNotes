// Recall:
// - Resources can only have one owner at a time

fn main() {
    let mut outer_planet: String;
    // Move
    {
        let mut inner_planet = String::from("Mercury");
        println!("inner_planet is {}", inner_planet);
        outer_planet = inner_planet;
        // Here is copies the pointer of inner_planet to outer_planet
        // This is similar to a shallow copy, but now the owner of the pointer is outer_planet
        // So this is called a move
        // inner_planet is now invalid

        // println!("inner_planet is {}", inner_planet);
        // This will not compile because inner_planet is now invalid after the move
    }
    println!("outer_planet is {}", outer_planet);

    // Clone
    // Problem: What if we want to use inner_planet? i.e. Perform a deep copy?
    // Solution:
    {
        let mut inner_planet = String::from("Mars");
        println!("inner_planet is {}", inner_planet);
        outer_planet = inner_planet.clone();
        // This is a deep copy, so now inner_planet and outer_planet are both valid
        println!("inner_planet is {}", inner_planet);
        // This will compile because inner_planet is still valid
        // Now what if we mutate inner_planet?
        inner_planet.clear();
        println!("inner_planet is {}", inner_planet);
    }
    println!("outer_planet is {}", outer_planet);
    // We can see that change of inner_planet does not affect outer_planet
    // This is because outer_planet is a deep copy of inner_planet

    // Copy
    // What if the data does not held by the heap, but by the stack?
    // For example, integers
    let outer_planet: i32;
    {
        let mut inner_planet = 1;
        outer_planet = inner_planet;
        // Since both variables are on the stack, instead of copy the pointer
        // Rust will copy the value of inner_planet to outer_planet
        // So there is no ownership change
        // This is called a copy
        // inner_planet is still valid
        println!("inner_planet is {}", inner_planet);

        // Now what if we mutate inner_planet?
        inner_planet += 1;
        println!("inner_planet is {}", inner_planet);
    }
    println!("outer_planet is {}", outer_planet);
    // We can see that change of inner_planet does not affect outer_planet
    // This is because outer_planet is a copy of inner_planet
}
