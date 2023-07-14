use super::token::{Token, TokenInfo};

#[derive(Clone, Debug, PartialEq)]
pub enum AST {
    Str,
    Where,
    Use,
    As,
    Pub,
    Mod,
    None
}

impl From<&str> for AST {
    fn from(entry: &str) -> Self {
        match entry {
            "str" => AST::Str,
            "where" => AST::Where,
            "use" => AST::Use,
            "as" => AST::As,
            "pub" => AST::Pub,
            "mod" => AST::Mod,
            _ => AST::None,
        }
    }
}

impl From<String> for AST {
    fn from(value: String) -> Self {
        AST::from(value.as_str())
    }
}

impl From<TokenInfo> for AST {
    fn from(entry: TokenInfo) -> Self {
        match entry.identifier {
            Token::Identifier => Self::from(entry.content.unwrap_or(String::new())),
            Token::String => AST::Str,
            _ => AST::None,
        }
    }
}

impl From<Token> for AST {
    fn from(entry: Token) -> Self {
        match entry {
            Token::Identifier => AST::Str,
            Token::String => AST::Str,
            _ => AST::None,
        }
    }
}
