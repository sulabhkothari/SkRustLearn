use std::fmt::Display;
use std::cmp::PartialOrd;
// One restriction to note with trait implementations is that we can implement a trait on a type only
// if either the trait or the type is local to our crate. For example, we can implement standard
// library traits like Display on a custom type like Tweet as part of our aggregator crate
// functionality, because the type Tweet is local to our aggregator crate. We can also implement
// Summary on Vec<T> in our aggregator crate, because the trait Summary is local to our aggregator crate.
// Without the rule, two crates could implement the same trait for the same type, and Rust wouldn’t
// know which implementation to use.


pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

//pub fn notify(item: impl Summary) {
//    println!("Breaking news! {}", item.summarize());
//}

pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}
// If we wanted this function to allow item1 and item2 to have different types, using impl Trait
// would be appropriate (as long as both types implement Summary). If we wanted to force both
// parameters to have the same type, that’s only possible to express using a trait bound, like this:
//pub fn notify<T: Summary>(item1: T, item2: T) {

// We can also specify more than one trait bound. Say we wanted notify to use display formatting on
// item as well as the summarize method: we specify in the notify definition that item must implement
// both Display and Summary. We can do so using the + syntax:
//pub fn notify(item: impl Summary + Display) {
//pub fn notify<T: Summary + Display>(item: T) {
// With the two trait bounds specified, the body of notify can call summarize and use {} to format item.
// Clearer Trait Bounds with where Clauses:
// Instead of fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
// Use
//  fn some_function<T, U>(t: T, u: U) -> i32
//    where T: Display + Clone,
//          U: Clone + Debug
//{

pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
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



// If we don’t want to restrict the largest function to the types that implement the Copy trait, we
// could specify that T has the trait bound Clone instead of Copy. Then we could clone each value in
// the slice when we want the largest function to have ownership. Using the clone function means
// we’re potentially making more heap allocations in the case of types that own heap data like
// String, and heap allocations can be slow if we’re working with large amounts of data.
// Another way we could implement largest is for the function to return a reference to a T value in
// the slice. If we change the return type to &T instead of T, thereby changing the body of the
// function to return a reference, we wouldn’t need the Clone or Copy trait bounds and we could
// avoid heap allocations.
pub fn largest<T>(list: &[T]) -> &T where T: PartialOrd {
    let mut largest = &list[0];
    let &x = &10;
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn largest2<T>(list: &[T]) -> T where T: PartialOrd+Copy {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}


// Using Trait Bounds to Conditionally Implement Methods
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
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

// We can also conditionally implement a trait for any type that implements another trait.
// Implementations of a trait on any type that satisfies the trait bounds are called blanket
// implementations and are extensively used in the Rust standard library. For example, the standard
// library implements the ToString trait on any type that implements the Display trait. The impl
// block in the standard library looks similar to this code:
// impl<T: Display> ToString for T {
//    // --snip--
// }
// Blanket implementations appear in the documentation for the trait in the “Implementors” section.

