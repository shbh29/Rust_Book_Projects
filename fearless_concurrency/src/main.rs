use std::thread;

fn main() {

    let v = vec![1,2,3];
    
    let handle = thread::spawn(move || {
        println!("vector v: {:?}", v);    
    });

    println!("hello from main");

    handle.join().unwrap();

}
