trait Summary {
    fn summarize(&self) -> String {
        String::from("Read more...")
    }
}

#[allow(dead_code)]
struct NewsArticle {
    headline: String,
    author: String,
    location: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

#[allow(dead_code)]
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    rewteet: bool,
}

impl Summary for Tweet {}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

// with trait bounds
// fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize())
// }

// using impl Trait is useful when we want items to have the same type
// if we want to force the parameters to have the same type we need to
// express this using a trait bound
// fn notifyMultiple<T: Summary>(item1: &T, item2: &T) {
//     println!("Breaking news! {}", item1.summarize());
//     println!("Breaking news! {}", item2.summarize())
// }

// we can also specify multiple trait bounds with the + syntax
// fn notify(item: &(impl Summary + Display)) {}
// fn notify<T: Summary + Display>(item: &T) {}

// trait bounds with where clauses
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
//     1
// }

// we can also return types that implement traits
// but this can only be used if you're returning a single type
// you cannot return either Tweet or NewsArticle for example
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("some_user"),
        content: String::from("My first tweet"),
        reply: false,
        rewteet: false,
    }
}

// using trait bounds to conditionally implement methods
// struct Pair<T> {
//     x: T,
//     y: T,
// }

// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self { x, y }
//     }
// }

// impl<T: Display + PartialOrd> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x > self.y {
//             println!("the largest member is: {}", self.x)
//         } else {
//             println!("the largest member is: {}", self.y)
//         }
//     }
// }

pub fn run() {
    let tweet = returns_summarizable();

    let article = NewsArticle {
        headline: String::from("A news headline!"),
        author: String::from("John Doe"),
        location: String::from("New Delhi, India"),
        content: String::from("..."),
    };

    println!("new tweet: {}", tweet.summarize());
    println!("new article: {}", article.summarize());
    notify(&article);
    notify(&tweet);
}
