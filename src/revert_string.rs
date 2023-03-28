use std::env;

#[allow(dead_code)]
fn get_string_arg_v1(index: usize) -> Result<String, &'static str> {
    match env::args().nth(index) {
        Some(value) => Ok(value),
        None => Err("Please supply a string argument")
    }
}

fn get_string_arg(index: usize) -> Result<String, &'static str> {
    env::args().nth(index).ok_or("Please supply a string argument")
}

fn revert() {
    let origin_str = get_string_arg(1).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });

    println!("The origin string is: {}", origin_str);

    let reverted_str: String = origin_str.chars().rev().collect();
    println!("The reverted string is: {}", reverted_str);
}

fn main() {
    revert();
}
