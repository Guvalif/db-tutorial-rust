use row::Row;
use table::Table;

enum StatementType {
    StatementInsert,
    StatementSelect,
}

enum StatementParams {
    NullaryParams,
    InsertParams(u32, String, String),
}

pub struct Statement {
    s_type: StatementType,
    s_params: StatementParams,
}

fn prepare_insert_handler(command: &str) -> Result<Statement, String> {
    let mut iter = command.split_whitespace();

    let _ = iter.next();
    let id = iter.next().and_then(|v| v.parse::<u32>().ok());
    let username = iter.next();
    let email = iter.next();
    let eos = iter.next();

    match (id, username, email, eos) {
        (Some(id), Some(username), Some(email), None) => Ok(Statement {
            s_type: StatementType::StatementInsert,
            s_params: StatementParams::InsertParams(id, username.to_owned(), email.to_owned()),
        }),
        _ => Err("Syntax Error: Could not parse statement.".to_owned()),
    }
}

fn prepare_select_handler() -> Result<Statement, String> {
    Ok(Statement {
        s_type: StatementType::StatementSelect,
        s_params: StatementParams::NullaryParams,
    })
}

pub fn prepare(command: &str) -> Result<Statement, String> {
    match command {
        _ if command.starts_with("insert") => prepare_insert_handler(command),
        "select" => prepare_select_handler(),
        _ => Err(format!("Syntax Error: Unrecognized command '{}'.", command)),
    }
}

fn execute_insert_handler(table: &mut Table, params: &StatementParams) -> Result<(), String> {
    let (id, username, email) = match params {
        StatementParams::InsertParams(id, username, email) => (id, username, email),
        _ => panic!("Accepts only 'StatementParams::InsertParams'!"),
    };

    Row::new(*id, username, email).and_then(|r| table.insert(r).map(|_| ()))
}

fn execute_select_handler(table: &Table) -> Result<(), String> {
    print!("{}", table);

    Ok(())
}

impl Statement {
    pub fn execute(&self, table: &mut Table) -> Result<(), String> {
        match self.s_type {
            StatementType::StatementInsert => execute_insert_handler(table, &self.s_params),
            StatementType::StatementSelect => execute_select_handler(table),
        }
    }
}
