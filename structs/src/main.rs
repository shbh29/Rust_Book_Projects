#[derive(Debug)]
struct User {
    name: String,
    email: String,
    active: bool,
    age: u32,
}

fn main() {
    let user = User {
        name: String::from("Shubham"),
        email: String::from("shbh29@gmail.com"),
        active: true,
        age: 28,
    };


    println!("Name of User is {}!", user.name);
    println!("email of User is {}!", user.email);
    println!("Age of User is {}!", user.age);
    println!("Status of User is {}!", user.active);

    // Copy the user 1 to user to changing the email
    let user2 = User {
        email: String::from("mst.shubhamsingh@gmail.com"),
        ..user
    };

    println!("email of User 2 is {}!", user2.email);

    // able to access user after copying values to user2?
    // println!("Name of User is {}!", user.name);
    // value is not accessible
    //

    // But, value should be accessible is all the Heap memory members are overwritten.
    let user3 = User {
        name: String::from("John"),
        email: String::from("joh.doe@email.com"),
        ..user2
    };


    println!("email of User 3 is {}!", user3.email);
    println!("email of User 2 is {}!", user2.email);

    // Printing the user variables
    // #[derive(Debug)]
    // {:?}

    println!("User 2 contents are: {:#?}", user2);
    println!("User 3  contents are: {:?}", user3);

}
