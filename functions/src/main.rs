fn main() {
    let x = calculate_x(55);

    println!("value of x is {x}")
}

fn calculate_x(init_value: i32) -> i32 {
    init_value + 1
}
