extern crate proc_macro;

use proc_macro::TokenStream;

use syn::{parse_macro_input, Data, DeriveInput, Fields};

use quote::{format_ident, quote};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // eprintln!("attrs: {:#?}", input.attrs);
    // eprintln!("vis: {:#?}", input.vis);
    // eprintln!("ident: {:#?}", input.ident);
    // eprintln!("generics: {:#?}", input.generics);
    // eprintln!("data: {:#?}", input.data);

    let ident = input.ident;

    let builder_ident = format_ident!("{}Builder", ident);

    let fields = if let Data::Struct(data) = input.data {
        if let Fields::Named(fields) = data.fields {
            fields
        } else {
            panic!("this macro can only be used on a struct with named fields")
        }
    } else {
        panic!("this macro can only be used on a struct");
    };

    let builder_fields = fields.named.iter().map(|field| {
        let ident = field.ident.clone();
        let ty = field.ty.clone();
        quote! {
            #ident: Option<#ty>
        }
    });

    let ctor_fields = fields.named.iter().map(|field| {
        let ident = field.ident.clone();
        quote! {
            #ident: None
        }
    });

    let setters = fields.named.iter().map(|field| {
        let ident = field.ident.clone();
        let ty = field.ty.clone();
        quote! {
            fn #ident(&mut self, #ident: #ty) -> &mut Self {
                self.#ident = Some(#ident);
                self
            }
        }
    });

    let build_checks = fields.named.iter().map(|field| {
        let ident = field.ident.clone();
        let ident_name = ident.clone().unwrap().to_string();
        quote! {
            #ident: self.#ident.take().ok_or(format!("field {} has not been explicitly set", #ident_name))?
        }
    });

    let tokens = quote! {
        impl #ident {
            pub fn builder() -> #builder_ident {
                #builder_ident {
                    #(#ctor_fields),*
                }
            }
        }

        pub struct #builder_ident {
            #(#builder_fields),*
        }

        use std::error::Error;
        impl #builder_ident {
            #(#setters)*

            pub fn build(&mut self) -> Result<#ident, Box<dyn Error>> {
                Ok(#ident{
                    #(#build_checks),*
                })
            }
        }
    };

    // eprintln!("TOKENS: {}", tokens);
    tokens.into()
}
