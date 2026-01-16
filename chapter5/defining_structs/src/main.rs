// struct User{
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
// fn main() {
//     let mut user1 = User{
//         active: true,
//         username: String::from("username"),
//         email: String::from("email@example.com"),
//         sign_in_count: 1,
//     };
//     let  user2 = User{
//         email: String::from("another@example.com"),
//         ..user1
//     };
//
// }
// fn build_user(email: String, username: String) -> User {
//     User{
//         active: true,
//         username,
//         email,
//         sign_in_count: 1,
//     }
// }

// tuple structs
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);
//
// fn main() {
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
//     // they are not the same type, even they have the same type of fields
// }

// unit like structs
struct AlwaysEqueal;
fn main() {
    let subject = AlwaysEqueal;
}