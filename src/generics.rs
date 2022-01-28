struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &elem in list {
        if elem > largest {
            largest = elem;
        }
    }

    largest
}

pub fn run() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102.2, 34.5, 6000.80, 89.2, 54.567, 2.85, 43.9, 8.55];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let integer = Point { x: 1, y: 10 };
    let float = Point { x: 2.5, y: 3.6 };
    println!("integer - X: {}, Y: {}", integer.x, integer.y);
    println!("float - X: {}, Y: {}", float.x, float.y);

    println!("{}", integer.x());
}
