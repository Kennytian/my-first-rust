//  rustc --out-dir target/debug ./src/nine_nine.rs
// ./target/debug/nine_nine 2

use std::env;

fn add() {
    let args: Vec<String> = env::args().skip(1).collect();

    let num1: f32 = args
        .get(0)
        .map(|arg| arg.parse().expect("Not a number"))
        .expect("Please input number 1");

    let num2 = args
        .get(1)
        .map(|arg| arg.parse::<f32>().expect("Not a number"))
        .expect("Please input number 2");

    println!("{} + {} = {}", num1, num2, num1 + num2);
}

fn main() {
    add();
}
