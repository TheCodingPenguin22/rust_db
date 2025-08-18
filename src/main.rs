use db::{DataBaseEntry, DataType};

use crate::db::DataBaseRow;
mod db;
fn main() {
    let entries = vec![DataBaseEntry::new(DataType::String(String::from("hello"))), 
                        DataBaseEntry::new(DataType::Integer(22))];

    let db_row = DataBaseRow::new(entries);

    db_row.print();
    
}
