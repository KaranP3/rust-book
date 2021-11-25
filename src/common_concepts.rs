fn sum(x: i32, y: i32) -> i32 {
    return x + y;
}

pub fn run() {
    let result = sum(10, 2);
    println!("{}", result);

    let numbers = [1, 2, 3, 4];

    for num in numbers {
        println!("{}", num);
    }

    let mut counter = 0;

    while counter < 10 {
        counter += 1;
    }
    println!("{}", counter);

    let count = loop {
        counter += 1;

        if counter == 20 {
            break counter;
        }
    };
    println!("{}", count);

    let condition = false;
    let something_else = if condition { 10 } else { 2 };
    println!("{}", something_else);
}
