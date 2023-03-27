//  rustc --out-dir target/debug ./src/cli.rs
// ./target/debug/cli 1 2

fn cli_enter() {
    let mut args = std::env::args().skip(1);
    let action = args.next().expect("Please specify an action");
    let item = args.next().expect("Please specify an item");

    println!("{:?}, {:?}", action, item);
}

fn main () {
    cli_enter();
}
