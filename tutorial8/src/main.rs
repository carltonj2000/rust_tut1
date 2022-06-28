fn main() {
    test();
    let a = add_numbers(3, 345) + add_numbers(9, 10);
    println!("added numbers = {}!", a);

    let number = {
        let x = 3;
        x + 1
    };

    println!("number {}", number);

    let c = 34;
    let d = 56;
    println!("{} + {} = {}", c, d, add_number2(c, d));
    println!("{} + {} = {}", 2, 3, add_number2(2, 3));
}

fn add_number2(x: i32, y: i32) -> i32 {
    let result = x + y;
    if result > 10 {
        result - 10
    } else {
        result
    }
}

fn add_numbers(x: i32, y: i32) -> i32 {
    println!("The sum is {}", x + y);
    x + y
    // return x + y; // optional
}

fn test() {
    println!("Test has been called ...")
}
