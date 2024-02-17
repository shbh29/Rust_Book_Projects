fn main() {
    let x = calculate_x(55, 76);

    println!("value of x is {x}");
    
    // Rust does not support function overloading, instead Traits are used.
    //let x = calculate_x(55);

    //println!("value of x is {x}");
}

fn calculate_x(init_value: i32, add_value: i32) -> i32 {
    init_value + add_value
}
//fn calculate_x(init_value: i32) -> i32 {
//    init_value + 1
//}
