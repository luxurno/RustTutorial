fn main() {
    let four = IpAddr::V4;
    let six = IpAddr::V6;


    let home = IpAddr::V4(127, 0, 0, 1);
    
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");

    // Something's wrong here
    //let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
}

enum Option<T> {
    Some(T),
    None,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        // TODO
    }
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); //tuple struct


enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct Ipv4Addr {
    kind: IpAddr,
    address: String,
}

struct Ipv6Addr {
    kind: IpAddr,
    address: String,
}
