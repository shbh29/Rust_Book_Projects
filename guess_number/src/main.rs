use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
        let secret_number = rand::thread_rng().gen_range(1..=100);

        //println!("your secret number is: {secret_number}");
	println!("Guess the number!");
	println!("Enter the number");

        loop {

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Error reading the value");

            let guess : u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            println!("your guess {guess}");

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("too less!"),
                Ordering::Greater => println!("too high!"),
                Ordering::Equal => {
                    println!("You win");
                    break;
                }
            }
        }

}
