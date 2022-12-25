fn main() {
    // Group together values of different types
    // Elements are ordered
    // Stored in a fixed size, contiguous block of memory
    // Data type must be known at compile time

    // declare a tuple
    let stuff = (10, 3.14, 'x');
    // Note: not like an array, we use round brackets
    
    // access the tuple
    let first_item = stuff.0;
    println!("first item is {}", first_item);
    // first item is 10 

    // Problem: What if we don't want to use the default data types?
    // Solution:
    let _new_stuff: (u8, f32, char) = (10, 3.14, 'x');

    // Problem: How to change the values in a tuple?
    // Solution: Use a mutable tuple
    let mut mutable_stuff = (10, 3.14, 'x');
    mutable_stuff.0 += 3;
    println!("mutable stuff is {:?}", mutable_stuff);

    // Problem: How to access all the values in a tuple?
    let (a, b, c) = mutable_stuff;
    println!("a is {}, b is {}, c is {}", a, b, c);
}
