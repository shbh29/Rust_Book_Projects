fn main() {
    let mut counter = 0;
    'label1:loop {
        let mut y = 10;
        loop {
            println!("remaining: {y}");
            if y == 9 { break; }
            if counter > 5 { break 'label1; }
            y -= 1;
        }
        println!("Hello, world!");
        counter += 1;
    }
}
