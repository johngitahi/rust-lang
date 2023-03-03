fn main() {
    let x = plus_ten(5);
    println!("x value is {x}");
}

fn plus_ten(x: i32) -> i32 {
    x + 10
}
