fn main() {
    let cond = 2 < 3;
    println!("{cond}");
    let cond = (2 as f32) < 2.2;
    println!("{cond}");
    let cond = false && cond || !false;
    println!("{cond}");

    let food = "bread";

    if food == "cookie" {
        println!("I like cookies!");
    } else if food == "fruit" {
        println!("You are healthy!");
    } else if food == "bread" {
        println!("How plain jain!");
    } else {
        println!("Oh! That is too bad!")
    }
}
