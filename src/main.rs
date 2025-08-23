use std::io;
//use db::{DataBaseEntry, DataBaseTable, DataType, DataBaseRow};
use sql::handle_sql;


mod sql;
mod db;
fn main() {
    println!("Welcome to a demo of rust_db!");
    println!("Please enter the command you want to execute:");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("unbable to read input");
    let input = input.trim().split(" ");
    let input = input.collect::<Vec<&str>>();

    handle_sql(input);

}
