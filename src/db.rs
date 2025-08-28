/*
Created 17/08/2025 by Fredrik Adolfsson
*/

pub use datatype::DataType;
pub mod datatype;
#[derive(Debug)]
pub struct DataBase {
    name: String,
    tables: Vec<DataBaseTable>,
    initialized: bool,
}

impl DataBase {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tables: Vec::new(),
            initialized: false,
        }
    }
    pub fn init(&mut self, name: String) {
        self.name = name;
        self.initialized = true;
    }
    pub fn is_init(&self) -> bool {
        self.initialized
    }
    pub fn add_table(&mut self, db_table: DataBaseTable) {
        self.tables.push(db_table);
    }
}

#[derive(Debug)]
pub struct DataBaseColumn {
    name: String,
    column_type: DataType,
}
impl DataBaseColumn {
    pub fn new(name: String, column_type: DataType) -> Self {
        Self { name, column_type }
    }
}

#[derive(Debug)]
pub struct DataBaseTable {
    name: String,
    columns: Vec<DataBaseColumn>,
    rows: Vec<DataBaseRow>,
}

impl DataBaseTable {
    pub fn new(name: String, columns: Vec<DataBaseColumn>) -> Self {
        Self {
            name,
            columns,
            rows: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: DataBaseRow) -> Result<(), &'static str> {
        if self.columns.len() != row.entries.len() {
            return Err("Column missmatch");
        }

        for i in 0..row.entries.iter().len() {
            match (&row.entries[i].data, &self.columns[i].column_type) {
                (DataType::String(_), DataType::String(_)) => (),
                (DataType::Integer(_), DataType::Integer(_)) => (),
                (DataType::Bool(_), DataType::Bool(_)) => (),
                _ => return Err("Type missmatch"),
            }
        }
        self.rows.push(row);
        Ok(())
    }
    pub fn get_row(&self, row_num: i32) -> &DataBaseRow {
        &self.rows[(row_num - 1) as usize]
    }
    pub fn get_name(&self) -> &String {
        &self.name
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

#[derive(Debug)]
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
