// example
// fn main() {
//     let width1 = 30;
//     let height1 = 50;
//
//     println!("The area of square is {}", area(width1, height1));
// }
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// refactoring with tupels
// fn main() {
//     let rect = (30, 50);
//     println!("The area of rectangle is {}", area(rect));
// }
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// with structs
// struct Rectangle{
//     width: u32,
//     height: u32
// }
// fn main() {
//     let rect = Rectangle{
//         width: 30,
//         height: 50
//     };
//     println!("The area of rectangle is {}", area(&rect));
// }
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// functionality
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    // let rect = Rectangle{
    //     width: 10,
    //     height: 40,
    // };
    // println!("rect is {rect:?}");

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 40,
    };
    dbg!(rect1);
}