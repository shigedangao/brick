// Inspire from https://github.com/shrynx/struct_morph/tree/main
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    meta::ParseNestedMeta,
    parse::{Parse, ParseStream, Parser, Result},
    parse_macro_input,
    spanned::Spanned,
    token::Token,
    Attribute, Field, Ident, ItemStruct, LitStr, Meta, PathSegment, Token, Type,
    ItemFn
};

#[derive(Default)]
enum ConverterType {
    #[default]
    From,
    TryFrom,
}

#[derive(Default)]
struct BrikStructAttributes {
    converter: ConverterType,
    source_struct: Option<LitStr>,
}

impl BrikStructAttributes {
    fn parse(&mut self, meta: ParseNestedMeta) -> Result<()> {
        if meta.path.is_ident("converter") {
            let converter: LitStr = meta.value()?.parse()?;
            self.converter = match converter.value().as_str() {
                "From" => ConverterType::From,
                "TryFrom" => ConverterType::TryFrom,
                _ => ConverterType::From,
            };

            Ok(())
        } else if meta.path.is_ident("source_struct") {
            self.source_struct = Some(meta.value()?.parse()?);

            Ok(())
        } else {
            Err(syn::Error::new(meta.path.span(), "Unknown attribute"))
        }
    }
}

enum BrickFieldArgs {
    ConvertFieldFn(LitStr),
}

impl Parse for BrickFieldArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let keyword: Ident = input.parse()?;
        let _eq_token: Token![=] = input.parse()?;

        match keyword {
            k if k == "convert_field_func" => Ok(BrickFieldArgs::ConvertFieldFn(input.parse()?)),
            _ => Err(syn::Error::new(keyword.span(), "Unknown attribute")),
        }
    }
}

#[proc_macro_attribute]
pub fn brick_field(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(args as BrickFieldArgs);

    input
}

fn create_expanded(
    attr: BrikStructAttributes,
    target_name: Ident,
    de_fields: Vec<proc_macro2::TokenStream>,
) -> proc_macro2::TokenStream {
    let source = format_ident!("{}", attr.source_struct.unwrap().value());

    // [From, SourceStructName]
    let expanded = match attr.converter {
        ConverterType::From => quote! {
           impl From<#source> for #target_name {
               fn from(arg: #source) -> Self {
                   Self {
                    #(#de_fields),*
                   }
               }
           }
        },
        ConverterType::TryFrom => quote! {
           impl TryFrom<#source> for #target_name {
               type Error = Result<Self, Self::Error>;

               fn try_from(arg: #source) -> Result<Self, Self::Error> {
                   Ok(Self {
                    #(#de_fields),*
                   })
               }
           }
        }
    };

    proc_macro2::TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn brick(args: TokenStream, target: TokenStream) -> TokenStream {
    let mut attrs = BrikStructAttributes::default();
    let brick_parser = syn::meta::parser(|meta| attrs.parse(meta));
    parse_macro_input!(args with brick_parser);

    let input = parse_macro_input!(target as ItemStruct);

    let fields = input
        .fields
        .iter()
        .map(|item| {
            let field_name = &item.ident;
            let fields_args: Option<BrickFieldArgs> = item.attrs.iter().find_map(|attr| {
                attr.path()
                    .is_ident("brick_field")
                    .then(|| attr.parse_args().expect("Expect to parse brick_fields arg"))
            });

            match fields_args {
                Some(BrickFieldArgs::ConvertFieldFn(convert_field)) => {
                    let f = format_ident!("{}", convert_field.value());

                    proc_macro2::TokenStream::from(quote! {
                        #field_name: #f(arg.#field_name)
                    })
                }
                _ => proc_macro2::TokenStream::from(quote! { #field_name: arg.#field_name }),
            }
        })
        .collect::<Vec<_>>();

    let mut input_clone = input.clone();

    // Dunno why it's needed. Maybe to remove the attribute to avoid any output in the AST ?
    input_clone.fields.iter_mut().for_each(|field| {
        field
            .attrs
            .retain(|attr| !attr.path().is_ident("brick_field"));
    });

    let expanded = create_expanded(attrs, input.ident, fields);

    proc_macro2::TokenStream::from(quote! {
        #input_clone
        #expanded
    })
    .into()
}
