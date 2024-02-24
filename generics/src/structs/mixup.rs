use crate::structs;

impl<T, U> structs::Point<T, U> {
    fn mixup<X, Y>(self, other: structs::Point<X, Y>) -> structs::Point<T, Y> {
        structs::Point {
            x: self.x,
            y: other.y
        }
    }
}

pub fn main() {
    let p1 = structs::Point {
        x: 5,
        y: 5.2,
    };
    let p2 = structs::Point {
        x: 6,
        y: 6.2
    };

    println!("p1: {:?}", p1);

    let p3 = p1.mixup(p2);

    // println!("p1: {:?}", p1); // cannot be used further
    // println!("p2: {:?}", p2); // cannot be used further
    println!("p3: {:?}", p3);

}