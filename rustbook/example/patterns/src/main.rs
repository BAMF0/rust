fn main() {
    mult_patterns();
}

// You can match multiple patterns using the | syntax, which is the pattern
// or operator
fn mult_patterns() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}
