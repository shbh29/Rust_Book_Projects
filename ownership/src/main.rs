fn main() {
    let immutable_var = String::from("immutable var");
    let mut mutable_var = String::from("mutable var");
    let immutable_ref : &str = &String::from("immutable reference");
    let mut mutable_ref : &mut str = &mut String:: from("mutable ref");

    //immutable var in heap ownership transfer to immutable var
    get_immutable(immutable_var);// As above declaration is of String type and not str
                                 // type(primitive type)(Stack memory) it can transfer its
                                 // ownership to another function.
                                 // str type could not transfer its ownership to another function


    //mutable var ownership transfer to immutable var
    get_immutable(mutable_var);  // mutable var transfer of ownership worked.

    // immutable ref ownership transfer to immutable var
    get_immutable(immutable_ref.to_string());

    // mutable ref ownership transfer to immutable var
    get_immutable(mutable_ref.to_string());

    //println!("immutable var: {immutable_var}");
    //println!("mutable var: {mutable_var}");
    //println!("immutable ref: {immutable_ref}");
    //println!("mutable ref: {mutable_ref}");
}

fn get_immutable(s : String) {
    println!("immutable var: {s}");
}

fn get_mutable(s: mut String) {
    s.push_str("..!!");
    println!("mutated string: {s}");
}

