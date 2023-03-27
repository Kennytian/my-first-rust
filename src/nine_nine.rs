//  rustc --out-dir target/debug ./src/nine_nine.rs
// ./target/debug/nine_nine 2

fn main() {
    let row = 9;
    for x in 1..=row {
        for y in 1..=x {
            print!("{} x {} = {:<2}  ", x, y, x * y);
        }
        println!("");
    }
}
