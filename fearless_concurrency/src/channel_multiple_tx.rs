use std::{sync::mpsc, thread, time::Duration};


pub fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    thread::spawn(move || {
        let v = vec![
            String::from("hi"),
            String::from("hello"),
            String::from("Namaste"),
            String::from("Ram Ram")
        ];
        
        for m in v {
            tx.send(m).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        
    });


    thread::spawn(move || {
        let v = vec![
            String::from("More"),
            String::from("Messages"),
            String::from("for"),
            String::from("You!")
        ];
        
        for m in v {
            tx1.send(m).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        
    });

    for r in rx {
        println!("Got: {}", r);
    }
    
}