use std::collections::HashMap;


fn main() {
    let mut map: HashMap<String, i32> = HashMap::new();

    map.insert(String::from("mykey"), 1);
    map.insert(String::from("mykey2"), 2);

    //if key present don't insert
    map.entry(String::from("mykey")).or_insert(2);

    //if key present increment the value by one and put back
    if let Some(x) = map.get_mut(&String::from("mykey")) {
        *x += 1;
    }

    println!(" map contents: {:#?}", map);

}
