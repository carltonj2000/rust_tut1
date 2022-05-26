use std::io;

fn main() {
    let x = 127_000i64;
    let y = 10_i64;
    let z = x / y;
    println!("z={z}");

    let x = 127_000 as i64;
    let y = 127_i32;
    let z = x / (y as i64);
    println!("z={z}");

    let x = i32::max as i64 + 1;
    let y = 10_i32;
    let z = x as i32 / y;
    println!("z={z}");

    let mut input = String::new();
    println!("type and then hit enter");
    io::stdin()
        .read_line(&mut input)
        .expect("expected to read line");
    println!("read input => {input}");

    let mut input = String::new();
    println!("enter number and then hit enter");
    io::stdin()
        .read_line(&mut input)
        .expect("expected to read line");
    let int_input: i64 = input.trim().parse().unwrap();
    println!("read input + 2 => {}", int_input + 2);
}
