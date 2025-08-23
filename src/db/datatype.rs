/*
Created 17/08/2025 by Fredrik Adolfsson
*/
#[derive(PartialEq, Eq, Debug)]
pub enum DataType {
    String(String),
    Integer(i32),
    Bool(bool)
}

pub trait FromDataType<T> {
    fn from_data_type(&self) -> Option<T>;
}

impl FromDataType<String> for DataType {
    fn from_data_type(&self) -> Option<String> {
        if let DataType::String(s) = self {
            Some(s.clone())
        } else {
            None
        }
    }
}

impl FromDataType<i32> for DataType {
    fn from_data_type(&self) -> Option<i32> {
        if let DataType::Integer(i) = self {
            Some(*i)
        } else {
            None
        }
    }
}

impl FromDataType<bool> for DataType {
    fn from_data_type(&self) -> Option<bool> {
        if let DataType::Bool(b) = self {
            Some(*b)
        }
        else {
            None
        }
    }
}

impl DataType {
    pub fn print(&self) {
        match self {
            DataType::String(_) => self.print_string(),
            DataType::Integer(_) => self.print_i32(),
            DataType::Bool(_) => self.print_bool(),
        }
    }
    fn print_string(&self)
    where
        Self: FromDataType<String>,
    {
        let result: Option<String> = self.from_data_type();
        if let Some(s) = result {
            println!("{s}");
        }
    }

    fn print_i32(&self)
    where
        Self: FromDataType<i32>,
    {
        let result: Option<i32> = self.from_data_type();
        if let Some(s) = result {
            println!("{s}");
        }
    }
    fn print_bool(&self)
        where 
        Self: FromDataType<bool>
    {
        let result: Option<bool> = self.from_data_type();
        if let Some(b) = result {
            println!("{b}");
        }
    }
            
}
