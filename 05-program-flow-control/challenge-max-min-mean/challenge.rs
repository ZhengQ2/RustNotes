// Problem: Determine the max, min, and mean of an array
// Mean = sum of all elements / number of elements
fn main() {
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32;
    let mut min: i32;
    let mut mean: f64;

    /* Your code goes here */
    max = i32::MIN;
    min = i32::MAX;
    let mut count = 0.0;
    let mut sum = 0.0;

    for item in numbers {
        if item > max {
            max = item;
        }
        if item < min {
            min = item;
        }
        sum += item as f64;
        count += 1.0;
    }

    mean = sum / count;

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests passed!");
}