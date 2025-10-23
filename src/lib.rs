use proc_macro::TokenStream;
use quote::quote;
use syn::{Item, parse_macro_input};

pub(crate) mod attributes;
pub(crate) mod fields;
pub(crate) mod item;

use attributes::BrickAttributes;
use item::ProcessItem;

/// Brick proc macro is a macro which generates a struct or enum with the specified attributes.
/// This allows to convert a struct to another struct which may contains similar fields while also
/// enabled to convert a field to another type or rename that field.
///
/// # Examples
/// struct Foo {
///     a: i32,
///     b: String,
/// }
///
/// #[brick(operator="From", source="Foo")]
/// struct Bar {
///     a: i32,
///     #[brick_field(rename="b")]
///     c: String,
/// }
#[proc_macro_attribute]
pub fn brick(args: TokenStream, target: TokenStream) -> TokenStream {
    let input_kind = parse_macro_input!(target as Item);

    let mut attrs = BrickAttributes::default();
    let brick_parser = syn::meta::parser(|meta| attrs.parse(meta));
    parse_macro_input!(args with brick_parser);

    let expanded = match input_kind {
        Item::Struct(mut item) => item.process(attrs, item::SupportedType::Struct),
        Item::Enum(mut item) => item.process(attrs, item::SupportedType::Enum),
        _ => unimplemented!("Type is not supported"),
    };

    quote! {
        #expanded
    }
    .into()
}
