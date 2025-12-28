// fn main() {
//     // when s comes into the scope, it is valid. it remains valid until it goes out of scope
//     let mut s = String::from("hello");
//     s.push_str(", world!");
//     println!("{s}")
// }

// variables and data interacting with move
// fn main() {
//     // integer
//     let x = 5;
//     let y = x;
//
//     //string
//     let s1 = String::from("hello");
//     let s2 = s1;
//     println!("{s1}, world!") // that doesn't work bc s1 moved to s2
// }

// scope assignment
// fn main() {
//     let mut s = String::from("hello"); // this will go out of scope and freed immediately
//     s = String::from("ahoy");
//     println!("{s}, world!")
// }

// stack only data: copy
// fn main() {
//     let x = 5;
//     let y = x;
//     println!("x = {x}, y = {y}"); // copying works in int, float, bool, char, and also tuple if they have the exact same type
// }

// ownership and functions
