fn main() {
    let message = String::from("Greetings from Earth!");


    let first_word = get_first_word(&message);
    println!("first_word is {}", first_word);

    // problem: we want to get the second word of the message
    // We know the first word is 8 characters long, so
    // let second_word = get_first_word(&message[8..]);
    // This will not work because we are expecting a &String, not a slice(&str)

    // Rules: when string slice is passed as a parameter, the parameter type must be specified as &str
    // However, if we don't know if we are going to pass a String or a string slice, we can use &str
    // This is known as deref coercion

    // Deref Coercion
    // - Rust will automatically convert a reference to a type that implements the Deref trait into a reference to a type that the Deref trait converts to

    // So, change the data type of the parameter to &str would resolve the problem

    let second_word = get_first_word(&message[10..]);
    println!("second_word is {}", second_word);

    // Lesson: To avoid the problem of having to pass a String or a string slice, we can use &str as the parameter type
}

// fn get_first_word(s: &String) -> &str {
fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index]; // found a space!
        }
    }

    &s // no spaces found; input is a single word
}
