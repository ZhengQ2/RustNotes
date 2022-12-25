fn main() {
    // Char data type:
    // A single character
    // e.g. single letter, number, symbol, etc.
    // It stored a unicode scalar value
    // the size of a char is 4 bytes

    // Given that not like C/C++ who use ASCII,
    // Rust uses unicode scalar value, which allows
    // us to use characters and emojis etc.

    // Example:
    let letter = 'a';
    let number = '1';

    // We uses single quotes to represent a char
    // Note: no arithmetic operations are supported

    // Another way to define a char:
    let finger = '\u{261D}'; // Unicode scalar value
    println!("{}\n{}\n{}", letter, number, finger);

    // Output:
    // a
    // 1
    // ☝

    // Warning: some of the unicode scalar values are not supported
    // It depends on the support of unicode.

}
