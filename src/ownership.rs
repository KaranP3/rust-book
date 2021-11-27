pub fn run() {
    // ownership rules
    // 1. Each value in Rust has a variable that's called its owner
    // 2. There can only be one owner at a time
    // 3. When the owner goes out of scope, the value will be dropped

    let x = 5;
    let y = x; // Copy - this implements the Copy trait.

    println!("{}", x);
    println!("{}", y);

    let s1 = String::from("Hello");
    let s2 = s1; // move (not shallow copy)
    let s3 = s2.clone(); // this is a copy

    println!("{}", s2);
    println!("{}", s3);

    // ownership and functions
    let greeting = String::from("hello");
    takes_ownership(greeting);
    // println!("{}", greeting); // borrow of a moved value

    let x = 10;
    makes_copy(x);
    println!("{}", x); // we can still use x after the funciton call

    let s1 = gives_ownership();
    println!("{}", s1); // the value from s1 has now been moved here
    let s2 = takes_and_gives_back(s1);
    println!("{}", s2);
    // println!("{}", s1); // this gives us a compiler error

    // references & borrowing
    let some_string = String::from("hello");
    let size = calculate_length(&some_string);
    println!("the length of {} is {}", some_string, size);

    let mut a_hello = String::from("Hello");
    change(&mut a_hello);

    // you can only have one mutable reference to a piece of data
    // in a particular scope
    // you can have multiple immutable references
    let r1 = &mut a_hello;
    println!("{}", r1);
    // let r2 = &mut a_hello; // this will cause a compiler error
    // println!("{}, {}", r1, r2);

    // dangle
    // let ref_to_nothing = dangle(); // compiler error in rust

    // The rules of references
    // 1. At any given time, you can have either one mutable reference
    // or  any number of immutable references
    // 2. References must always be valid

    // slices
    let another_string = String::from("Hello world");

    // let hello = &another_string[0..5];
    // let world = &another_string[6..];
    let hello = first_word(&another_string);
    // another_string.clear(); // this fails
    println!("{}", another_string);
    println!("{}", hello);

    // slices work with other types too
    let my_arr = [1, 2, 3, 4, 5];
    let my_arr_slice = &my_arr[..3];
    println!("{:?}", my_arr_slice);
}

fn first_word(s: &str) -> &str {
    // &str in the param type lets us take &str and String both
    let s_bytes = s.as_bytes();

    for (i, &item) in s_bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn change(s: &mut String) {
    s.push_str(" World");
}

fn calculate_length(str: &String) -> usize {
    str.len()
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(num: i32) {
    println!("{}", num);
}

fn gives_ownership() -> String {
    let some_string = String::from("from gives ownership");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
