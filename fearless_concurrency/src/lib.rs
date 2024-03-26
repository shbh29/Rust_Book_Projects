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
