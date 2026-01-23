use std::collections::HashMap;

fn main() {
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);
    // // let team_name = String::from("Blue");
    // // let score = scores.get(&team_name).copied().unwrap_or(0);
    //
    // for (key, value) in &scores {
    //     println!("{key}, {value}");
    // }

    // ownership in hashmap
    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");
    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);

    // updating hashmap
    // let mut scores = HashMap::new();
    // scores.insert(String::from("blue"), 10);
    // scores.insert(String::from("blue"), 25);
    // println!("{scores:?}")

    // adding key and value
    // let mut scores = HashMap::new();
    // scores.insert(String::from("blue"), 10);
    // scores.entry(String::from("yellow")).or_insert(50); // that add value if key doesn't exist. if key exist it just do nothing
    // scores.entry(String::from("blue")).or_insert(50);
    // println!("{scores:?}");

    // updating a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");
}
