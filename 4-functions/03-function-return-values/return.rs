fn main() {
    let result = square(13);
    println!("result is {}", result);
    let result = square_with_print(13);
    println!("result is {}", result);
    let result = square_with_origin_val(13);
    // Issue: Using the default print format of a tuple will produce an error
    // println!("result is {}", result);
    // Solution: use the debug format
    println!("result is {:?}", result);
}

// for return values, we use the -> operator, followed by the return data type
// there is no need for a variable name, like the argument
fn square(x: i32) -> i32 {
    println!("squaring {}", x);
    x * x
    // Recall that line 8 is an expression
    // In rust, if the last line of a function is an expression, it will be returned
}

// Problem: What if we want to a return value earlier?
// Solution: Use the return keyword

fn square_with_print(x: i32) -> i32 {
    println!("squaring {}", x);
    return x * x;
    println!("this line will never be printed");
}

// Problem: What if we want to return not one, but two variables?
// Solution: Use a tuple
fn square_with_origin_val(x: i32) -> (i32, i32) {
    println!("squaring {}", x);
    return (x, x * x);
}

// What happened if there is no return value?
// Rust will return a unit type

// Unit data type
// - Used when there is no other meaningful return value
// - Represented by the empty tuple ()
// - It can be used as a return type, however, it is not necessary

fn no_return() -> () {
    println!("no return");
}
