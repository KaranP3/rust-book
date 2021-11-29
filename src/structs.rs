#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple structs without named fields
#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// note: you can have multiple impl blocks for a struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // does not use the instance
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

pub fn run() {
    let user = User {
        active: true,
        username: String::from("newuser"),
        email: String::from("newuser@email.com"),
        sign_in_count: 1,
    };

    println!("{:?}", user);

    // to mutate values in a struct
    // you need to make mutable and use .
    // let mut user2 = User {
    //     active: true,
    //     username: String::from("newuser"),
    //     email: String::from("newuser@email.com"),
    //     sign_in_count: 1,
    // };

    // user2.email = String::from("somethingelse@email.com");
    // note that only certain fields cannot be made mutable
    // it is either the whole thing or none

    // using the build_user function
    let new_user = build_user(
        String::from("something@something.com"),
        String::from("someuser"),
    );
    println!("{:?}", new_user);

    // struct update syntax
    let updated_user = User {
        email: String::from("anotheremail@email.com"),
        ..user // this moves the values of user, so we can no longer use user
    };

    println!("{:?}", updated_user);
    // println!("{:?}", user); // this has now been partially moved and won't work

    // tuple structs without named fields
    let color = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{:#?}", color);
    println!("{:#?}", origin);

    // example program using structs
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    if rect.width() {
        println!(
            "the width of the rectangle is greater than zero, it is {}",
            rect.width
        );
    }

    let rect2 = Rectangle {
        width: 25,
        height: 30,
    };

    if rect.can_hold(&rect2) {
        println!(
            "Rectangle({}, {}) can hold Rectangle({}, {})",
            rect.width, rect.height, rect2.width, rect2.height
        );
    }

    println!("area = {}", rect.area());
    dbg!(&rect);

    // associated function, not a method, does not use rectangle instance
    let square = Rectangle::square(20);
    println!("{:?}", square);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
