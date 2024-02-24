use std::fmt::Display;



fn main() {
    lifetimes::structs_lifetime::main();
    lifetimes::simple_lifetime::main();

    let s: &'static str = "I have a static lifetime.";

    println!("{s}");
    let mut longest = "";
    let s1 = String::from("Jai Shree Ram");
    {
        let s2 = String::from("Jaish");
        longest = longest2(&s1, &s2, "Comparision Begins");
        println!("Longest string is {longest}");
    }
    
}

fn longest2<'a, T>(s1 : &'a str, s2: &'a str, ann: T) -> &'a str where T: Display {
    println!("{}", ann);
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}