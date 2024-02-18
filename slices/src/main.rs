fn main() {
    let s = String::from("this is my sample sentence");

    let first_word_str = first_word(&s);
    println!("First word is: {first_word_str}");

    let first_word_str = first_word(&s[0..4]);
    println!("First word is: {first_word_str}");

    let first_word_str = first_word(&s[5..]);
    println!("First word is: {first_word_str}");

    let s = "this is my sample sentence";

    let first_word_str = first_word(&s);
    println!("First word is: {first_word_str}");

    let first_word_str = first_word(&s[0..4]);
    println!("First word is: {first_word_str}");

    let first_word_str = first_word(&s[5..]);
    println!("First word is: {first_word_str}");
}

fn first_word(s : &str) -> &str {
    let byte_arr = s.as_bytes();

    for (i, &character) in byte_arr.iter().enumerate() {
        if character == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
