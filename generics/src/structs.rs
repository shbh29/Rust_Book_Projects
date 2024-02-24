
pub mod mixup;

#[derive(Debug)]
pub struct Point<T, U> {
    pub x: T,
    pub y: U,
}


pub fn structs_main() {
    let point = Point::<i32, f32> {
        x: 5, y: 6.0,
    };

    println!("x: {}, y: {}", point.x, point.y);

}