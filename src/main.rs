#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");

        let args = input.trim().split_whitespace().collect::<Vec<&str>>();

        if args.is_empty() {
            continue;
        }

        match args.as_slice() {
            ["exit", ..] => break,
            ["echo", msg @ ..] => println!("{}", msg.join(" ")),
            ["type", cmd] if ["exit", "echo", "type"].contains(cmd) => {
                println!("{} is a shell builtin", cmd)
            }
            ["type", cmd @ ..] => println!("{}: not found", cmd.join(" ")),
            _ => eprintln!("{}: command not found", args[0]),
        }
    }
}
