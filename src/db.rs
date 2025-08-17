use std::any::{Any, TypeId};
use std::fmt::Display;
use std::fmt;

pub struct DataBase<T> {
    tables: Vec<DataBaseTable<T>>,
}

pub struct DataBaseColumn {
    name: String,
    type_id: TypeId,
    type_name: &'static str,
}

pub struct DataBaseTable<T> {
    name: String,
    columns: Vec<DataBaseColumn>,
    rows: Vec<DataBaseRow<T>>,
    column_count: i32,
}

pub struct DataBaseRow<T> {
    content: Vec<T>,
}
impl std::fmt::Display for DataBaseColumn {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl DataBaseColumn {
    pub fn new<T: 'static + Any + Display>(name: String) -> Self {
        Self {
            name,
            type_id: TypeId::of::<T>(),
            type_name: std::any::type_name::<T>() 
        }
    }
}

impl<T: Display> DataBase<T> {
    pub fn new() -> Self {
        Self { tables: Vec::new() }
    }
    pub fn create_table(&mut self, name: String, columns: Vec<DataBaseColumn>) {
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
    pub fn new(name: String, columns: Vec<DataBaseColumn>) -> Self {
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
