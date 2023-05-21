use std::cmp::PartialOrd;
use std::fmt::Display;
use std::println;

fn largest<T: PartialOrd + Display>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    println!("Largest: {}", largest);
    largest
}

fn smallest<T>(list: &[T]) -> &T
where
    T: PartialOrd + Display,
{
    let mut smallest = &list[0];
    for item in list.iter() {
        if item < smallest {
            smallest = item;
        }
    }
    println!("Smallest: {}", smallest);
    smallest
}

fn longest_string<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn bogus<'a>(a: i32, b: i32) -> &'a str {
    if a > b {
        "First is bigger"
    } else {
        "Second!"
    }
}


// We can't return result.as_str() as the reference of that string dies
// inside the function, meaning other functions calling this function
// can't borrow it. What a shame.
fn this_returns_string() -> String {
    let result = String::from("This is a test");
    result
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
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
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let result = smallest(&number_list);
    println!("The smallest number is {}", result);

    let string1 = String::from("abcd");
    let string2 = "xyz";
    println!("The longest string is {}", longest_string(&string1, &string2));

    println!("Bogus {}", bogus(1,2));
    println!("What {}", this_returns_string());

    let tweet = Tweet {
        username: String::from("amithmkini"),
        content: String::from("This is a tweet"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
