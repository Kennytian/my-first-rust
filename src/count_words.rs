// rustc --out-dir target/debug ./src/count_words.rs; ./target/debug/count_words

use std::io;

fn count_words() {
    let mut input: String = String::new();
    println!("Please enter some text:");

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let delimiter = " ";
    let words_count: usize = input.trim().split(delimiter).filter(|word| !word.is_empty()).count();

    println!("{}", words_count);
}

fn main() {
    count_words();
}
