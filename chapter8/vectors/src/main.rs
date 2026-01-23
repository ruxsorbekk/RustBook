fn main() {
    // let v: Vec<i32> = Vec::new(); // empty vector
    // let v = vec![1, 2, 3]; // creating a new vector containing values

    // updating vector
    // let mut v = Vec::new();
    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);

    // reading elements of vectors
    // let v = vec![1, 2, 3, 4, 5];
    // let third: &i32 = &v[2];
    // println!("The 3rd element is {}", third);
    //
    // let third: Option<&i32> = v.get(2);
    // match third {
    //     Some(third) => println!("The third element is {third}"),
    //     None => println!("There is no third element!"),
    // }

    // trying to get 100th element in a vector containing 5 elements
    // let v = vec![1, 2, 3, 4, 5];
    // let doesnt_exist = &v[100];
    // println!("{doesnt_exist}");

    // let mut v = vec![1, 2, 3, 4, 5];
    // v.push(6);
    // let first = &v[0];
    // println!("{first}");

    // iterating values
    // let mut v = vec![100, 2, 32];
    // for i in &mut v {
    //     *i += 50;
    //     println!("{i}");
    // }


    // using enum to store multiple times
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("hello")),
        SpreadsheetCell::Float(10.12),
    ];
    for i in &row {
        println!("{:?}", i);
    }
}
