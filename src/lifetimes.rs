use std::fmt::Display;

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// This annotation means an instance of
// ImportantExcerpt can’t outlive the reference
// it holds in its part field.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("announcement: {}", announcement);
        self.part
    }
}

// Generic Type Parameters, Trait Bounds, and Lifetimes Together
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn run() {
    // let string1 = String::from("abcd");
    // let string2 = "xyz";

    // let result = longest(string1.as_str(), string2);
    // println!("{}", result);

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("result = {}", result);
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split(".").next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("important excerpt: {}", i.part);
    i.announce_and_return_part("This is an announcement");

    // static lifetime means it lives throughout the duration of the program
    let s: &'static str = "I have a static lifetime"; // all string literals have a static lifetime!
    println!("{}", s);

    let string3 = String::from("some long string");
    let string4 = String::from("abc");

    println!(
        "result = {}",
        longest_with_an_announcement(
            string3.as_str(),
            string4.as_str(),
            "This is another announcement",
        )
    );
}
