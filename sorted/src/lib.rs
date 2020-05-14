extern crate proc_macro;

mod imp;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, Item};

#[proc_macro_attribute]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(args as AttributeArgs);
    let ast = parse_macro_input!(input as Item);

    let compile_err_or_not = imp::sorted(&ast).unwrap_or_else(|err| err.to_compile_error());

    let tokens = quote! {
        #ast
        #compile_err_or_not
    };
    tokens.into()
}
