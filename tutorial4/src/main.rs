fn main() {
    basic();
    tups();
    arrs();
}

fn arrs() {
    let mut arr: [i32; 4] = [1, 2, 4, 5];
    arr[2] = 22;
    println!("arr is: {}", arr[2]);
}

fn tups() {
    let mut tup: (i32, bool, char) = (1, true, 's');
    tup.0 = 10;
    println!("tup is: {} {}", tup.0, tup.1);
    tup = (12, false, 'b');
    println!("tup is: {} {}", tup.0, tup.1);
}

fn basic() {
    let x: i32 = 2;
    println!("x is: {x}");
    let f: f32 = 1.23;
    println!("f is: {f}");
    let bool_val: bool = false;
    println!("bool_val is: {bool_val}");
    let letter: char = 'a';
    println!("letter is: {letter}");
}
