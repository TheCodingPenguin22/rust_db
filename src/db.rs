use std::fmt::Display;

pub struct DataBase<T> {
    tables: Vec<DataBaseTable<T>>,
}

pub struct DataBaseTable<T> {
    name: String,
    columns: Vec<String>,
    rows: Vec<DataBaseRow<T>>,
    column_count: i32,
}

pub struct DataBaseRow<T> {
    content: Vec<T>,
}

impl<T: Display> DataBase<T> {
    pub fn new() -> Self {
        Self { tables: Vec::new() }
    }
    pub fn create_table(&mut self, name: String, columns: Vec<String>) {
        let db_table = DataBaseTable::new(name, columns);
        self.tables.push(db_table);
    }

    pub fn get_table(&mut self, table_name: String) -> Result<&mut DataBaseTable<T>, &'static str> {
        for table in self.tables.iter_mut() {
            if table.name == table_name {
                return Ok(table);
            }
        }
        Err("Table not found...")
    }
}

impl<T: Display> DataBaseTable<T> {
    pub fn new(name: String, columns: Vec<String>) -> Self {
        let column_count = columns.len() as i32;
        Self {
            name,
            columns,
            rows: Vec::new(),
            column_count,
        }
    }
    fn print_cols(&self) {
        print!("| ");
        let mut index = 0;
        for el in self.columns.iter() {
            index += 1;
            if index != self.columns.len() {
                print!("{} -- ", el);
            } else {
                println!("{} |", el);
            }
        }
    }

    pub fn print_table(&self) {
        self.print_cols();

        if !self.rows.is_empty() {
            for row in self.rows.iter() {
                print!("| ");
                for element in row.content.iter() {
                    print!("{} ", element);
                }
                println!(" |");
            }
        }
    }
    fn add_row_to_db(&mut self, row: DataBaseRow<T>) -> Result<(), &'static str> {
        if row.content.len() as i32 != self.column_count {
            return Err("Row length does not match column count...");
        }
        self.rows.push(row);
        Ok(())
    }
    pub fn add_row(&mut self, row: DataBaseRow<T>) {
        match self.add_row_to_db(row) {
            Ok(()) => println!("Row added!"),
            Err(e) => println!("Failed to add row, {e}"),
        }
    }
}

impl<T> DataBaseRow<T> {
    pub fn new(content: Vec<T>) -> Self {
        Self { content }
    }
}
