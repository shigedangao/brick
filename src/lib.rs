use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item};

pub(crate) mod attributes;
pub(crate) mod fields;
pub(crate) mod item;

use attributes::BrickAttributes;
use fields::BrickFieldArgs;
use item::ProcessItem;

#[proc_macro_attribute]
pub fn brick_field(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(args as BrickFieldArgs);

    input
}

#[proc_macro_attribute]
pub fn brick(args: TokenStream, target: TokenStream) -> TokenStream {
    let mut attrs = BrickAttributes::default();
    let brick_parser = syn::meta::parser(|meta| attrs.parse(meta));
    parse_macro_input!(args with brick_parser);

    let input_kind = parse_macro_input!(target as Item);

    let expanded = match input_kind {
        Item::Struct(mut item) => item.process_item(attrs),
        Item::Enum(mut item) => item.process_item(attrs),
        _ => panic!("Type is not supported"),
    };

    proc_macro2::TokenStream::from(quote! {
        #expanded
    })
    .into()
}
