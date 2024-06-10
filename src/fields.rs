use syn::{parse::Parse, parse::ParseStream, Ident, LitStr, Result, Token};

pub enum BrickFieldArgs {
    ConvertFieldFn(LitStr),
}

impl Parse for BrickFieldArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let keyword: Ident = input.parse()?;
        let _eq_token: Token![=] = input.parse()?;

        match keyword {
            k if k == "transform_func" => Ok(BrickFieldArgs::ConvertFieldFn(input.parse()?)),
            _ => Err(syn::Error::new(keyword.span(), "Attribute not supported")),
        }
    }
}
