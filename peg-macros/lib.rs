extern crate quote;
extern crate proc_macro2;
extern crate proc_macro;

use quote::quote_spanned;

// This can't use the `peg` crate as it would be a circular dependency, but the generated code in grammar.rs
// requires `::peg` paths.
extern crate peg_runtime as peg;

mod ast;
mod tokens;
mod grammar;
mod translate;
mod analysis;

#[proc_macro]
pub fn parser(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tokens = tokens::FlatTokenStream::new(input.into());
    let grammar = match grammar::peg::peg_grammar(&tokens) {
        Ok(g) => g,
        Err(err) => {
            let msg = format!("expected {}", err.expected);
            return quote_spanned!(err.location.0=> compile_error!(#msg);).into();
        }
    };

    translate::compile_grammar(&grammar).into()
}
