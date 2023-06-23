use std::str::Chars;

use crate::models::token::{Token, TokenInfo};

pub struct Scope <'a> {
    pub identifiers: Vec<TokenInfo>,
    pub chars: Chars<'a>,
    pub cursor: usize
}

impl <'a> Scope <'a> {
    pub fn next(&mut self) -> Option<char> {
        self.cursor += 1;
        self.chars.next()
    }
}

impl <'a> Scope <'a> {
    pub fn new(chars: Chars<'a>) -> Self {
        Self { identifiers: Vec::new(), chars, cursor: 0 }
    }
}

// - - - - - - - - - - - - - - - - - - - - -

pub fn lexer(scope: &mut Scope) -> Vec<TokenInfo> {
    while let Some(char) = scope.next() {
        if char == ' ' { continue; }
        
        match char {
            'a'..='z' | 'A'..='Z' => {
                let mut identifier = String::new();
                identifier.push(char);

                while let Some(char) = scope.next() {
                    if char == ' ' { break; }

                    identifier.push(char);
                }

                scope.identifiers.push(
                    TokenInfo::new(Token::Identifier, Some(identifier), scope.cursor)
                );
            },
            '"' => {
                let mut string = String::new();
                let mut escape = false;

                while let Some(char) = scope.next() {
                    if escape == false {
                        if char == '\\' { escape = true; continue }
                        if char == '"' { break; }
                    }

                    string.push(char);
                    escape = false;
                }

                scope.identifiers.push(
                    TokenInfo::new(Token::String, Some(string), scope.cursor)
                );
            },
            _ => {
                scope.identifiers.push(
                    TokenInfo::new(Token::Unknown, Some(char.to_string()), scope.cursor)
                );
            }
        }
    }

    scope.identifiers.clone()
}
