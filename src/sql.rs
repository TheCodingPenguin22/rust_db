use crate::{
    db::{DataBase, DataBaseTable, DataType},
    sql::parse::{Keywords, parse_sql},
};

mod parse;

pub fn handle_sql(command: String, db: &mut DataBase) {
    let command_vec: Vec<&str> = command.split_whitespace().collect();
    let command_tokenized = parse_sql(command_vec);
    dbg!(&command_tokenized);

    if command_tokenized[0] == Keywords::CREATE {
        if command_tokenized[1] == Keywords::DATABASE {
            match &command_tokenized[2] {
                Keywords::VARIABLE(v) => {
                    db.init(v.to_string());
                }
                _ => panic!("ERROR"),
            }
        }
        if command_tokenized[1] == Keywords::TABLE {
            if !db.is_init() {
                println!(
                    "Database is not initialized. Please initialized a database before adding a table."
                );
            } else {
                let column_types = vec![DataType::String("".to_string())];
                match &command_tokenized[2] {
                    Keywords::VARIABLE(v) => {
                        db.add_table(DataBaseTable::new(String::from(v), column_types))
                    }
                    _ => panic!("ERROR"),
                }
            }
        }
        dbg!(db);
    }
}
