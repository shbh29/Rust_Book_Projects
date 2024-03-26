pub mod simple_thread_run {
    use std::{thread, time::Duration};

    pub fn main() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hello from thread {}", i);
                thread::sleep(Duration::from_millis(1))
            }
        });

        for i in 1..5 {
            println!("Hello from main! {} ", i);
            thread::sleep(Duration::from_millis(1));
        }

        handle.join().unwrap();
    }
}

pub mod move_keyword {
    use std::thread;

    pub fn main() {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(move || {
            println!("vector v: {:?}", v);
        });

        println!("hello from main");

        handle.join().unwrap();
    }
}
