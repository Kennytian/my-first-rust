//  rustc --out-dir target/debug ./src/calc.rs
// ./target/debug/calc 2

use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let num: i32 = match args.get(0) {
        Some(arg) => match arg.parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("错误：参数‘{}’无法解析为数字", arg);
                return;
            }
        },
        None => {
            eprintln!("错误：缺少参数");
            return;
        }
    };

    if num > 10 {
        println!("大于 10");
    } else {
        println!("小于等于 10");
    }
}
