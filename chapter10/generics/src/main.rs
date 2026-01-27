// using generics with functions

// fn main() {
//     let number_list: Vec<i32> = vec![1, 2, 3, 4, 5];
//     println!("largest number is {}", get_largest(number_list));
//     let char_list = vec!['a', 'b', 'z', 'j', 'm', 'y'];
//     println!("largest char is {}", get_largest(char_list));
// }
//
// fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
//     let mut largest = number_list[0];
//     for number in number_list {
//         if number > largest {
//             largest = number;
//         }
//     }
//     largest
// }

// with structs
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    // let p1 = Point {x: 5, y: 10};
    // let p2 = Point {x: 5.0, y: 10.0};
    // let p3 = Point {x: 5, y: 10.0};
    let p1 = Point {x: 5, y: 10.4};
    let p2 = Point {x: "hello", y: 'c'};
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// with enums
// fn main() {
//     enum Option<T> {
//         Some(T),
//         None,
//     }
//
//     enum Result<T, E> {
//         Ok(T),
//         Err(E),
//     }
//
// }

