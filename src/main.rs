use db::{DataBaseEntry, DataBaseTable, DataType};

use crate::db::DataBaseRow;
mod db;
fn main() {
    let mut table = DataBaseTable::new(
        "test".to_string(),
        vec![DataType::String("".to_string()), DataType::Integer(0)],
    );
    let entries = vec![
        DataBaseEntry::new(DataType::String("hello".to_string())),
        DataBaseEntry::new(DataType::Integer(22)),
    ];

    let db_row = DataBaseRow::new(entries);
    match table.add_row(db_row) {
        Ok(()) => println!("ok!"),
        Err(s) => println!("{s}"),
    }

    let db_row = DataBaseRow::new(vec![
        DataBaseEntry::new(DataType::String("kalle".to_string())),
        DataBaseEntry::new(DataType::Integer(27)),
    ]);
    match table.add_row(db_row) {
        Ok(()) => println!("ok!"),
        Err(s) => println!("{s}"),
    }

    for i in 1..=2 {
        table.get_row(i).print();
    }
}
