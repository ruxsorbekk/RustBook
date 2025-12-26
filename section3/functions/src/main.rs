// functions
// fn main() {
//     println!("Hello, world!");
//
//     another_function();
// }
// fn another_function() {
//     println!("Another function");
// }

// parameters 1
// fn main() {
//     another_function(5);
// }
// fn another_function(x: i32){
//     println!("The value of x is {x}");
// }

// 2
// fn main() {
//     print_labeled_measurement(5, 'h');
// }
// fn print_labeled_measurement(value: i32, unit_label: char){
//     println!("The measurement is: {value}{unit_label}");
// }

// statements and expressions
// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };
//     println!("The value of y is {y}");
// }

// functions with return values 1
// fn five() -> i32 {
//     5
// }
// fn main() {
//     let x = five();
//     println!("The value of x is {x}");
// }

// 2
fn main() {
    let x = plus_one(5);
    println!("x = {x}");
}
fn plus_one(x: i32) -> i32 {
    x + 1
}
