#[derive(Debug, Eq, PartialEq)]
pub enum Keywords {
    CREATE,
    SELECT,
    FROM,
    TABLE,
    DATABASE,
    VARIABLE(String)
}

pub fn parse_sql(command: Vec<&str>) -> Vec<Keywords> {
    let mut keywords: Vec<Keywords> = Vec::new();
    for c in command.iter() {
        match *c {
            "CREATE" => keywords.push(Keywords::CREATE),
            "SELECT" => keywords.push(Keywords::SELECT),
            "FROM" => keywords.push(Keywords::FROM),
            "TABLE" => keywords.push(Keywords::TABLE),
            "DATABASE" => keywords.push(Keywords::DATABASE),
            _ => keywords.push(Keywords::VARIABLE(c.to_string()))
        }
    }
    keywords
}
