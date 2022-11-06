use std::fmt;
use std::ops::Add;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot") 
    }
}

impl Animal for Dog {
   fn baby_name() -> String {
       String::from("puppy")
   } 
}


#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y) 
    } 
}

impl OutlinePrint for Point {
    
}

impl Add for Point {
   type Output = Point; 

   fn add(self, other: Point) -> Point {
        Point { 
           x: self.x + other.x,
           y: self.y + other.y,
        }
   }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0} + Point { x: 2, y: 3},
        Point { x: 3, y: 3}
        );
    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
