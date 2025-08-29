#[derive(Debug, Eq, PartialEq)]
pub enum Keywords {
    CREATE,
    INSERT,
    INTO,
    SELECT,
    FROM,
    TABLE,
    DATABASE,
    LPAREN,
    RPAREN,
    COMMA,
    RQUOTE,
    LQUOTE,
    STRING,
    INTEGER,
    BOOL,
    VARIABLE(String),
}

pub fn parse_sql(command: Vec<String>) -> Vec<Keywords> {
    let mut keywords: Vec<Keywords> = Vec::new();
    for i in 0..command.len() {
        let c = &command[i];
        match c.trim() {
            "CREATE" => keywords.push(Keywords::CREATE),
            "SELECT" => keywords.push(Keywords::SELECT),
            "FROM" => keywords.push(Keywords::FROM),
            "TABLE" => keywords.push(Keywords::TABLE),
            "DATABASE" => keywords.push(Keywords::DATABASE),
            "INSERT" => keywords.push(Keywords::INSERT),
            "INTO" => keywords.push(Keywords::INTO),
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

    dbg!(&tokenized_command);
    tokenized_command
}
