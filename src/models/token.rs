use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Identifier,
    String,

    Unknown,
    None,
}

impl Token {
    pub fn from<T>(entry: T) -> Self
    where
        T: AsRef<str>,
    {
        match entry.as_ref() {
            "Identifier" => Token::Identifier,
            "String" => Token::String,
            "None" => Token::None,
            _ => Token::Unknown,
        }
    }
}

impl Token {
    pub fn to_string(&self) -> String {
        match self {
            Token::Identifier => String::from("Identifier"),
            Token::String => String::from("String"),
            Token::Unknown => String::from("Unknown"),
            Token::None => String::from("None"),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

// - - - - - - - - - - - - - - - - - - - - -

#[derive(Clone, Debug)]
pub struct TokenInfo {
    pub identifier: Token,
    pub content: Option<String>,
    pub cursor: usize,
}

impl TokenInfo {
    pub fn new(identifier: Token, content: Option<String>, cursor: usize) -> Self {
        Self { identifier, content, cursor }
    }
}

impl Default for TokenInfo {
    fn default() -> Self {
        Self { identifier: Token::None, content: None, cursor: 0 }
    }
}
