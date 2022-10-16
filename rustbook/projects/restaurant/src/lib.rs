use std::fmt::Result;
use std::io::Result as IoResult;

// import std::io and std::io::Write
use std::io::{self, Write};
// import multiple from same namespace
// use std::{cmp::Ordering, io};

// the glob operator, bring all public items into scope
use std::collections::*;

fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { 
                toast: String::from(toast), 
                seasonal_fruit: String::from("peaches"),
            } 
        } 
    }
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

mod front_of_house;

// re-exporting hosting outside module
// external paths can now use restaurant::hosting::add_to_waitlist() instead
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    //  crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // use keyword
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries")

    let order1 = back_of_house::Appetizer::Soup; 
    let order2 = back_of_house::Appetizer::Salad;
}
