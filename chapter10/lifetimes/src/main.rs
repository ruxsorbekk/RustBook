// fn main() {
//     let x = 5;
//     let r = &x;
//     println!("r: {r}");
// }


// generic lifetime annotations
// fn main() {
//     let string1 = String::from("abkadsjh");
//     let string2 = String::from("yokoso watashi no soul society");
//
//     let result = longest(string1.as_str(), string2.as_str());
//     println!("The longest string is '{result}'");
// }
//
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else { y }
// }

// struct
struct ImportantExcerpt<'a>{
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me. some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    
}