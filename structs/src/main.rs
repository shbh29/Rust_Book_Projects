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
}
