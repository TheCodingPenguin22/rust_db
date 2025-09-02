#[derive(Debug, Eq, PartialEq)]
pub enum Keywords {
    CREATE,
    INSERT,
    INTO,
    SELECT,
    FROM,
    SHOW,
    TABLES,
    TABLE,
    DATABASE,
    LPAREN,
    RPAREN,
    COMMA,
    WILDCARD,
    RQUOTE,
    LQUOTE,
    STRING,
    INTEGER,
    BOOL,
    VARIABLE(String),
}
 impl std::fmt::Display for Keywords{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Keywords::CREATE => write!(f, "CREATE" ),
            Keywords::INSERT => write!(f, "INSERT" ),
            Keywords::INTO => write!(f, "INTO" ),
            Keywords::SELECT => write!(f, "SELECT" ),
            Keywords::FROM => write!(f, "FROM" ),
            Keywords::SHOW => write!(f, "SHOW" ),
            Keywords::TABLES => write!(f, "TABLES" ),
            Keywords::TABLE => write!(f, "TABLE" ),
            Keywords::DATABASE => write!(f, "DATABASE" ),
            Keywords::LPAREN => write!(f, "LPAREN" ),
            Keywords::RPAREN => write!(f, "RPAREN" ),
            Keywords::COMMA => write!(f, "COMMA" ),
            Keywords::WILDCARD => write!(f, "WILDCARD" ),
            Keywords::RQUOTE => write!(f, "RQUOTE" ),
            Keywords::LQUOTE => write!(f, "LQUOTE" ),
            Keywords::STRING => write!(f, "STRING" ),
            Keywords::INTEGER => write!(f, "INTEGER" ),
            Keywords::BOOL => write!(f, "BOOL" ),
            Keywords::VARIABLE(_) => write!(f, "VARIABLE"),
        }
    }
 }

pub fn parse_sql(command: Vec<String>) -> Vec<Keywords> {
    let mut keywords: Vec<Keywords> = Vec::new();
    for i in 0..command.len() {
        let c = &command[i];
        match c.trim() {
            "CREATE" => keywords.push(Keywords::CREATE),
            "SELECT" => keywords.push(Keywords::SELECT),
            "FROM" => keywords.push(Keywords::FROM),
            "SHOW" => keywords.push(Keywords::SHOW),
            "TABLES" => keywords.push(Keywords::TABLES),
            "TABLE" => keywords.push(Keywords::TABLE),
            "DATABASE" => keywords.push(Keywords::DATABASE),
            "INSERT" => keywords.push(Keywords::INSERT),
            "INTO" => keywords.push(Keywords::INTO),
            "*" => keywords.push(Keywords::WILDCARD),
            "(" => keywords.push(Keywords::LPAREN),
            ")" => keywords.push(Keywords::RPAREN),
            "\"" => {
                if i + 2 >= command.len() {
                    keywords.push(Keywords::RQUOTE);
                }
                else if &command[i + 2] == "\"" {
                    keywords.push(Keywords::LQUOTE);
                } else {
                    keywords.push(Keywords::RQUOTE);
                }
            }
            "," => keywords.push(Keywords::COMMA),
            "STRING" => keywords.push(Keywords::STRING),
            "INTEGER" => keywords.push(Keywords::INTEGER),
            "BOOL" => keywords.push(Keywords::BOOL),
            _ => keywords.push(Keywords::VARIABLE(c.to_string())),
        }
    }
    keywords
}

pub fn tokenize_command(command: String) -> Vec<String> {
    let mut tokenized_command: Vec<String> = Vec::new();

    let mut word: String = String::new();
    let chars: Vec<char> = command.chars().collect();
    let delimiters: Vec<char> = "\",();".chars().collect();
    for char in chars.iter() {
        if char.is_whitespace() {
            if !word.is_empty() {
                tokenized_command.push(word.clone());
            }
            word = String::new();
            continue;
        }
        if delimiters.contains(char) {
            if !word.is_empty() {
                tokenized_command.push(word.clone());
            }
            word = String::new();

            word.push(*char);
            tokenized_command.push(word.clone());
            word = String::new();
        } else {
            word.push(*char);
        }
    }

    if !word.is_empty() {
        tokenized_command.push(word.trim().to_string());
    }

    tokenized_command
}
