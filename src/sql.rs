use crate::{
    db::{DataBase, DataBaseColumn, DataBaseEntry, DataBaseRow, DataBaseTable, DataType},
    sql::{
        parse::{Keywords, parse_sql, tokenize_command},
        pretty_print::{print_cols, print_table, print_tables},
    },
};

mod parse;
mod pretty_print;

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
    } else if cmd_tokenized[0] == Keywords::SHOW && cmd_tokenized[1] == Keywords::TABLES {
        print_tables(&*db);
    } else if cmd_tokenized[0] == Keywords::SELECT {
        if cmd_tokenized[1] == Keywords::WILDCARD && cmd_tokenized[2] == Keywords::FROM {
            let table = match &cmd_tokenized[3] {
                Keywords::VARIABLE(v) => db.get_table(v.to_string()),
                _ => Err("Wrong keyword"),
            };
            match table {
                Ok(t) => print_table(t),
                Err(e) => println!("Table not found. Error: {e}"),
            };
        } else {
            match cmd_tokenized[1] {
                Keywords::VARIABLE(_) => {
                    let cols = select_columns(db, &cmd_tokenized[1..]);
                    let cols = match cols {
                        Ok(c) => c,
                        Err(_) => panic!("ASKJLASKHJ"),
                    };
                    let table = match &cmd_tokenized[cmd_tokenized.len() - 1] {
                        Keywords::VARIABLE(v) => db.get_table(v.to_string()),
                        _ => panic!("JKJHBASD"),
                    };

                    let table = match table {
                        Ok(t) => t,
                        Err(_) => panic!("askdjh"),
                    };
                    print_cols(table, cols);
                }
                _ => println!("Wrong command. Cannot find variable {}.", cmd_tokenized[1]),
            };
        }
    }
}

fn select_columns(
    db: &mut DataBase,
    cmd_tokenized: &[Keywords],
) -> Result<Vec<DataBaseColumn>, &'static str> {
    let mut cols_found: Vec<DataBaseColumn> = Vec::new();
    let table = match &cmd_tokenized[cmd_tokenized.len() - 1] {
        Keywords::VARIABLE(v) => db.get_table(v.to_string()),
        _ => return Err("Last word of command is not a variable."),
    };

    let table = match table {
        Ok(v) => v,
        Err(_) => return Err("No table with given name found"),
    };
    let cols = table.get_columns();

    for word in cmd_tokenized.iter() {
        match word {
            Keywords::COMMA => continue,
            Keywords::FROM => break,
            Keywords::VARIABLE(col_name) => {
                for col in cols.iter() {
                    if col.get_name() == col_name {
                        cols_found.push(col.clone());
                    }
                }
            }
            _ => return Err("Unexpected keyword."),
        };
    }
    if cols_found.is_empty() {
        return Err("No columns found matching given names");
    }

    Ok(cols_found)
}

fn insert_into_table(db: &mut DataBase, cmd_tokenized: &[Keywords]) {
    let table_result = match &cmd_tokenized[2] {
        Keywords::VARIABLE(s) => db.get_table(s.to_string()),
        _ => Err("No table name found"),
    };
    let table;
    match table_result {
        Ok(t) => {
            table = t;
            let entries = create_table_entires(&cmd_tokenized[3..]);

            match table.add_row(DataBaseRow::new(entries)) {
                Ok(_) => println!("Row added"),
                Err(e) => println!("{e}"),
            };
        }
        Err(e) => {
            println!("{e}")
        }
    };
}

fn create_table_entires(cmd_tokenized: &[Keywords]) -> Vec<DataBaseEntry> {
    let mut entries: Vec<DataBaseEntry> = Vec::new();
    let mut i = 0;
    while i < cmd_tokenized.len() {
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
        _ => println!(
            "Error creating database. Found {}, expected {}",
            cmd_tokenized[2],
            Keywords::VARIABLE("".to_string())
        ),
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
            _ => {
                println!(
                    "Wrong Keyword. Found {} expected {}.",
                    &cmd_tokenized[2],
                    Keywords::VARIABLE("".to_string())
                )
            }
        }
    }
}

fn create_columns(cmd_tokenized: &[Keywords]) -> Vec<DataBaseColumn> {
    let mut columns: Vec<DataBaseColumn> = Vec::new();
    let mut table_count = 0;
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
                columns.push(DataBaseColumn::new(table_count, name, DataType::Integer(0)));
                table_count += 1;
            }
            Keywords::STRING => {
                let name = match &cmd_tokenized[i - 1] {
                    Keywords::VARIABLE(v) => v.to_string(),
                    _ => panic!("ERROR"),
                };
                columns.push(DataBaseColumn::new(
                    table_count,
                    name,
                    DataType::String("".to_string()),
                ));
                table_count += 1;
            }
            Keywords::BOOL => {
                let name = match &cmd_tokenized[i - 1] {
                    Keywords::VARIABLE(v) => v.to_string(),
                    _ => panic!("ERROR"),
                };
                columns.push(DataBaseColumn::new(table_count, name, DataType::Bool(true)));
                table_count += 1;
            }
            _ => panic!("ERROR!"),
        }
    }
    columns
}
