fn return_values_with_loop() {
   let arr = [5,3,6,6,7,8,44,5,5,5];
    let mut count = 0;
   let count_len = loop {
        count += 1;
        if count >= arr.len() {
            break count;
        }
   };
   println!(" count of array is: {count_len}");
}

fn while_loop() {
    let mut counter = 0;
    while counter < 5 {
        println!("counter value is: {counter}");
        counter += 1;
    }

}

fn main() {
    // print array of elements
    let arr = [1,2,3,4,5,6,7,8];

    for e in arr {
        println!(" array element: {e}");
    }

    // print elements using index values
    println!("printing using array elements");
    for i in (0..8) {// 8 is exclusive. i.e. range will be 0 to 7
        println!(" array element: {}", arr[i]);
    }

}
