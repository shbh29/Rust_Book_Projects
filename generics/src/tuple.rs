use crate::structs;

#[derive(Debug)]
enum PointLocations {
    Origin1,
    Origin(structs::Point<i32, f32>),
}


pub fn main() {
    let p1 = structs::Point {
        x:32, y:54.2,
    };

    let origin = PointLocations::Origin(p1);
    let origin1 = PointLocations::Origin1;

    println!("origin: {:?}", origin);


}