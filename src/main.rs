#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input= String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");

        let command = input.trim();

        println!("{command}: command not found");
    }
}