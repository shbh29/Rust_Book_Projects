fn main() {
    let immutable_var = String::from("immutable var");
    //let mut mutable_var = String::from("mutable var");
    //let immutable_ref : &str = &String::from("immutable reference");
    //let mut mutable_ref : &mut str = &mut String:: from("mutable ref");

    get_immutable(immutable_var);// As above declaration is of String type and not str
                                 // type(primitive type)(Stack memory) it can transfer its
                                 // ownership to another function.
                                 // str type could not transfer its ownership to another function



    //println!("immutable var: {immutable_var}");
    //println!("mutable var: {mutable_var}");
    //println!("immutable ref: {immutable_ref}");
    //println!("mutable ref: {mutable_ref}");
}

fn get_immutable(s : String) {
    println!("immutable var: {s}");
}

