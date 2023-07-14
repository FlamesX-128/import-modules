use modules::{lexical, syntactic};

mod models;
mod modules;

extern crate proc_macro;

#[proc_macro]
pub fn import(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let document = input.to_string();
    let chars = document.chars();

    let mut scope = lexical::Scope::new(chars);
    let identifiers = lexical::lexer(&mut scope);

    let mut scope = syntactic::Scope::new(&document, identifiers);
    let result = syntactic::parser(&mut scope);

    println!("{:?}", result);

    "".parse().unwrap()
}
