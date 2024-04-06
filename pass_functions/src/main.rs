
fn add_one(v: i32) -> i32 {
    v + 1
}

fn add_with_function(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn get_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn add_with_closure(f: Box<dyn Fn(i32) -> i32>, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let v = add_with_closure(get_closure(), 5);
    assert_eq!(12, v);
    let v = add_with_function(add_one, 5);
    assert_eq!(12, v);
}
