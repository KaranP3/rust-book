use std::fmt::Debug;

enum List<T>
where
    T: Debug,
{
    Cons(T, Box<List<T>>),
    Nil,
}

fn print_list<T>(list: &List<T>)
where
    T: Debug,
{
    match list {
        List::Cons(value, next) => {
            print!("{:?} -> ", value);
            print_list(next.as_ref());
        }
        List::Nil => print!("Nil\n"),
    }
}

pub fn run() {
    use List::{Cons, Nil};

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    print_list(&list);
}
