

pub fn main() {
    let mut longest = "";
    let s1 = String::from("Jai Shree Ram");
    {
        let s2 = String::from("Jaish");
        longest = longest1(&s1, &s2);
        println!("Longest string is {longest}");
    }
    
}

fn longest1<'a>(s1 : &'a str, s2 : &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}


// below wont work
// fn longest<'a>(s1 : &'a str, s2 : & str) -> &'a str {
//     &String::from("dd").as_str()
// }

