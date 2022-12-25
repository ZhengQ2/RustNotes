// Using while loop to iterate over an array may not be the best way to do it
// We can use for loop to do the same thing

fn main() {
    let message = ['H', 'e', 'l', 'l', 'o'];

    // for loop
    for item in message {
        println!("letter is {}", item);
    }
    // Here, rust converts the array into an iterator

    // Iterator:
    // - Implement the Iterator trait, i.e. what is the next item (next()) and what is the last item

    // Side note: this feature is only supported in Rust 2018
    // Rust 2015 does not support this feature
    // the equivalent code in Rust 2015 is:
    // for item in message.iter() {
    //     println!("letter is {}", item);
    // }
    
    // Consider: what if we also want to know the index of each item?
    // Solution: use the enumerate() method

    for (index, item) in message.iter().enumerate() {
        println!("letter {} is {}", index, item);
    }

    // Adding up with break:

    for (index, item) in message.iter().enumerate() {
        println!("letter {} is {}", index, item);
        if item == &'e' {
            break;
        }
        // Since item is a reference to the item in the array, we need to use & to get the value
    }

    // Another usage of for loop: range
    for number in 0..5 {
        println!("number is {}", number);
    }

}