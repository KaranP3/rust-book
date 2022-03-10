use std::collections::HashMap;
use std::hash::Hash;

struct Cacher<T, U, V>
where
    T: Fn(&U) -> V,
    U: Hash + Eq + Copy,
    V: Copy,
{
    calculation: T,
    results: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
where
    T: Fn(&U) -> V,
    U: Hash + Eq + Copy,
    V: Copy,
{
    fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            results: HashMap::new(),
        }
    }

    fn value(&mut self, arg: &U) -> V {
        match self.results.get(arg) {
            Some(result) => *result,
            None => {
                let v = (self.calculation)(arg);
                self.results.insert(*arg, v);
                v
            }
        }
    }
}

fn factorial(n: u128) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        _ => factorial(n - 1) * n,
    }
}

pub fn run() {
    let calculation = |x: &u128| factorial(*x);
    let mut cacher = Cacher::new(calculation);

    let numbers: Vec<u128> = vec![1, 2, 3, 4, 5, 7, 8, 9, 10];
    let results: Vec<u128> = numbers.iter().map(|x| cacher.value(x)).collect();

    println!("numbers: {:?}", numbers);
    println!("results: {:?}", results);

    // move closure
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;

    let y = vec![1, 2, 3];

    // println!("{:?}", x); - this will not compile
    println!("{}", equal_to_x(y));
}
