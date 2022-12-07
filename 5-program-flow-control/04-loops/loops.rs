// Consider the situation:
// We have a chunk of code that we want to run multiple times
// In this case, we can use loops, they include:
// - loop: loops forever
// - while: loops while a condition is true
// - for: loops over a collection


fn main () {
    let mut count = 0;

    //loop
    
    let result = loop {
        count += 1;
        println!("count is {}", count);

        // however. we usually don't want to loop forever
        // so we can use break to exit the loop
        // if count == 10 {
        //     break;
        // }
        // Note: break will not stop the loop immediately, 
        // it will stop the loop after the current iteration

        // In Rust, break can also return a value
        if count == 10 {
            break count * 2;
        }

    }; // since we are using let, we need to add a semicolon here
    println!("result is {}", result);

}