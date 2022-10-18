pub mod aggregator {
    use std::fmt::{Debug, Display};
    // Conditionally implement methods
    struct Pair<T> {
        x: T,
        y: T,
    }

    // Pair fields are private so a constructor is needed
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }
    
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }


    pub trait Summary {
        fn summarize_author(&self) -> String;

        fn summarize(&self) -> String {
            format!("Read more from {}...", self.summarize_author())
        }
    }

    //syntax sugar for trait bound:
    // pub fn notify<T: Summary + Display>(item: &T) {...}
    // this can express more complexity in other cases
    // trait bound constrains the function parameters
    // to have the same type
    pub fn notify(item: &(impl Summary + Display)) {
        println!("Breaking news! {}", item.summarize());
    }

    // Clearer Trait Bounds with where Clauses
    fn some_function<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        10
    }



    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize_author(&self) -> String {
            format!("@{}", self.author)
        }
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        } 
    }
}
