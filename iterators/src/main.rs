
fn main() {
    let v = vec![1,2,3, 4,5];

    let v_iter = v.iter();

    // print even numbers using filter
    let even_numbers : Vec<_>= v_iter.filter(|e| *e % 2 == 0).collect();
    println!("even numbers: {:?}, original vector: {:?}",even_numbers, v);


    let v_iter = v.iter();
    
    // map into multiple of 10
    let multply10x : Vec<i32> = v_iter.map(|x| x * 10).collect();
    println!("10 numbers: {:?}, original numbers: {:?}", multply10x, v);


}
