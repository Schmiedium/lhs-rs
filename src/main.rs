use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let input: &str = &args[1].unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = lhsrs::run(input) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}