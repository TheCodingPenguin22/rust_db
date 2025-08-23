use crate::{db::{DataBaseTable, DataType}, sql::parse::{parse_sql, Keywords}};

mod parse;

pub fn handle_sql(command: Vec<&str>) {
    let comand_tokenized = parse_sql(command);

    let mut _table: DataBaseTable;
    if comand_tokenized[0] == Keywords::CREATE && comand_tokenized[1] == Keywords::TABLE {
        let column_types = vec![DataType::String("".to_string())];
        match &comand_tokenized[2] {
            Keywords::VARIABLE(v) => _table = DataBaseTable::new(String::from(v), column_types),
            _ => panic!("ERROR"),
        }
    }
}
