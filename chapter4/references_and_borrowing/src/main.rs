// reference and borrowing
// fn main() {
//     let s1 = String::from("hello");
//     let len = calculate_length(&s1);
//     println!("The length of '{s1}' is {len}");
// }
// fn calculate_length(s: &String) -> usize{
//     s.len()
// }

// borrowing
// fn main() {
//     let s = String::from("hello");
//     change(s);
// }
// fn change(some_string: &String) {
//     some_string.push_str(", rust");
// }

// mutable references
// fn main() {
//     let mut s = String::from("hello");
//     change(&mut s);
// }
// fn change(s: &mut String) {
//     s.push_str(", world!");
// }

// fn main() {
//     let mut s = String::from("hello");
//     let r1 = &s;
//     let r2 = &s;
//     println!("r1 = {r1}, r2 = {r2}");
//
//     let r3 = &mut s;
//     println!("{r3}");
// }

// dangle references
fn main() {
    let dangle_reference = dangle();
}
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// } // that doesn't work bc, string will be dropped after scope . after string can't be borrowed
fn dangle() -> String{
    let s = String::from("hello");
    s
}