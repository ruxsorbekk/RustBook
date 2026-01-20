// enum IpAddr {
//     V4(String),
//     V6(String),
// }
// enum IpAddrKind {
//     V4,
//     V6,
// }
// fn main() {
//     // let home = IpAddr {
//     //     kind: IpAddrKind::V4,
//     //     address: String::from("127.0.0.1"),
//     // };
//     // let loopback = IpAddr {
//     //     kind: IpAddrKind::V6,
//     //     address: String::from("::1"),
//     // };
//     let home = IpAddr::V4(String::from("127.0.0.1"));
//     let loopback = IpAddr::V6(String::from("::1"));
// }

// enum Message {
//     Quit,
//     Move {x: i32, y: i32},
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// struct QuitMessage; // unit struct
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // also tuple struct
//
// impl Message {
//     fn call(&self) {
//
//     }
//
// }
// fn main() {
//     let m = Message::Write(String::from("hello"));
//     m.call();
// }

// option enum

enum Option<T> {
    Some(T),
    None,
}
fn main() {
    let _some_number = Some(3);
    let _some_char = Some('a');

    let _absent_number: Option<i32> = Option::None;

}