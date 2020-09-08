
#[derive(Debug)]
enum Message {
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    Change(i32, i32, i32),
}

impl Message {
    fn print(&self) {
        match *self {
            Message::Quit => println!("Quit"),
            Message::Move{x, y} => println!("Move x = {}, y = {}", x, y),
            Message::Change(a, b, c) => println!("Change a = {}, b = {}, c = {}", a, b, c),
            _ => println!("Write"),
        }
    }
}

fn main() {

    let msg1 = Message::Quit;
    msg1.print();
    println!("msg1: {:#?}", msg1);

    let msg2 = Message::Move{
        x: 1,
        y: 2,
    };
    msg2.print();
    println!("msg2: {:#?}", msg2);

    let msg3 = Message::Change(1,2,3);
    msg3.print();


    println!("Hello, world!");
}
