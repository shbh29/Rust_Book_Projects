#[derive(Debug)]
enum Node {
    Val(i32, Rc<Node>),
    Nil,
}

use Node::{Val, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Val(5, Rc::new(Val(3, Rc::new(Nil)))));
    println!("Rc count is: {}", Rc::strong_count(&a));
    let b = Rc::new(Val(2, Rc::clone(&a)));
    println!("Rc count is: {}", Rc::strong_count(&a));
    {
        let c = Rc::new(Val(1, Rc::clone(&a)));
        println!("Rc count is: {}", Rc::strong_count(&a));
        println!("{:?}", c);
    }
    println!("after c out of scope Rc count is: {}", Rc::strong_count(&a));
    
    println!("{:?}", b);

}


