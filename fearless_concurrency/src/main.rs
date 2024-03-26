use std::{sync::mpsc, thread};


fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msg = "hi";
        tx.send(msg).unwrap();
        println!("Hello from spawned thread: message sent: {}", msg);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}