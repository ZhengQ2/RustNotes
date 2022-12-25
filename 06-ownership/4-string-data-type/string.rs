// Previouly:
// - Integer
// - Floating Point
// - Boolean
// - Character
// - Array
// - Tuple

// All of them are stored on the stack, which means that the
// size of these types must be known at compile time and
// they are stored entirely on the stack.

// String, however, is stored on the heap, and is a growable,
// mutable, UTF-8 encoded string type.

// Two types of strings:
// - String Literal
//   - Hardcoded into the excutable
//   - Immutable
//   - Value must be known at compile time
// - String Type
//   - Stored on the heap
//   - Mutable
//   - Value can be changed at runtime

fn main() {
    // String Type: 
    let mut message = String::from("Earth");
    // The chars 'E', 'a', 'r', 't', 'h' are stored on the stack
    // The memory address of it is stored on the heap, with additional metadata
    // like the length of the string and the capacity of the string
    println!("message is {}", message);

    message.push_str(" is home.");
    // The string is now "Earth is home"
    println!("message is {}", message);
}
