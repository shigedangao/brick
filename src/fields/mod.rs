use crate::item::enum_item::EnumInnerFields;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, LitBool, LitStr, Result, Token, parse::Parse, parse::ParseStream};

pub mod enums;
pub mod structure;

#[derive(Clone)]
pub enum BrickFieldArgs {
    ConvertFieldFn(LitStr),
    FnFromExtern(LitStr),
    Rename(LitStr),
    Exclude(LitBool),
    IsFallible(LitBool),
}

impl Parse for BrickFieldArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let keyword: Ident = input.parse()?;
        let _eq_token: Token![=] = input.parse()?;

        match keyword {
            k if k == "transform_func" => Ok(BrickFieldArgs::ConvertFieldFn(input.parse()?)),
            k if k == "fn_from_extern" => Ok(BrickFieldArgs::FnFromExtern(input.parse()?)),
            k if k == "rename" => Ok(BrickFieldArgs::Rename(input.parse()?)),
            k if k == "exclude" => Ok(BrickFieldArgs::Exclude(input.parse()?)),
            k if k == "is_fallible" => Ok(BrickFieldArgs::IsFallible(input.parse()?)),
            _ => Err(syn::Error::new(keyword.span(), "Attribute not supported")),
        }
    }
}
