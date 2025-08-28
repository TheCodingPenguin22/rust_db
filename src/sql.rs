use crate::{
    db::{DataBase, DataBaseColumn, DataBaseTable, DataType},
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
    }
}

fn create_database(db: &mut DataBase, cmd_tokenized: &Vec<Keywords>) {
    match &cmd_tokenized[2] {
        Keywords::VARIABLE(v) => {
            db.init(v.to_string());
        }
        _ => panic!("ERROR"),
    }
}
fn create_table(db: &mut DataBase, cmd_tokenized: &Vec<Keywords>) {
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

fn create_columns(cmd_tokenized: &Vec<Keywords>) -> Vec<DataBaseColumn> {
    let mut columns: Vec<DataBaseColumn> = Vec::new();
    for i in 3..cmd_tokenized.len() {
        match &cmd_tokenized[i] {
            Keywords::LPAREN => continue,
            Keywords::RPAREN => break,
            Keywords::VARIABLE(v) => continue,
            Keywords::INTEGER => {
                let name = match &cmd_tokenized[i-1] {
                    Keywords::VARIABLE(v) => v.to_string(),
                    _ => panic!("ERROR"),
                };
                columns.push(DataBaseColumn::new(name, DataType::Integer(0)));
            },
            Keywords::STRING => {
                let name = match &cmd_tokenized[i-1] {
                    Keywords::VARIABLE(v) => v.to_string(),
                    _ => panic!("ERROR"),
                };
                columns.push(DataBaseColumn::new(name, DataType::String("".to_string())));
            },
            Keywords::BOOL => {
                let name = match &cmd_tokenized[i-1] {
                    Keywords::VARIABLE(v) => v.to_string(),
                    _ => panic!("ERROR"),
                };
                columns.push(DataBaseColumn::new(name, DataType::Bool(true)));
            },
            _ => panic!("ERROR!"),
        }
    }
    columns
}
