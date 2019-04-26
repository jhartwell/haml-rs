extern crate proc_macro;

use proc_macro::TokenStream;
use syn;
use quote::quote;
use traits::AsAny;

#[proc_macro_derive(AsAny)]
pub fn any_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_as_any_macro(&ast)
}

fn impl_as_any_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl AsAny for #name {
            fn to_any(&self) -> Self {
                self
            }
        }
    }
}