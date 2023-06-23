use proc_macro_error::{abort, proc_macro_error, emit_warning};
use quote::IdentFragment;

mod models;
mod modules;

extern crate proc_macro;

#[proc_macro_error]
#[proc_macro]
pub fn import(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let span = input.to_string().span().unwrap();
    let str = input.to_string();

    if str.len() > 3 {
        emit_warning!(
            span, "import! macro only accepts one argument";

            help = "try removing the extra arguments";
            note = "";
            yay = "i see";
        )
    }

    "".parse().unwrap()
}
