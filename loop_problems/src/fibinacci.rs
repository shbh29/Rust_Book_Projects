use std::io;

fn main() {
    println!("Please enter nth fibonacci series");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Error in io op");

    let n = n.trim()
        .parse()
        .expect("not a number");

    let mut f1 = 0;
    let mut f2 = 1;
    let mut f3 = f1 + f2;

    println!("0th fibonacci number is {f3}");

    for i in 1..n {
        f1 = f2;
        f2 = f3;
        f3 = f1 + f2;
        println!("{i}th fibonacci number is {f3}");
    }

}

