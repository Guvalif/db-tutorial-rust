use std::process::exit;

pub fn execute(command: &str) -> Result<(), String> {
    match command {
        ".exit" => exit(0),
        _ => Err(format!("Unrecognized meta command '{}'.", command)),
    }
}
