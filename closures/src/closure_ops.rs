use std::{thread, time::Duration};


pub fn main() {
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

    //list printing
    let list = vec![1,2,3];
    println!("list content: {:?}", list);
    let borrow_only = || println!("list contents from macro: {:?}", list);
    println!("list content: {:?}", list);
    borrow_only();
    println!("list content: {:?}", list);

    //list mutating
    let mut list = vec![1,2,3];
    println!("list content: {:?}", list);
    let mut borrow_mutate = || {
        list.push(7);
        println!("list contents from macro: {:?}", list);
    };
    // println!("list content: {:?}", list); // won't work
    borrow_mutate();
    println!("list content: {:?}", list);


}