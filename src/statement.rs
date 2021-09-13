enum StatementType {
    StatementInsert,
    StatementSelect,
}

pub struct Statement {
    s_type: StatementType,
}

pub fn prepare(command: &str) -> Result<Statement, String> {
    match command {
        _ if command.starts_with("insert") => Ok(Statement {
            s_type: StatementType::StatementInsert,
        }),
        "select" => Ok(Statement {
            s_type: StatementType::StatementSelect,
        }),
        _ => Err(format!("Unrecognized command '{}'.", command)),
    }
}

impl Statement {
    pub fn execute(&self) {
        match self.s_type {
            StatementType::StatementInsert => println!("This is where we would do an insert."),
            StatementType::StatementSelect => println!("This is where we would do a select."),
        }
    }
}
