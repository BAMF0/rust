fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    for number in 1..21 {
        println!("Fibonacci of {} = {}", number, fib(number));
    }
}

// Return the n:th number in the fibonnaci sequence
fn fib(n: i32) -> i32 {
    match n {
        1 => 1,
        2 => 1,
        _ => fib(n-1) + fib(n-2)
    }
}
