// float numbers
// fn main() {
//     let x = 2.0; // default is f64
//     let y: f32 = 3.0; // f32
// }

// numeric operations
// fn main() {
//     //addition
//     let sum = 5 + 10;
//
//     //subtraction
//     let diff = 95.5 - 4.3;
//
//     //multiplication
//     let mul = 4 * 30;
//
//     //division
//     let quotient = 5.0 / 3.0;
//     let truncated = -5 / 3;
//
//     //remainder
//     let remain = 43 % 5;
//     println!("sum = {sum}");
//     println!("diff = {diff}");
//     println!("mul = {mul}");
//     println!("quotient = {quotient}");
//     println!("truncated = {truncated}");
//     println!("reaminder = {remain}");
// }

// boolean type
// fn main() {
//     let t = true;
//     let f: bool = false;
// }

// the character type
// fn main(){
//     let c = 'z';
//     let z: char = 'ℤ';
//     let heart_eyed_cat = '😻';
//     println!("c = {c}, \
//     z = {z},\
//     emoji = {heart_eyed_cat}");
// }

// tuple
// fn main(){
//     let x = (500, 6.4, 1);
//     // indexing
//     let five_hundred = x.0;
//     let six_point_four = x.1;
//     let one = x.2;
// }

// array
// fn main(){
//     // giving type to array
//     let a: [i32; 5] = [1, 2, 3, 4, 5];
//     // repeat element
//     let repeated = [3; 5];
//     println!("{:?},", repeated);
//     // indexing
//     let first = repeated[0];
// }

// Invalid Array Element Access
use std::io;
fn main(){
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index. ");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Entered index is not a number");

    let element = a[index];
    println!("The value of {index}th element is {element}");
}