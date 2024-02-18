#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_contain(&self, other : &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
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

    // can_contain 
    // dbg! to print values
    let rect2 = Rectangle {
        width: 25,
        height: 45,
    };

    let rect3 = Rectangle {
        width: 35,
        height: 55,
    };

    println! ("rect1 can contain rect2: {}", rect.can_contain(&rect2));
    println! ("rect1 can contain rect3: {}", rect.can_contain(&rect3));

    // dbg! macro
    let scale = 2;
    let rect4 = Rectangle {
        width: dbg!(rect.width * scale),
        height: rect.height,
    };

    dbg!(&rect4);

}

fn area(rect : &Rectangle) -> u32 {
    rect.width * rect.height
}
