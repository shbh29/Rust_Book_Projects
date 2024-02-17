
fn main() {

    let arr = ["A partridge in a pear tree."
        , "Two turtle doves,"
        , "Three French hens,"
        , "Four calling birds,"
        , "Five golden rings,"
        , "Six geese a-laying,"
        , "Seven swans a-swimming,"
        , "Eight maids a-milking,"
        , "Nine ladies dancing,"
        , "Ten lords a-leaping,"
        , "Eleven pipers piping,"
        , "Twelve drummers drumming,"];
    let num_str = ["first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"];
    // print the song twelve days of christmas
    for i in 0..arr.len() {
        println!();
        print_first_two_lines(num_str[i].to_string());
        for j in (0..i+1).rev() {
            println! ("{}", arr[j]);
        }
    }
    
}

fn print_first_two_lines(num_str: String) {
   //On the first day of Christmas,
    //my true love gave to me 
    println!("On the {num_str} day of Christmas");
    println!("my true love gave to me");
}
