fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);

    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", no_dangle());

    // String Slices
    let s = String:: from("hello world");

    let _hello = &s[0..5];
    let _world = &s[6..11];

    // Other Slices
    let a = [1,2,3,4,5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3])
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/* fn dangle() -> &String { // dangle returns a reference to a String
    
    let s = String::from("hello"); // s is a new string
    
    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
*/

fn no_dangle() -> String {
    let s = String::from("hello"); 

    s // The solution!
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
           return &s[0..i];
        }
    }

    &s[..] 
}
