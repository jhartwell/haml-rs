extern crate proc_macro;
use crate::proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};
use quote::quote;

#[proc_macro_attribute]
pub fn to(attr: TokenStream, item: TokenStream) -> TokenStream {
    eprintln!("attr: {}", attr.to_string());
    item
}