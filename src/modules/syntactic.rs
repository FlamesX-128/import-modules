use std::vec::IntoIter;

use crate::models::token::TokenInfo;

pub struct Scope <'a> {
    pub identifiers: IntoIter<TokenInfo>,
    pub document: &'a str,
    pub cursor: usize
}

impl <'a> Scope <'a> {
    pub fn next(&mut self) -> Option<TokenInfo> {
        self.cursor += 1;
        self.identifiers.next()
    }
}

impl <'a> Scope <'a> {
    pub fn new(document: &'a str, identifiers: Vec<TokenInfo>) -> Self {
        Self { identifiers: identifiers.into_iter(), document, cursor: 0 }
    }
}

// - - -

pub enum Node {
    As,
    Module,
    Pub,
    Use,
    Where,
    
}

// - - -

pub fn parser(scope: &mut Scope) {}
