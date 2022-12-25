// Slice
// - Reference to a contiguous sequence of elements in a collection
// - Can be used to borrow a section of an array, vector, or string
// - Commonly encountered as the string slice type: &str
// - String literals are stored in the binary and are therefore string slices

fn main() {
    let s = String::from("Hello, world!");
    let hello = &s[0..5];
    let world = &s[6..12];
    println!("{}, {}!", hello, world);

    // What if the slice is out of bounds?
    // let world = &s[6..14]; // panic!

    // So, if we want the slice to go to the end of the string:
    let world_new = &s[6..];
    println!("{}, {}", hello, world_new);
    
    // String Slices
    // - Length in bytes
    // - Recall that strings are UTF-8 encoded
    // - Range of indices must occur on valid UTF-8 character boundaries

    let s_in_chinese = String::from("你好，世界！");
    // let hello_in_chinese = &s_in_chinese[0..4];
    // This will panic because the first byte of the first character is not a valid UTF-8 character boundary
    // In this case, character 好 is 3 bytes long
    // So, we need to use the range 0..6：
    let hello_in_chinese = &s_in_chinese[0..6];
    let world_in_chinese = &s_in_chinese[9..15];
    println!("{}, {}!", hello_in_chinese, world_in_chinese);

    // Slices of Array
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("{:?}", slice);

}
