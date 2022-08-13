use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let input: &str = &args[1];

    if let Err(e) = lhs_rs::run(input) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}