fn main() {
    let value = 0b1111_0101;
    // rust allows to insert optional '_' chars to help better visualization
    // default data type is i32
    let value1 = 0b1111_0101u8;
    // now rust makes this binary integer stores in u8
    println!("value is {}", value);
    
    // Consider: What if we also want to print it as a binary number
    // Answer: Use println formatting.
    println!("value is {:08b}", value1);
    // Here, 'b' asks the print macro to try to print every thing in binary
    // '8' is the number of bits to display
    // '0' means to fill the beginning of the number with 0
    
    // When we use add, multiply, etc, we treat them as a whole number
    // What if we want a calculation based on individual bits?
    // e.g. what if the integer represents a set of 8 switches
    // Solution: Bitwise Ops
    // this includes: NOT, AND, OR, XOR, SHIFT
    // Truth table see CS245
    
    // Example:
    // NOT Op:
    // Useage: To reverse all bits
    let mut not = 0b11110101u8;
    not = !not;
    println!("not: {:08b}", not);
    
    // AND Op:
    // Usage 1: To change one of the bits to 0
    let mut and = 0b11110101;
    and = and & 0b11110111;
    println!("and: {:08b}", and);
    // Usage 2: To check for specific bit
    and = and & 0b00000010;
    if and == 0{
        println!("the second digit is 0");
    } else {
        println!("the second digit is 1");
    }

    // OR Op:
    // Usage: To change one of the bits to 1
    let mut or = 0b10110101;
    or = or | 0b01000000;
    println!("or: {:08b}", or);
    
    // XOR Op:
    // Usage: To compare two bit patterns
    let mut xor = 0b10110101;
    xor = xor ^ 0b01010101;
    println!("xor: {:08b}", xor);

    // SHIFT Op:
    // Usage: To shift bit pattern left to right by a certain number of bits
    // Beginning bits will be filled with 0
    let mut shift = 0b10110101u8;
    shift = shift << 4;
    println!("shift: {:08b}", shift);
    shift = shift >> 2;
    println!("shift: {:08b}", shift);

}
