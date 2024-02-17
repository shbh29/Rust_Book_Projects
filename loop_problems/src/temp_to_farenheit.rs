use std::io;

fn main() {
    println! ("Please entre temprature in Celcious");
    let mut degree_celcious = String::new();

    io::stdin()
        .read_line(&mut degree_celcious)
        .expect("Could not fetch a number");

    let degree_celcious: i32 = degree_celcious
        .trim()
        .parse()
        .expect("Not a number");

    let farenheit = (degree_celcious * 9/5) + 32;

    println!("{degree_celcious}oC is {farenheit}oF temprature");
}
