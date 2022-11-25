fn main() {
    // declare a 2D array
    let parking_lot = [[1, 2, 3], 
                       [4, 5, 6]];

    // access the 2D array
    let number = parking_lot[0][1];
    println!("number is {}", number);
    // number is 2

    /* let parking_lot = [[1, 2, 3], 
                          [4, 5, 6, 7]]; */
    // Error: Array size mismatch
    // Lesson: All rows must have the same number of columns

    // declare a 3D array
    // let garage: [[[i32; 100]; 20]; 5];
    // Or define it at the same time
    let _garage = [[[0; 100]; 20]; 5];
}
