fn main() {
    // rust build in arithmetic operators:
    // + (addition), - (subtraction), * (multiplication), / (division), % (modulus)
    let a = 10;
    let b = 3;
    let c = a + b;
    let d = a - b;
    let e = a * b;
    let f = a / b;
    let g = a % b;
    println!("c is {}", c);
    println!("d is {}", d);
    println!("e is {}", e);
    println!("f is {}", f);
    println!("g is {}", g);
    // > c is 13
    // > d is 7
    // > e is 30
    // > f is 3
    // > g is 1

    // note that since both a and b are integers, the result of f is an integer
    let a1 = 10.0;
    let b1 = 3.0;
    let f1 = a1 / b1;
    println!("f1 is {}", f1);
    // > f1 is 3.3333333333333335
    // now since a1 and b1 are floats, the result of f1 is a float

    // Consider: {integer} / {float}
    // let f2 = a / b1;
    // println!("f2 is {}", c2);
    // Produces an error, since this division operation is not defined
    // >> error: cannot divide `{integer}` by `{float}`
    // Reason behind it is it leaves ambiguity between integer and float
    // Rust does not allow this operation to be performed

    // Solution: Casting one of the variables to the same data type as the other variable
    let f2 = a as f64 / b1;
    println!("f2 is {}", f2);
    // > f2 is 3.3333333333333335

    // Be careful with casting!!!
    // Sometimes casting may result in loss of precision
    // int to float is usually fine, but float to int is not
    // note that when casting to int from float, rust uses floor
    // casting between one int to another int may also leads to weird behaviors
    // e.g.
    // - 3 as f64 -> 3.0
    // 3.9 as i32 -> 3
    // 300 as u8 -> 44
    // -300 as u32 -> 4294966996
    // Luckily, it will not have any "undefined behavior" as C languages do
    // it follows specific rules even if it is out of scope when casting between
    // two integer types

    // When multiple operations occured in a single assignment, * / % has priority over + -
    // parenthesis are able to change the order of operations
    let h = a as f64 / b1 + 1.0;
    println!("h is {}", h);
    // > h is 4.333333333333334
    let h1 = a as f64 / (b1 + 1.0);
    println!("h1 is {}", h1);
    // > h1 is 2.5
}