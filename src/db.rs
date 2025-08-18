/*
Created 17/08/2025 by Fredrik Adolfsson
*/

pub use datatype::DataType;
pub mod datatype;

pub struct DataBaseEntry {
    pub data: DataType,
}

pub struct DataBaseRow {
    entries: Vec<DataBaseEntry>,
}

impl DataBaseEntry {
    pub fn new(data: DataType) -> Self {
        Self { data }
    }
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
