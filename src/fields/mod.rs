use crate::item::enum_item::EnumInnerFields;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, LitBool, LitStr, Result, Token, parse::Parse, parse::ParseStream};

pub mod enums;
pub mod structure;

// Constants
const ERROR_PARSE_FN: &str = "Expect a function call";

#[derive(Clone)]
pub enum BrickeFieldArgs {
    ConvertFieldFn(LitStr),
    Rename(LitStr),
    Exclude(LitBool),
    IsFallible(LitBool),
}

impl Parse for BrickeFieldArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let keyword: Ident = input.parse()?;
        let _eq_token: Token![=] = input.parse()?;

        match keyword {
            k if k == "transform_fn" => Ok(BrickeFieldArgs::ConvertFieldFn(input.parse()?)),
            k if k == "rename" => Ok(BrickeFieldArgs::Rename(input.parse()?)),
            k if k == "exclude" => Ok(BrickeFieldArgs::Exclude(input.parse()?)),
            k if k == "is_fallible" => Ok(BrickeFieldArgs::IsFallible(input.parse()?)),
            _ => Err(syn::Error::new(keyword.span(), "Attribute not supported")),
        }
    }
}
