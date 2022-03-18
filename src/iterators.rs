struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

pub fn run() {
    let v1 = vec![1, 2, 3, 4];

    let mut v1_iter = v1.iter();

    println!("{:?}", v1_iter.next());
    println!("{:?}", v1_iter.next());

    let counter = Counter::new();

    for elem in counter.into_iter() {
        println!("{}", elem);
    }

    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    println!("{}", sum);

    // methods that call next are called consuming adapters
    // because calling them uses up the iterator.

    // methods defined on the Iterator trait, known as iterator
    // adapters, allow you to change iterators into different
    // kinds of iterators

    // because all iterators are lazy, you have to call one of the
    // consuming adaptor methods to get results from calls to
    // iterator adaptors

    // the collect method consumes the iterator and collects the
    // resulting values into a collection data type.
}
