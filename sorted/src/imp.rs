use proc_macro2::{Span, TokenStream};
use syn::{Error, Item, Result};

pub fn sorted(input: &Item) -> Result<TokenStream> {
    let variants = match input.clone() {
        Item::Enum(e) => Ok(e.variants),
        _ => Err(Error::new(
            Span::call_site(),
            "expected enum or match expression",
        )),
    }?;

    let mut previous: Option<syn::Variant> = None;
    for (i, curr) in variants.iter().enumerate() {
        if let Some(ref prev) = previous.replace(curr.clone()) {
            if prev.ident.to_string() > curr.ident.to_string() {
                let mut least_upper_idx = 0;
                for j in i - 1..=0 {
                    if variants[j].ident.to_string() < curr.ident.to_string() {
                        least_upper_idx = j + 1;
                    }
                }
                return Err(Error::new(
                    curr.ident.span(),
                    format!(
                        "{} should sort before {}",
                        curr.ident, variants[least_upper_idx].ident
                    ),
                ));
            }
        };
    }
    Ok(TokenStream::new())
}
