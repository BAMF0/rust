fn main() {
    let s = String::from("hello world");

    let len = s.len();

    let hello = &s[..5];
    let world = &s[6..];

    let hello_world = &s[..];

    let a = [1,2,3,4,5];
    let slice = &a[1..3];

    assert_eq!(slice, &[2,3])
}