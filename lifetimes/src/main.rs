


fn main() {
    let s1 = String::from("Jai Shree Ram");
    let s2 = String::from("Jaish");
    let longest = longest(&s1, &s2);
    println!("Longest string is {longest}");
}

fn longest<'a>(s1 : &'a str, s2 : &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
