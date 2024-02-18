fn main() {
    let s = takes_ownership_from();

    println!("Got ownership of {s}");

    let stri = takes_ownership_and_gives_back(&s);

    println!("I transferred the ownership and brought back : {stri}: {}", &stri);

    passing_ownership_to(s);

    //println!("this should be illegal to access! {s}")
}

fn takes_ownership_from() -> String {
    let s = String::from("hello");

    s
}

fn takes_ownership_and_gives_back(s : &String) -> &String {
    s
}

fn passing_ownership_to(mut s : String) {// immutable reference passed mutable String and modified.
    println!("I have ownership of {s}");

    s.push_str(", world");

    println!("I modified value of s to {s}");
}



