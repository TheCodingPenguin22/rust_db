use std::fmt::Display;
pub struct DataBase<T> {
    columns: Vec<String>,
    rows: Vec<DataBaseRow<T>>
}

pub struct DataBaseRow<T> {
    content: Vec<T>,
}

impl<T: Display> DataBase<T> {
    pub fn new(columns: Vec<String>, rows: Vec<DataBaseRow<T>>) -> Self {
        Self { columns, rows }
    }
    fn print_cols(&self) {
        print!("| ");
        let mut index = 0;
        for el in self.columns.iter() {
            index += 1;
            if index != self.columns.len() {
                print!("{} -- ", el);
            }
            else {
                println!("{} |", el);
            }
        }
    }

    pub fn print_table(&self){
        self.print_cols();

        if self.rows.len() > 1 {
            for row in self.rows.iter() {
                print!("| ");
                for element in row.content.iter() {
                    print!("{} ", element);
                }
                println!(" |");
            }
        }
    }
    pub fn add_row(&mut self, row: DataBaseRow<T>) {
        self.rows.push(row);
    }
}

impl<T> DataBaseRow<T> {
    pub fn new(content: Vec<T>) -> Self {
        Self { content }
    }
}
