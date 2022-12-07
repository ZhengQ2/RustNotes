fn main() {
    // Problem: What if we want to decide run/not runa a chunk of program based on a condition?
    // Solution: Conditional Execution

    let mut x = 3;

    // 1. if expression
    if x == 3 {
        println!("x is 3");
    }

    x = 4;

    if x != 3 {
        println!("x is NOT 3");
    }

    // x ==/!= 3 returns boolean true/false

}
