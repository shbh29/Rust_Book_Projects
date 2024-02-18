fn main() {
    let immutable_var = String::from("immutable var");
    let mut mutable_var = String::from("mutable var");
    let immutable_ref : &String = &String::from("immutable reference");
    let mut mutable_ref : &mut String = &mut String:: from("mutable ref");
    
    //----------------------------- transfer to immutable var in function -----
    //immutable var in heap ownership transfer to immutable var
    //get_immutable(immutable_var);// As above declaration is of String type and not str
                                 // type(primitive type)(Stack memory) it can transfer its
                                 // ownership to another function.
                                 // str type could not transfer its ownership to another function


    //mutable var ownership transfer to immutable var
    //get_immutable(mutable_var);  // mutable var transfer of ownership worked.

    // immutable ref ownership transfer to immutable var
    //get_immutable(immutable_ref.to_string());

    // mutable ref ownership transfer to immutable var
    //get_immutable(mutable_ref.to_string());
    //------------------------------------------------------
    

    //---------------------Transfer Ownership to mutable String type-----
    //get_mutable(immutable_var); //the function gets an object with a immutable reference. But when
                                // passed to function that intends to mutate the object is able to
                                // mutate the object.
                                //
    //println!("Still cannot access immutable_var: {}", immutable_var);

    // mutable to mutable
    //get_mutable(mutable_var); // ownership transferred and cannot access this further in the code
                              // block
    //println!("Still can access mutable_var: {}", mutable_var);

    // imm ref to mutable
    //get_mutable(immutable_ref.to_string());
    //println!("Still can access immutable_ref: {}", immutable_ref);

    // mut_ref to mutable
    //get_mutable(mutable_ref.to_string());
    //println!("Still can access mutable_ref: {}", mutable_ref);
    //------------------------------------------------------

    //
    //---------------------Transfer Ownership to immutabl Ref type-----
    //transfer_to_immutable_ref(&immutable_var);
    //transfer_to_immutable_ref(&mutable_var);
    //transfer_to_immutable_ref(immutable_ref);
    //transfer_to_immutable_ref(mutable_ref);
    //
    //------------------------------------------------------
    //


    //--------------------Transfer Ownership to mutable reference type---
    //transfer_to_mutable_ref(&mut immutable_var);
    //not legal to pass

    transfer_to_mutable_ref(&mut mutable_var);// legal
    
    // Not legal to pass
    //transfer_to_mutable_ref(&mut immutable_ref);
    
   // legal to pass with &mut 
    transfer_to_mutable_ref(&mut mutable_ref);


    

    //println!("immutable var: {immutable_var}");
    //println!("mutable var: {mutable_var}");
    //println!("immutable ref: {immutable_ref}");
    //println!("mutable ref: {mutable_ref}");
}

fn get_immutable(s : String) {
    println!("immutable var: {s}");
}

fn get_mutable(mut s : String) {
    s.push_str("..!!");
    println!("mutated string: {s}");
}

fn transfer_to_immutable_ref(s : &String) {
    println!("immutable ref access: {}", s);
}

fn transfer_to_mutable_ref(s: &mut String) {
    s.push_str("...!!");
    println!("mutated by reference: {}",s);
}

