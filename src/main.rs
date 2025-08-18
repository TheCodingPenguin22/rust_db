use db::{DataBase, DataBaseColumn, DataBaseEntry, DataBaseRow, DataType};
mod db;
fn main() {
    let mut db = DataBase::<String>::new();

    db.create_table(
        String::from("names"),
        vec![
            DataBaseColumn::new(String::from("id")),
            DataBaseColumn::new::<String>(String::from("name")),
        ],
    );

    let table = db.get_table(String::from("names")).unwrap();
    table.add_row(DataBaseRow::new(
        table,
        vec![
            DataBaseEntry::new(1, Data),
            DataBaseEntry::new(String::from("Kalle"))
        ],
    ));
    table.print_table();
}
