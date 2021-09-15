use std::io::{stdin, stdout, Write};

mod meta_command;
mod row;
mod statement;
mod table;

fn print_prompt() {
    print!("db > ");

    stdout().flush().unwrap();
}

fn interpreter(table: &mut table::Table) {
    print_prompt();

    let command = {
        let mut line = String::new();

        stdin().read_line(&mut line).expect("Unrecognized input.");

        line.trim().to_owned()
    };

    let result: Result<(), String> = match command {
        _ if command.is_empty() => Ok(()),
        _ if command.starts_with('.') => meta_command::execute(&command),
        _ => statement::prepare(&command).and_then(|s| s.execute(table)),
    };

    result.unwrap_or_else(|e| println!("{}", e));
}

fn main() {
    let mut table = table::Table::new();

    loop {
        interpreter(&mut table);
    }
}
