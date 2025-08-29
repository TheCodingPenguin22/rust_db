use sql::handle_sql;
use std::io;

use crate::db::{DataBase, DataBaseColumn, DataBaseTable, DataType};

mod db;
mod sql;
fn main() {
    println!("Welcome to a demo of rust_db!");
    let mut db = DataBase::new("UNINITIALIZED".to_string());
    loop {
        println!("Please enter the command you want to execute:");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("unbable to read input");

        if input.trim() == "EXIT" {
            println!("Thanks for using rust_db!");
            break;
        }

        handle_sql(input.to_string(), &mut db);
    }
}
