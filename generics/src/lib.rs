pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl Summary for NewsArticle {} //implementing default trait

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("by {}", self.author)
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
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait Bound Syntax //
////////////////////////

// Simple variant
pub fn _notify2<T: Summary>(_item: &T) {}

use std::fmt::{Debug, Display};

// Variant 2.
// both parameters must be of the same type

pub fn _notify3<T: Summary>(_item1: &T, _item2: &T) {}
pub fn _notify<T: Summary + Display>(_item: &T) {} // Multiple Triat bounds variant

// Variant 3.
// both parameters may not be of same type but both must implement Summary Trait

pub fn _notify4(_item1: &impl Summary, _item2: &impl Summary) {}
pub fn _notify5(_item: &(impl Summary + Display)) {} // Multiple Triat bounds variant

// Variant 4. (using where clause)
fn _some_function<T, U>(_t: &T, _u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{}

// This is the same as the function below
fn _some_function2<T: Display + Clone, U: Clone + Debug>(_t: &T, _u: &U) {}

fn _returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horese_ebooks"),
        content: String::from("of course, as you probabaly already know, people"),
        reply: false,
        retweet: false,
    }
}
