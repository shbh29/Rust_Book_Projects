
#[derive(Debug)]
enum IpAddrKind {
    V4(u8,u8,u8,u8),
    V6(String),
}

fn main() {
   let home = IpAddrKind::V4(127,0,0,0);

   let localhost  = IpAddrKind::V6(String::from("::1"));

   dbg!(&home);
   dbg!(&localhost);
}
