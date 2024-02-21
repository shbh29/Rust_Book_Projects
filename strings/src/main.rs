fn main() {
    // String reverse operation
    //

    let s = "abcdefghij";

    let s_rev  = s.chars()
        .rev()
        .collect::<String>();

    println!("reversed s: {s_rev}");

    // sub string operations
    //println!("substring: {}",s);
}
