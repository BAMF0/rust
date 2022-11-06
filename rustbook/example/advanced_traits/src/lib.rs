use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
   type Output = Millimeters; 

   fn add(self, other: Meters) -> Self::Output {
       Millimeters(self.0 + (other.0 * 1000))
   }
}

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
}

impl Counter {
   fn new() -> Counter {
       Counter { count: 0 }
   } 
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
