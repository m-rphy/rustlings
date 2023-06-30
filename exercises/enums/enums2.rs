// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.


#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Move { x: u32, y: u32},
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

// impl assigns something similar to a method on the enum Message
// in this case the method name would be `call` and has a borrowed 
// refrence to `self`
impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
