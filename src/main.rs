use std::io::{stdin, stdout, Write};
use std::process::exit;

fn print_prompt() {
    print!("db > ");

    stdout().flush().unwrap();
}

fn main() {
    let mut line = String::new();

    loop {
        print_prompt();

        stdin().read_line(&mut line).expect("Unrecognized input.");

        let input = line.split_whitespace().next().unwrap();

        match input {
            ".exit" => exit(0),
            _ => println!("Unrecognized command '{}'.", input),
        }

        line.clear();
    }
}
