// Scalar Data Types:
// - Integer
// - Floating-Point
// - Boolean
// - Char

// Compound Data Types:
// - Array
// - Tuple

fn main() {
    // Array Data Type
    // - Collection of elements of the same data type
    // - Stored in order
    // - Stored in contiguous memory locations
    // - Fixed length

    // Array is mainly used to store sequential data
    // e.g. Time-Series Data

    // Array Declaration
    let letters = ['a', 'b', 'c'];
    
    // Array Access
    let first_letter = letters[0];
    println!("first letter is {}", first_letter);

    // Edit item in array
    // Note 1: To allow editing, the array must be declared as mutable
    // Note 2: The new value must be of the same data type as the array
    let mut new_letters = ['a', 'b', 'c'];
    new_letters[0] = 'x';
    println!("first letter is {}", new_letters[0]);
    
    // What if we want an array with a given size but we don't know the variables?
    // Solution: Declare, but not initialize
    let numbers: [i32; 5];
    // Array numbers is declared with a size of 5 and data type i32, but not initialized

    // println!("last number is {}", numbers[4]);
    // Error: Array numbers is not initialized
    // This is an compile-time error

    // So we need to initialize the array
    // numbers = [0, 0, 0, 0, 0];
    // Now we can access the array

    // Problem: What if the array is much larger?
    numbers = [0; 5];
    println!("last number is {}", numbers[4]);
    // number = [0, 0, 0, 0, 0];
    // This is a shortcut to initialize an array with a given size and data type

    // What if we access an index that is out of bounds?
    // numbers[5];
    // Error: Index out of bounds
    // This is a compile-time error

    // What if we access an index that is out of bounds, but using a variable?
    // let index = numbers.len(); /* <- Access the length of the array */
    // println!("last number is {}", numbers[index]);
    // Note: index data type must be usize
    // Error: Index out of bounds
    // This is a runtime error

    // About usize
    // - Unsigned integer data type
    // - Size is based on number of bytes needed to store the address of the largest object
    // - 32-bit: 4 bytes
    // - 64-bit: 8 bytes

}
