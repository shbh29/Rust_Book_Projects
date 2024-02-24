use crate::sumarizer::{summary::Summary, tweet::Tweet};
use std::fmt::Display;

pub fn broadcast<T: Summary, U: Summary, X: Summary>(t: &T, u: &U, x: &X) {
    println!("Breaking News: {}", t.summarize());
    println!("Breaking News: {}", u.summarize());
    println!("Breaking News: {}", x.summarize());
}


pub fn display(s : &impl Display) {
    println!("{}", s);
}

pub fn show(s : &(impl Display + Summary)) {
    println!("\n In Today's show we have \n");
    println!("{}", s);
    println!("!!! Breaking News !!! \n {}", s.summarize());
}

pub fn get_tweet() -> impl Summary {
    Tweet {
        user: String::from("Something"),
        content: String::from("Something Else"),
        retweet: false,
    }
}