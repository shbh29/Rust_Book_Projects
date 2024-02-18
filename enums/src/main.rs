
#[derive(Debug)]
enum IpAddrKind {
    V4(u8,u8,u8,u8),
    V6(String),
}

enum Message {
    Quit,
    Move{x: u8, y:u8 },
    Write(String),
    ChangeColor(u32,u32,u32),
}


fn main() {
   let home = IpAddrKind::V4(127,0,0,0);

   let localhost  = IpAddrKind::V6(String::from("::1"));

   dbg!(&home);
   dbg!(&localhost);

   let message: Message = Message::Move { x: 45, y: 30 };

   match message {
       Message::Quit => println!("quit"),
       Message::Move{x,y} => println!("x: {}, y: {}",x,y),
       _ => (),
   };


}
