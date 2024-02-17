use std::io;

fn main() {

    println!("Enter a number");

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Could not get user input");

    let number: i32 = number.trim().parse().expect("Please enter a valid number");

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3 and 2.");
    }


}
