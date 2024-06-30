#[derive(Debug)]

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main(){
    let msg1 = Message::Move{x: 1, y:2 };
    let msg2 = Message::Write(String::from("hello, world"));
    println!("suc");

    println!("{:?}", msg1);
    println!("{:?}", msg2);
}