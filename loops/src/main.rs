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


