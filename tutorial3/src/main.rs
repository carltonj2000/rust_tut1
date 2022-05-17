fn main() {
    let mut x = 4;
    println!("x is: {x}");
    x = 5;
    println!("x is: {x}");
    let y = 4;
    println!("y is: {y}");
    let y = 5;
    println!("y is: {y}");
    let y = 3;
    println!("y is: {y}");
    {
        let y = y - 10;
        println!("y is: {y}");
    }
    let y = y + 1;
    println!("y is: {y}");
    let y = "y = y + 1";
    println!("y is: {y}");

    const SECONDS_IN_MINUTE: u32 = 60;
    println!("SECONDS_IN_MINUTE is: {SECONDS_IN_MINUTE}");
}
