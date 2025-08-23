/*
Created 17/08/2025 by Fredrik Adolfsson
*/

pub use datatype::DataType;
pub mod datatype;

pub struct DataBaseTable {
    name: String,
    column_types: Vec<DataType>,
    rows: Vec<DataBaseRow>,
}

impl DataBaseTable {
    pub fn new(name: String, column_types: Vec<DataType>) -> Self {
        Self {
            name,
            column_types,
            rows: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: DataBaseRow) -> Result<(), &'static str> {
        if self.column_types.len() != row.entries.len() {
            return Err("Column missmatch")
        }

        for i in 0..row.entries.iter().len() {
            match (&row.entries[i].data, &self.column_types[i]) {
                (DataType::String(_), DataType::String(_)) => (),
                (DataType::Integer(_), DataType::Integer(_)) => (),
                _ => return Err("Type missmatch")
            }
        }
        self.rows.push(row);
        Ok(())
    }
    pub fn get_row(&self, row_num: i32) -> &DataBaseRow {
        &self.rows[(row_num - 1) as usize]
    }
}

#[derive(Debug)]
pub struct DataBaseEntry {
    data: DataType,
}

impl DataBaseEntry {
    pub fn new(data: DataType) -> Self {
        Self { data }
    }
}

pub struct DataBaseRow {
    entries: Vec<DataBaseEntry>,
}

impl DataBaseRow {
    pub fn new(entries: Vec<DataBaseEntry>) -> Self {
        Self { entries }
    }
    pub fn print(&self) {
        for entry in self.entries.iter() {
            entry.data.print();
        }
    }
}
