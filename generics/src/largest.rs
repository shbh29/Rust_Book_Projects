
pub fn largest_main() {
    let v = vec![2,5,7,8,5,5,4];
    println!("largest number is : {}", largest(&v));
    
    let v = vec!['g','c','a','h','h','f','n',];
    println!("largest number is : {}", largest(&v));
}


fn largest<T: std::cmp::PartialOrd>(v : &[T]) -> &T {
    
    let mut largest = &v[0];
    
    for e in v {
        if e > largest {
            largest = e;
        }
    }
    
    largest
}