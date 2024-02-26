use std::thread;

#[derive(Debug)]
pub struct Rectangle {
    pub width: i32,
    pub height: i32,
}


fn main() {
    // move to thread
    // let list = vec![1, 2, 3];
    
    // thread::spawn(move || println!("list content from thread: {:?}", list))
    //     .join()
    //     .unwrap();


    //Rectangle sort by width
    let mut arr = [
        Rectangle{width: 13, height: 7},
        Rectangle{width: 3, height: 6},
        Rectangle{width: 7, height: 10},
    ];

    println!("array elements: {:#?}", arr);

    arr.sort_by_key(|e| -e.width);


    println!("array elements: {:#?}", arr);

}
