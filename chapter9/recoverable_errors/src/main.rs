use std::fs::File;
use std::io::{read_to_string, ErrorKind};
use std::io::{self, Read};
use std::error::Error;

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// fn main() {
//     let greeting_file_result = File::open("hello.txt");
//
//     let greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Problem creating the file: {e:?}"),
//             },
//             _ => {
//                 panic!("Problem opening the file {error:?}");
//             }
//         },
//     };
// }

// alternative version
// fn main() {
//     let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
//         if error.kind() == ErrorKind::NotFound {
//             File::create("hello.txt").unwrap_or_else(|error | {
//                 panic!("problem creating the file: {error:?}");
//             } )
//         } else {
//             panic!("problem opening the file: {error:?}");
//         }
//     });
// }

// shortcuts for panic error
// fn main() {
//     // let greeting_file = File::open("hello.txt").unwrap();
//     let greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project");
// }
//
// // propagating errors
// fn read_username_from_file() -> Result<String, io::Error>{
//     let username_file_result = File::open("hello.txt");
//
//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//
//     let mut username = String::new();
//
//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }

// ? operator shortcut
// fn read_username_from_file() -> Result<String, io::Error>{
//     let mut username_file = File::open("hello.txt")?;
//     let mut username = String::new();
//     username_file.read_to_string(&mut username)?;
//     Ok(username) // that shortcut is legendary in my opinion
// }

// shorten version
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username = String::new();
//
//     File::open("hello.txt")?.read_to_string(&mut username)?;
//     Ok(username)
// }

// listing 9-12, i have no idea what it is but just wanna write
fn main() -> Result<(), Box<dyn Error>>{
    let greeting_file = File::open("hello.txt");

    Ok(())
}