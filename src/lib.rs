use std::{str::Chars, fmt::Display, path::Path};

extern crate proc_macro;

use fancy_regex::Regex;

// - - -

#[derive(Clone, Debug, PartialEq)]
enum Token {
    Identifier,
    String,
    Unknown,
    None
}

impl Display for Token{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Identifier => write!(f, "Identifier"),
            Token::String => write!(f, "String"),
            Token::Unknown => write!(f, "Unknown"),
            Token::None => write!(f, "None")
        }
    }
}

#[derive(Clone, Debug)]
struct TokenInfo {
    category: Token,
    value: String,
    cursor: usize
}

impl Default for TokenInfo {
    fn default() -> Self {
        Self { category: Token::None, value: String::new(), cursor: 0 }
    }
}

impl TokenInfo {
    fn default_with_cursor(cursor: usize) -> Self {
        Self { category: Token::None, value: String::new(), cursor }
    }
}

impl TokenInfo {
    pub fn new(category: Token, value: String, cursor: usize) -> Self {
        Self { category, value, cursor }
    }
}

// - - -

struct LexerScope <'a> {
    pub identifiers: Vec<TokenInfo>,
    pub chars: Chars<'a>,
    pub cursor: usize
}

impl <'a> LexerScope <'a> {
    pub fn next(&mut self) -> Option<char> {
        self.cursor += 1;
        self.chars.next()
    }
}

impl <'a> LexerScope <'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            identifiers: Vec::new(), chars: input.chars(), cursor: 0
        }
    }
}

fn lexer(scope: &mut LexerScope) -> Vec<TokenInfo> {
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
                    TokenInfo::new(Token::Identifier, identifier, scope.cursor)
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
                    TokenInfo::new(Token::String, string, scope.cursor)
                );
            },
            _ => {
                scope.identifiers.push(
                    TokenInfo::new(Token::Unknown, char.to_string(), scope.cursor)
                );
            }
        }
    }

    scope.identifiers.clone()
}

// - - -

struct ParserScope {
    pub document: String,
    pub tokens: Vec<TokenInfo>,
    pub cursor: usize
}

impl ParserScope {
    pub fn new(document: String, tokens: Vec<TokenInfo>) -> Self {
        Self { document, tokens, cursor: 0 }
    }
}

impl ParserScope {
    pub fn next(&mut self) -> Option<TokenInfo> {
        self.cursor += 1;
        self.tokens.get(self.cursor - 1).cloned()
    }
}

impl ParserScope {
    pub fn panic(&self, message: String, cursor: usize) {
        let mut trace = String::new();

        for _ in 0..(cursor - 1) {
            trace.push('-');
        }

        trace.push('^');

        panic!(
            "{}\n {}\n {}", message, self.document, trace
        );
    }
}

fn parser(scope: &mut ParserScope) {
    // [Directory]
    let token = scope.next().unwrap_or(TokenInfo::default());

    if token.category != Token::String {
        scope.panic(
            format!("Expected string, found \"None\" at {}", token.cursor),
            token.cursor
        );
    }

    // Validate directory
    if Path::new(&token.value).exists() == false {
        scope.panic(
            format!("Directory \"{}\" does not exist at {}", token.value, token.cursor),
            token.cursor
        );
    }

    // where
    let token = scope.next().unwrap_or(TokenInfo::default_with_cursor(token.cursor));

    if token.category != Token::Identifier || token.value != "where" {
        scope.panic(
            format!("Identifier \"where\" expected, found \"{}\" at {}", token.value, token.cursor),
            token.cursor
        )
    }

    // [Pattern]
    let token = scope.next().unwrap_or(TokenInfo::default_with_cursor(token.cursor));

    if token.category != Token::String {
        scope.panic(
            format!("Expected string, found \"{}\" at {}", token.value, token.cursor),
            token.cursor
        );
    }

    // Validate pattern
    let pattern = token.value.clone();

    match Regex::new(&pattern) {
        Err(err) => {
            scope.panic(
                format!("{}", err), token.cursor
            );
        }
        Ok(_) => {},
    }

    // use
    let token = scope.next().unwrap_or(TokenInfo::default_with_cursor(token.cursor));

    if token.category != Token::Identifier || token.value != "use" {
        scope.panic(
            format!("Identifier \"use\" expected, found \"{}\" at {}", token.value, token.cursor),
            token.cursor
        );
    }

    // as
    let token = scope.next().unwrap_or(TokenInfo::default_with_cursor(token.cursor));

    if token.category != Token::Identifier || token.value != "as" {
        scope.panic(
            format!("Identifier \"as\" expected, found \"{}\" at {}", token.value, token.cursor),
            token.cursor
        );
    }

    // [Module] | mod | pub mod
    let token = scope.next().unwrap_or(TokenInfo::default_with_cursor(token.cursor));

    match token.category {
        Token::String => {
            if token.category != Token::String {
                scope.panic(
                    format!("Expected string, found \"{}\" at {}", token.category, token.cursor),
                    token.cursor
                )
            }
        },
        Token::Identifier => {
            if token.category != Token::Identifier {
                scope.panic(
                    format!("Expected identifier, found \"{}\" at {}", token.category, token.cursor),
                    token.cursor
                )
            }

            if token.value == "pub" {
                // mod
                let token = scope.next().unwrap_or(TokenInfo::default_with_cursor(token.cursor));

                if token.category != Token::Identifier {
                    scope.panic(
                        format!("Identifier \"mod\" expected, found \"{}\" at {}", token.value, token.cursor),
                        token.cursor
                    )
                }

                if token.value != "mod" {
                    scope.panic(
                        format!("Identifier \"mod\" expected, found \"{}\" at {}", token.value, token.cursor),
                        token.cursor
                    )
                }
            } else if token.value != "mod" {
                scope.panic(
                    format!("Identifier \"mod\" expected, found \"{}\" at {}", token.value, token.cursor),
                    token.cursor
                )
            }
        },
        _ => {
            scope.panic(
                format!("Expected string or identifier, found \"{}\" at {}", token.category, token.cursor),
                token.cursor
            )
        }
    }
}

// - - -

#[proc_macro]
pub fn import(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = input.to_string();

    let mut scope = LexerScope::new(&input);
    let tokens = lexer(&mut scope);

    let mut scope = ParserScope::new(input, tokens.clone());

    parser(&mut scope);

    println!("{:?}", tokens);

    "".parse().unwrap()
}
