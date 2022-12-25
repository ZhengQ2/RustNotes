fn main() {
    // Comparison operations:
    // >, <, >=, <=, ==, !=
    // The same as C/C++

    // Example:
    let a = 1;
    let b = 2;
    println!("a is {} and b is {}", a, b);
    println!("a EQUALS To be is {}", a = b);
    println!("a NOT EQUALS To be is {}", a != b);
    println!("a GREATER THAN b is {}", a > b);
    println!("a GREATER THAN OR EQUALS TO b is {}", a >= b);
    println!("a LESS THAN b is {}", a < b);
    println!("a LESS THAN OR EQUALS TO b is {}", a <= b);

    // Output:
    // a is 1 and b is 2
    // a EQUALS To be is false
    // a NOT EQUALS To be is true
    // a GREATER THAN b is false
    // a GREATER THAN OR EQUALS TO b is false
    // a LESS THAN b is true
    // a LESS THAN OR EQUALS TO b is true 

    // Note that these ops support more than integer data types
    // e.g.
    let c = true;
    let d = false;
    println!("c is {} and d is {}", c, d);
    println!("c EQUALS To be is {}", c = d);
    println!("c NOT EQUALS To be is {}", c != d);
    println!("c GREATER THAN d is {}", c > d);
    println!("c GREATER THAN OR EQUALS TO d is {}", c >= d);
    println!("c LESS THAN d is {}", c < d);
    println!("c LESS THAN OR EQUALS TO d is {}", c <= d);

    // Output:
    // c is true and d is false
    // c EQUALS To be is false
    // c NOT EQUALS To be is true
    // c GREATER THAN d is true
    // c GREATER THAN OR EQUALS TO d is true
    // c LESS THAN d is false
    // c LESS THAN OR EQUALS TO d is false
    // Note that true corresponds to 1 and false corresponds to 0
    
    // However, we cannot compare different data types
    // Would produce Error: no implementation for `bool == {integer}`
}
