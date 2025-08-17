use db::{DataBase, DataBaseRow};
mod db;
fn main() {
    let mut db = DataBase::new(
        vec![String::from("id"), String::from("name")],
        vec![DataBaseRow::new(vec!["1", "hello"])],
    );

    let db_row = DataBaseRow::new(vec!["2", "hello!"]);

    db.add_row(db_row);
    db.print_table();
}
