
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "Area of rectangle is: {} sq unit",
        rect.area()
    );
}

fn area(rect : &Rectangle) -> u32 {
    rect.width * rect.height
}
