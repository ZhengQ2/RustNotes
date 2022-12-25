fn main() {
    let mut count = 0;

    // while loop

    while count < 10 {
        count += 1;
        println!("count is {}", count);
    }
    // Note: although the result of the code presented above is the same as the code in loops.rs,
    // while loop and loop are not the same thing
    // while loop is a conditional loop, it will only run if the condition is true
    // loop is an infinite loop, it will run forever unless we use break to stop it
    // also note that we can use break to stop a while loop, but it will not return a value

    // Now, let's consider iterating over an array
    count = 0;
    let letters = ['a', 'b', 'c'];

    while count < letters.len() {
        // we can use the len() method to get the length of an array
        // this helps the while loop to know when to stop and will not cause an error
        println!("letter is {}", letters[count]);
        count += 1;
    }
}