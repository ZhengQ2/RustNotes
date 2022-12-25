// Goal:
// Write a function to remove all leading and trailing spaces from a string.
// - Function name: trim_spaces
// - Parameter: string reference or string slice
// - Return value: string slice

fn main() {
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");
    
    let test2 = String::from("   There's space in front.");
    assert_eq!(trim_spaces(&test2), "There's space in front.");
    
    let test3 = String::from("There's space to the rear. ");
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");   
    
    let test4 = "  We're surrounded by space!    ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");
    
    let test5 = "     ";
    assert_eq!(trim_spaces(test5), "");
    
    let test6 = "";
    assert_eq!(trim_spaces(test6), "");
    
    let test7 = " 🚀 ";
    assert_eq!(trim_spaces(test7), "🚀");
    println!("Tests passed!");
}

/* YOUR CODE GOES HERE */
fn trim_spaces(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut start = bytes.len();
    let mut end = 0;
    for (index, &item) in bytes.iter().enumerate() {
        if item != b' ' {
            start = index;
            break;
        }
    }
    for (index, &item) in bytes.iter().rev().enumerate() {
        if item != b' ' {
            end = bytes.len() - index;
            break;
        }
    }
    if start >= end {
        return "";
    }
    &s[start..end]
}
