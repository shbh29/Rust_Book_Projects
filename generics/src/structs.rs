struct Point<T> {
    x: T,
    y: T,
}


pub fn structs_main() {
    let point = Point::<i32> {
        x: 5, y:10
    };

    println!("x: {}, y: {}", point.x, point.y);

}