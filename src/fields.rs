use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse::ParseStream, Ident, LitStr, Result, Token};

#[derive(Clone)]
pub enum BrickFieldArgs {
    ConvertFieldFn(LitStr),
    Rename(LitStr),
}

impl Parse for BrickFieldArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let keyword: Ident = input.parse()?;
        let _eq_token: Token![=] = input.parse()?;

        match keyword {
            k if k == "transform_func" => Ok(BrickFieldArgs::ConvertFieldFn(input.parse()?)),
            k if k == "rename" => Ok(BrickFieldArgs::Rename(input.parse()?)),
            _ => Err(syn::Error::new(keyword.span(), "Attribute not supported")),
        }
    }
}

impl BrickFieldArgs {
    pub(crate) fn create_template(name: Ident, fields: Vec<Self>) -> TokenStream {
        let mut from_field_name: Option<Ident> = Some(name.clone());
        let mut func: Option<Ident> = None;

        for field in fields {
            if let Self::Rename(n) = field.to_owned() {
                from_field_name = Some(Ident::new(&n.value(), Span::call_site()));
            }

            if let Self::ConvertFieldFn(f) = field {
                func = Some(Ident::new(&f.value(), Span::call_site()));
            }
        }

        match func {
            Some(f) => quote! {
                #name: #f(arg.#from_field_name)
            },
            None => quote! { #name: arg.#from_field_name },
        }
    }
}
