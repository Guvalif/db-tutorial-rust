use std::io::{stdin, stdout, Write};

mod meta_command;
mod statement;

fn print_prompt() {
    print!("db > ");

    stdout().flush().unwrap();
}

fn interpreter() {
    print_prompt();

    let command = {
        let mut line = String::new();

        stdin().read_line(&mut line).expect("Unrecognized input.");

        line.trim().to_owned()
    };

    let result: Result<(), String> = match command {
        _ if command == "" => Ok(()),
        _ if command.starts_with(".") => meta_command::execute(&command),
        _ => statement::prepare(&command).map(|s| s.execute()),
    };

    result.unwrap_or_else(|e| println!("{}", e));
}

fn main() {
    loop {
        interpreter();
    }
}
