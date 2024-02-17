fn main() {
    let integer : u32 = 64;

    println! ("integer value is : {integer}");

    let integer : i32 = 28;

    println! ("integer value is i32 : {integer}");

    let my_float : f64 = 28.5;

    println!("my float is : {my_float}");

    let mychar: char = 'x';

    println!("mychar is : {}", mychar);

    let mybool: bool = true;

    println!("mybool value is : {mybool}");

    let x: (u32, f64, u8) = (90, 51.4, 13);

    let my_tuple_int = x.0;

    println!("my tuple values: {my_tuple_int}");

    //let (a, b, c) = x;

    println! ("my tuple float is : {}", x.1);

    
    
}
