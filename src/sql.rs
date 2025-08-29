use crate::{
    db::{DataBase, DataBaseColumn, DataBaseEntry, DataBaseRow, DataBaseTable, DataType},
    sql::parse::{parse_sql, tokenize_command, Keywords},
};

mod parse;

pub fn handle_sql(command: String, db: &mut DataBase) {
    let command_vec: Vec<String> = tokenize_command(command);
    let cmd_tokenized = parse_sql(command_vec);
    dbg!(&cmd_tokenized);

    if cmd_tokenized[0] == Keywords::CREATE {
        if cmd_tokenized[1] == Keywords::DATABASE {
            create_database(db, &cmd_tokenized);
        }
        if cmd_tokenized[1] == Keywords::TABLE {
            create_table(db, &cmd_tokenized);
        }
    } else if cmd_tokenized[0] == Keywords::INSERT && cmd_tokenized[1] == Keywords::INTO {
        insert_into_table(db, &cmd_tokenized);
    }
}

fn insert_into_table(db: &mut DataBase, cmd_tokenized: &[Keywords]) {
    let table_name: String = match &cmd_tokenized[2] {
        Keywords::VARIABLE(s) => s.to_string(),
        _ => panic!("No table name found"),
    };
    let table = db.get_table(table_name);

    let table = match table {
        Ok(t) => t,
        Err(_) => panic!("whoopps"),
    };

    let entries = create_table_entires(&cmd_tokenized[3..]);
    dbg!(&entries);

    match table.add_row(DataBaseRow::new(entries)) {
        Ok(_) => println!("Row added"),
        Err(e) => println!("{e}"),
    };
    dbg!(db);
}

fn create_table_entires(cmd_tokenized: &[Keywords]) -> Vec<DataBaseEntry> {
    let mut entries: Vec<DataBaseEntry> = Vec::new();
    let mut i = 0;
    while i < cmd_tokenized.len(){
        dbg!(&cmd_tokenized[i]);
        match &cmd_tokenized[i] {
            Keywords::LPAREN => (),
            Keywords::RQUOTE => (),
            Keywords::COMMA => (),
            Keywords::RPAREN => break,
            Keywords::LQUOTE => {
                let string = parse_string(&cmd_tokenized[i..i + 2]);
                let entry = DataBaseEntry::new(DataType::String(string));
                entries.push(entry);
                i += 2;
            }
            Keywords::VARIABLE(_) => {
                let datatype = parse_variable(&cmd_tokenized[i]);
                let entry = DataBaseEntry::new(datatype);
                entries.push(entry);
            }
            _ => panic!("AHAAHAHA"),
        };
        i += 1;
    }

    entries
}

fn parse_variable(cmd_tokenized: &Keywords) -> DataType {
    match &cmd_tokenized {
        Keywords::VARIABLE(v) => {
            println!("{v}");
            if v == "true" {
                DataType::Bool(true)
            } else if v == "false" {
                DataType::Bool(false)
            } else if v.trim().parse::<i32>().is_ok() {
                let number = v.trim().parse::<i32>().unwrap();
                DataType::Integer(number)
            } else {
                DataType::String(v.to_string())
            }
        }
        _ => panic!("HELP!!!"),
    }
}
fn parse_string(cmd_tokenized: &[Keywords]) -> String {
    dbg!(cmd_tokenized);
    match &cmd_tokenized[1] {
        Keywords::VARIABLE(s) => s.to_string(),
        _ => panic!("String not found"),
    }
}

fn create_database(db: &mut DataBase, cmd_tokenized: &[Keywords]) {
    match &cmd_tokenized[2] {
        Keywords::VARIABLE(v) => {
            db.init(v.to_string());
        }
        _ => panic!("ERROR"),
    }
}
fn create_table(db: &mut DataBase, cmd_tokenized: &[Keywords]) {
    if !db.is_init() {
        println!(
            "Database is not initialized. Please initialized a database before adding a table."
        );
    } else {
        let column_types = create_columns(cmd_tokenized);
        match &cmd_tokenized[2] {
            Keywords::VARIABLE(v) => {
                db.add_table(DataBaseTable::new(String::from(v), column_types))
            }
            _ => panic!("ERROR"),
        }
    }
    dbg!(db);
}

fn create_columns(cmd_tokenized: &[Keywords]) -> Vec<DataBaseColumn> {
    let mut columns: Vec<DataBaseColumn> = Vec::new();
    for i in 3..cmd_tokenized.len() {
        match &cmd_tokenized[i] {
            Keywords::LPAREN => continue,
            Keywords::COMMA => continue,
            Keywords::RPAREN => break,
            Keywords::VARIABLE(_) => continue,
            Keywords::INTEGER => {
                let name = match &cmd_tokenized[i - 1] {
                    Keywords::VARIABLE(v) => v.to_string(),
                    _ => panic!("ERROR"),
                };
                columns.push(DataBaseColumn::new(name, DataType::Integer(0)));
            }
            Keywords::STRING => {
                let name = match &cmd_tokenized[i - 1] {
                    Keywords::VARIABLE(v) => v.to_string(),
                    _ => panic!("ERROR"),
                };
                columns.push(DataBaseColumn::new(name, DataType::String("".to_string())));
            }
            Keywords::BOOL => {
                let name = match &cmd_tokenized[i - 1] {
                    Keywords::VARIABLE(v) => v.to_string(),
                    _ => panic!("ERROR"),
                };
                columns.push(DataBaseColumn::new(name, DataType::Bool(true)));
            }
            _ => panic!("ERROR!"),
        }
    }
    columns
}
