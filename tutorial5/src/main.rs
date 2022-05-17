use std::io;

fn main() {
    println!("Type something in and hit enter.");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    println!("You typed: {input}");
}
