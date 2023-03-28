// rustc --out-dir target/debug ./src/read_lines.rs; ./target/debug/read_lines

use std::io;

fn read_words_from_readline() {
    let mut input: String = String::new();

    println!("Please enter some text: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    println!("You typed: {}", input.trim());
}

fn main() {
    read_words_from_readline();
}
