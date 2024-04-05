use std::ops::Add;

#[derive(Debug,Clone,Copy,PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2};
    let p2 = Point { x: 1, y: 2};

    let p3 = p1 + p2;

    println!("p3 values: {:?}", p3);

}
