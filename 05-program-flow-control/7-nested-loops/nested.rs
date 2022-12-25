fn main() {
    // Consider:
    let mut matrix = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];

    // How do we print out all the elements in the matrix?
    // Solution: nested loops

    for row in matrix {
        for item in row {
            print!("{}\t", item);
        }
        println!();
    }

    // What if we want to change the value of each item in the matrix?
    // Solution: use iter_mut() method

    for row in matrix.iter_mut() {
        for item in row.iter_mut() {
            *item += 10;
            print!("{}\t", item);
        }
        println!();
    }

    
}