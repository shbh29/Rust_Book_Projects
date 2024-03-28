use std::sync::Mutex;


pub fn main() {
    let m = Mutex::new(5);

    {
        let mut value = m.lock().unwrap();
        *value = 6;
    }

    println!("value of mutex: {:?}", m);
}