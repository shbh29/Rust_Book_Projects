use std::{sync::{Arc, Mutex}, thread};



fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let m = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            println!(" mutex: {:?} ", m);
            let mut val = m.lock().unwrap();
            *val += 1;
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("mutex: {:?} ", counter.lock().unwrap());
}