// struct that takes references

pub struct ImportantExcepts<'a>{
    imps : &'a str
}

impl<'a> ImportantExcepts<'a> {
    fn level(&self) -> i32  {
        3
    }
}


pub fn main() {
    // lifetimes::simple_lifetime::main();

    let s = String::from("Why do you fear? When I am here. Do you love me dear?");

    let first = s.split('?').next().expect("no question marks found. ");

    let st = ImportantExcepts {
        imps: first
    };

    print!("Important Excepts: {}\n {}\n", st.imps, st.level());
}

