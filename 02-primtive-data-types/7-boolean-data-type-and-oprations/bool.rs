fn main() {
    // Two possible values: true or false
    // represeted by 1 or 0, similar to C/C++

    //Oprations:
    // AND: &&
    // OR: ||
    // NOT: !
    // XOR: ^
    // The same op as bitwise op, but with bool data type

    // Example:
    let a = true;
    let b = false;
    
    println!("a is {} and b is {}", a, b);
    println!("NOT a is {}", !a);
    println!("a AND b = {}", a && b);
    println!("a OR b = {}", a || b);
    println!("a XOR b = {}", a ^ b);
    // Output:
    // a is true and b is false
    // NOT a is false
    // a AND b = false
    // a OR b = true
    // a XOR b = true

    // Additionally, there is a special or/and op
    // which is short-circuiting or/and: |/&

    // Example:
    let c = (a ^ b) || do_something('c');
    // Would produce an error even if a ^ b is false
    let d = (a ^ b) | do_something('d');
    // Would not produce an error since a ^ b is false
    // and the op is short-circuiting
}

fn do_something(c: char) -> bool {
    println!("I am doing something when {} called me", c);
    return false;
}
