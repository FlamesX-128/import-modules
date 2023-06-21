use std::str::Chars;

extern crate proc_macro;

// - - -

#[derive(Clone, Debug, PartialEq)]
enum Token {
    Identifier,
    String,
    Unknown,
    None
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
    pub fn prev(&mut self) -> Option<char> {
        self.cursor -= 1;
        self.chars.next_back()
    }

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
    pub tokens: Vec<TokenInfo>,
    pub cursor: usize
}

impl ParserScope {
    pub fn new(tokens: Vec<TokenInfo>) -> Self {
        Self { tokens, cursor: 0 }
    }
}

impl ParserScope {
    pub fn prev(&mut self) -> Option<TokenInfo> {
        self.cursor -= 1;
        self.tokens.get(self.cursor).cloned()
    }

    pub fn next(&mut self) -> Option<TokenInfo> {
        self.cursor += 1;
        self.tokens.get(self.cursor).cloned()
    }
}

fn parser(scope: &mut ParserScope) {
    if scope.next().unwrap_or(TokenInfo::default()).category != Token::String {
        panic!("The first token must be a string literal");
    }

    let t_1 = scope.next().unwrap_or(TokenInfo::default());

    if t_1.category != Token::Identifier {
        panic!("The second token must be an identifier");
    }

    let t_2 = scope.next().unwrap_or(TokenInfo::default());

    
}

// - - -

#[proc_macro]
pub fn import(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = input.to_string();

    let mut scope = LexerScope::new(&input);
    let tokens = lexer(&mut scope);    

    println!("{:?}", tokens);

    "".parse().unwrap()
}
