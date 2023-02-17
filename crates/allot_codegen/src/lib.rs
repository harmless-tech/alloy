use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use crate::structs::LibReturnMacroInput;

mod i;
mod structs;

#[proc_macro_derive(RawEnum)]
pub fn raw_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    i::raw_enum(input).into()
}

#[proc_macro]
pub fn lib_return(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LibReturnMacroInput);
    i::lib_return(input).into()
}
