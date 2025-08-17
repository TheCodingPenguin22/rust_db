use db::{DataBase, DataBaseColumn, DataBaseRow};
mod db;
fn main() {
    let mut db = DataBase::<String>::new();

    db.create_table(
        String::from("names"),
        vec![
            DataBaseColumn::new::<i32>(String::from("id")),
            DataBaseColumn::new::<String>(String::from("name")),
        ],
    );

    let table = db.get_table(String::from("names")).unwrap();
    table.add_row(DataBaseRow::new(vec![
        String::from("1"),
        String::from("kalle"),
    ]));
    table.add_row(DataBaseRow::new(vec![
        String::from("2"),
        String::from("hanna"),
    ]));
    table.print_table();
}
