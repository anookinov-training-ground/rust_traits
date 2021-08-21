use std::fmt::Display;
use std::fmt::Debug;

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        // String::from("(Read more...)")
        format!("(Read more from {}...", self.summarize_author())
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
        // format!("{}: {}", self.username, self.content)
        format!("@{}", self.username)
    }
}

// traits as parameters - impl Trait syntax
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// trait bound
pub fn notify_generic<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_two(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}

pub fn notify_two_generic<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}

// multiple trait bounds with the + syntax
pub fn notify_multiple(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_multiple_generic<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    println!("{}, {:#?}", t, u);
    10
}

// clearer trait bounds with where clauses
pub fn some_function_where<T, U>(t: &T, u: &U) -> i32
    where   T: Display + Clone,
            U: Clone + Debug
{
    println!("{}, {:#?}", t, u);
    10
}