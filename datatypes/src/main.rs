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

    let my_arr = [1,2,3,4,5];

    println!("my array first element: {}",my_arr[0]);

    let my_arr = [5; 10];

    println!("my arr of 10 elements: {}", my_arr[5]);
    println!("my arr of 10 length: {}", my_arr.len());
    
}
