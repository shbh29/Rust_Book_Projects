fn main() {
    // concatinating two string, format! macro, chars(), bytes().
    // lets try operations like string reverse and sub_string operations.
    let s1 = String::from("Hello, ");
    let s2 = String::from("World !!");
    let s = s1 + &s2;
    println!("String concat: {}", s);

    //format!
    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");

    let game_name = format!("{tic}-{tac}-{toe}");
    println!("game name: {game_name}");

    // print all chars of string
    let mystr = "lorem lorem lispsum loren on sunday";
    print!("\nmy string by chars: ");
    for c in mystr.chars() {
        print!("{c} ");
    }
    println!();

    assert_eq!(mystr.chars().count(), mystr.bytes().len());

    // print by bytes of string
    print!("\nmy string from bytes: ");
    for c in mystr.bytes() {
        print!("{} ", c as char);
    }
    println!();



}
