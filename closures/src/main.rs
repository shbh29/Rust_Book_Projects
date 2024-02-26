use std::thread;

#[derive(Debug)]
pub struct Rectangle {
    pub width: i32,
    pub height: i32,
}


fn main() {
    // move to thread
    let list = vec![1, 2, 3];
    
    thread::spawn(move || println!("list content from thread: {:?}", list))
        .join()
        .unwrap();


    //Rectangle sort by width
    let mut arr = [
        Rectangle{width: 13, height: 7},
        Rectangle{width: 3, height: 6},
        Rectangle{width: 7, height: 10},
    ];

    // let mut sort_operations = vec![];
    // let value = String::from("by key called");
    let mut num_sort_operations = 0;

    println!("array elements: {:#?}", arr);

    arr.sort_by_key(|e| {
        // sort_operations.push(value);
        num_sort_operations += 1;
        -e.width
    });

    println!();
    println!("{:#?}, sorted in {num_sort_operations} operations", arr);

    // println!("array elements: {:#?}", arr);

}
