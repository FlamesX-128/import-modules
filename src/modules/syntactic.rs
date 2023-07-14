use std::{vec::IntoIter, fmt::Error, path::Path};

use fancy_regex::Regex;

use crate::models::{token::TokenInfo, ast::AST};

pub struct Scope <'a> {
    pub identifiers: Vec<TokenInfo>,
    pub document: &'a str,
    pub cursor: usize
}


impl <'a> Scope <'a> {
    pub fn new(document: &'a str, identifiers: Vec<TokenInfo>) -> Self {
        Self { identifiers, document, cursor: 0 }
    }
}

// - - - 

fn is_valid_directory(path: &str) -> Result<(), &str> {
    let message = format!("{} is not a valid directory", path);
    
    match Path::new(&path).exists() {
        false => Err(&message),
        true => Ok(())
    }
}

fn is_valid_regex(regex: &str) -> Result<(), &str> {
    match Regex::new(regex) {
        Err(err) => Err(&err.to_string()),
        Ok(_) => Ok(())
    }
}

fn fase_two(typ: usize, mut data: IntoIter<TokenInfo>) -> Result<(), &str> {
    match typ {
        1..=3 => {
            is_valid_directory(&data.nth(0).unwrap().content.unwrap())?;
            is_valid_regex(&data.nth(2).unwrap().content.unwrap())?;
        }
        4..=6 => {
            is_valid_directory(&data.nth(0).unwrap().content.unwrap())?;
        }
        _ => {}
    }

    Ok(())
}

// - - -

pub fn parser(scope: &mut Scope) -> Result<usize, &str> {
    let accepts: &[&[AST]] = &[
        &[AST::Str, AST::Where, AST::Str, AST::Use, AST::As, AST::Pub, AST::Mod],
        &[AST::Str, AST::Where, AST::Str, AST::Use, AST::As, AST::Mod],
        &[AST::Str, AST::Where, AST::Str, AST::Use, AST::As, AST::Str],
        &[AST::Str, AST::Use, AST::As, AST::Pub, AST::Mod],
        &[AST::Str, AST::Use, AST::As, AST::Mod],
        &[AST::Str, AST::Use, AST::As, AST::Str],
    ];

    let identifiers = scope.identifiers.clone();
    let identifiers = identifiers.into_iter();

    let mut data: Vec<AST> = Vec::new();

    identifiers.for_each(|identifier| {
        data.push(AST::from(identifier));
    });

    let data = data.as_slice();

    let result = accepts.iter().enumerate().find(|(_, accept)| {        
        if accept.len() != data.len() {
            return false;
        }

        accept.iter().zip(data.iter()).all(|(accept, data)| {         
            accept == data
        })

    });

    match result {
        Some((index, _)) => {
            fase_two(index, identifiers)?;
            Ok(index)
        }
        None => Err("Invalid syntax")
    }
}
