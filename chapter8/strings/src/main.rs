fn main() {
    // let mut s = String::new();
    // let data = "initial contents";
    // let s = data.to_string();

    // let s = "initial contents".to_string(); // that also works

    // using String::from()
    // let s = String::from("initial contents");

    // updating string. append
    // let mut s = String::from("foo");
    // s.push_str("bar");

    // let mut s1 = String::from("foo");
    // let  s2 = "bar";
    // s1.push_str(s2);
    //
    // println!("s2 is {s2}");

    // concatenating with + or format!
    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2; // s1 has been moved here and can no longer be used
    // println!("s3 = {s3}");


    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");
    // let s = s1 + "-" + &s2 + "-" + &s3;
    // let s = format!("{s1}-{s2}-{s3}");
    // println!("{s}");

    // indexing
    // let s1 = String::from("hello");
    // let h = &s1[0]; // does not compile, reason is on the doc
    // println!("first letter is {h}");

    // slicing
    // let hello = "Здравствуйте";
    // let s = &hello[0..4]; // answer is "Зд" because each character takes 2 bytes of mem
    // println!("{s}");

    // iterating strings
    for c in "Зд".bytes(){
        println!("{c}");
    };
}