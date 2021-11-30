// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// #[derive(Debug)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// impl Message {
//     fn call(&self) {
//         println!("{:?}", self)
//     }
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

pub fn run() {
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", loopback);

    // let m = Message::Write(String::from("Hello world"));
    // m.call();

    // Option enum
    let some_number = Some(5);
    let some_string = Some("A string");
    let absent_number: Option<i32> = None;

    println!("{:?}", some_number);
    println!("{:?}", some_string);
    println!("{:?}", absent_number);

    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);

    // let sum = x + y; // this will not compile

    // match control flow operator

    // let coin = Coin::Penny;
    // println!("penny in cents: {}", value_in_cents(coin));

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = match y {
        Some(i) => i + x,
        None => -1,
    };

    println!("sum is {}", sum);

    // catch-all patterns
    let something = 10;
    let result = match something {
        10 => "You win",
        12 => "Try again",
        _ => "You lose",
    };

    println!("{}", result);

    if let Some(y_val) = y {
        println!("the sum is {}", x + y_val);
    }
}

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }
