use std::fmt::Debug;

pub fn run() {
    // unlike the built-in array and tuple types, the data these
    // point to are all stored on the heap.

    /*
      three collections discussed here:
        1. vector - allows you to store a variable numbers of values
           next to each other
        2. string - a collection of characters
        3. hashmap - allows you to associate a value with a particular
           key.
    */

    // -------------------------------------------------------------

    /*
        vectors
    */

    // let v: Vec<i32> = Vec::new();
    // we need the type annotation beceasue it's empty

    let mut v = vec![1, 2, 3, 4]; // with macros

    // adding elements
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("{:?}", v);
    // note: dropping a vector drops its elements

    // reading elements
    let third: &i32 = &v[2];
    println!("third: {}", third);

    match v.get(2) {
        Some(num) => println!("third is: {}", num),
        None => println!("no such element"),
    }

    // iterating
    let v2 = vec![100, 32, 27];
    for i in &v2 {
        println!("loop: {}", i);
    }

    let mut v3 = vec![100, 32, 27];
    for i in &mut v3 {
        *i += 100;
    }
    println!("after mutating: {:?}", v3);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("row: {:?}", row);

    /*
       strings
    */
    let s = String::new();

    // string slice(str) to String
    let s1 = "Hello, ".to_string();

    println!("s: {}, s1: {}", s, s1);

    // appending
    let mut s2 = String::from("foo");
    println!("s2 before: {}", s2);

    s2.push_str("bar");
    println!("s2 after push_Str: {:?}", s2);

    let s3 = s1 + &s2;
    println!("s3: {}", s3);

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");

    let tic_tac_toe = format!("{}-{}-{}", tic, tac, toe);
    println!("{}", tic_tac_toe);

    // iterating over strings
    let hello = String::from("hello");

    for s in hello.chars() {
        println!("chars: {}", s);
    }

    for s in hello.bytes() {
        println!("bytes: {}", s);
    }
}
