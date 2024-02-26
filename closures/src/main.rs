use std::{thread, time::Duration};


fn main() {
    // creating an expensive macro
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    println!("calculated number: {}", expensive_closure(2));

    // compile time function declaration
    let return_same = |x| x;
    let my = return_same("My Name is Singh");
    println!("printed {}", my);

    


}