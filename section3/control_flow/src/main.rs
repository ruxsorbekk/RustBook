// fn main() {
//     let number = 3;
//     if number != 0{
//         println!("number is not zero");
//     }
// }

// else if
// fn main() {
//     let number = 6;
//     if number % 4 == 0 {
//         println!("Number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("Number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("Number is divisible by 2");
//     } else {
//         println!("Number is not divisible by 4, 3 or 2!");
//     }
// }

// using if in a let statement
// fn main() {
//     let condition = true;
//     let number = if condition {5} else { 6 }; // variables must be single type, othwerwise we'll get an error
//     println!("the value of number is {number}");
// }


// loops
// fn main() {
//     loop {
//         println!("again")
//     }
// }

// returning values from loops
// fn main() {
//     let mut counter = 0;
//     let result = loop {
//         counter += 1;
//
//         if counter == 10 {
//             break counter * 2;
//         }
//     };
//     println!("result = {result}");
// }

// disambiguating with loop labels
// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;
//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }
//         count += 1;
//     }
//     println!("End count = {count}");
// }

// streamlining conditional loops with while
// fn main() {
//     let mut number = 3;
//     while number != 0{
//         println!("{number}");
//         number -= 1;
//     }
//     println!("LIFTOFF!");
// }

// looping through a collection with for
// fn main() {
//     let mut a = [10, 20, 30, 40, 50];
//     let mut index = 0;
//
//     while index < 5{
//         println!("the value is {}", a[index]);
//         index += 1;
//     }
//
// }

// shorter variation
// fn main() {
//     let a = [10, 20, 30, 40, 50];
//
//     for element in a{
//         println!("the value is {element}");
//     }
// }

// using reverse. rev()
fn main() {
    for number in (1..3).rev(){
        println!("{number}");
    }
    println!("LIFTOFF!");
}