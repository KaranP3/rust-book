use std::fs;
use std::io::{self};

pub fn run() {
    /*
       unrecoverable errors
    */

    // function that panics
    // unrecoverable();

    // panic from a library
    // let v = vec![1, 2, 3];

    // println!("{}", v[99])

    // recoverable errors

    // // basic
    // let f = match File::open("hello.txt") {
    //     Ok(f) => f,
    //     Err(e) => panic!("Problem opening the file: {}", e),
    // };

    // matching on error types
    // let f = match File::open("hello.txt") {
    //     Ok(f) => f,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {}", e),
    //         },
    //         other_error => panic!("Problem opening the file: {:?}", other_error),
    //     },
    // };

    // a better way
    // let f = File::open("hello.txt").unwrap_or_else(|err| {
    //     if err.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|err| {
    //             panic!("Problem creating the file: {}", err);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {}", err)
    //     }
    // });

    // shortcut with unwrap
    // unwrap calls the panic!() macro for us
    // let f = File::open("hello.txt").unwrap();

    // shortcut with expect
    // expect also calls the panic! macro for us
    // using expect with good error messages is a best practice
    // let f = File::create("hello.txt").expect("Failed to open hello.txt");

    // propagating errors
    let username = match read_username_from_file() {
        Ok(u) => u,
        Err(e) => panic!("Problem reading username: {}", e),
    };

    println!("{}", username);
}

// more concise
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut s = String::new();
//     File::open("hello.txt")?.read_to_string(&mut s)?;
//     Ok(s)
// }

// even more concise
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");

//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();

//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// fn unrecoverable() {
//     panic!("crash and burn")
// }
