
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
   let home = IpAddrKind::V4;

   let localhost  = IpAddrKind::V6;

   dbg!(&home);
   dbg!(&localhost);
}
