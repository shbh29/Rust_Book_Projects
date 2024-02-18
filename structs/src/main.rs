
struct EmptyStruct;
#[derive(Debug)]
struct Color(u8, u8, u8);
#[derive(Debug)]
struct Point(u8, u8, u8);

impl EmptyStruct {
    fn hello() {
        println!("Hello from empty struct");
    }
}

fn main() {

    EmptyStruct::hello();

    let black = Color (0, 0, 0);
    let origin = Point (0, 0, 0);

    dbg!(&black);
    dbg!(&origin);
}

