fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);

    assert_eq!(2, v.len());

    let mut v2 = vec![3,4,5];

    v.append(&mut v2);

    assert_eq!(5, v.len()); 

    println!("{:?}",v);

    println!("second element is: {}", &v[1]);

    assert_eq!(Some(&1), v.get(0));

    println!("first element is: {}", v.get(7).unwrap_or_else(|| &-1));
    
    //print this array
    for i in &v {
        println!("ite: {}", i);
    }

}
